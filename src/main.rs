use std::path::Path;
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;
use std::io::{self, Write};
use tokio::fs;
use std::collections::HashMap;

async fn ask_user_for_path(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn find_installer_exe(home_dir: &str) -> Option<String> {
    let search_path = format!("{}/Downloads/gla_installer.exe", home_dir);
    let path = Path::new(&search_path);
    if path.exists() {
        Some(path.to_string_lossy().to_string())
    } else {
        None
    }
}

async fn detect_distro() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let content = fs::read_to_string("/etc/os-release").await?;
    let mut os_info = HashMap::new();

    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            os_info.insert(key.trim().to_string(), value.trim_matches('"').to_string());
        }
    }

    Ok(os_info.get("ID").cloned().unwrap_or_default())
}

/// Verifica se o Wine está instalado
async fn check_wine_installed() -> bool {
    Command::new("which")
        .arg("wine")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false)
}


// lê o conteúdo do arquivo /etc/os-release para detectar a distribuição Linux
async fn try_install_wine() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let distro = detect_distro().await?;

    let install_cmd = match distro.as_str() {
        "ubuntu" | "debian" | "linuxmint" => vec!["apt", "install", "wine", "-y"],
        "arch" | "manjaro" => vec!["pacman", "-S", "--noconfirm", "wine"],
        "fedora" => vec!["dnf", "install", "-y", "wine"],
        "opensuse" => vec!["zypper", "install", "-y", "wine"],
        other => {
            return Err(format!("Distribuição '{}' não suportada automaticamente. Instale o wine manualmente.", other).into());
        }
    };

    println!("Instalando wine usando: sudo {}", install_cmd.join(" "));

    let status = Command::new("sudo")
        .args(&install_cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        Err("Falha ao instalar o Wine automaticamente.".into())
    } else {
        Ok(())
    }
}

async fn start_game(
    wine_prefix: &str,
    game_exe: &str,
) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
    let prefix_path = Path::new(wine_prefix);
    if !prefix_path.exists() {
        tokio::fs::create_dir_all(prefix_path).await?;
    }

    let exe_path = Path::new(game_exe);
    if !exe_path.exists() {
        return Err(format!("Executável não encontrado: {}", game_exe).into());
    }

    let mut cmd = Command::new("wine");
    cmd.arg(game_exe)
        .current_dir(exe_path.parent().ok_or("Falha ao obter diretório do executável")?)
        .env("WINEPREFIX", wine_prefix)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let mut reader = BufReader::new(stdout).lines();
        tokio::spawn(async move {
            while let Ok(Some(line)) = reader.next_line().await {
                println!("[wine] {}", line);
            }
        });
    }

    if let Some(stderr) = child.stderr.take() {
        let mut reader = BufReader::new(stderr).lines();
        tokio::spawn(async move {
            while let Ok(Some(line)) = reader.next_line().await {
                eprintln!("[wine-err] {}", line);
            }
        });
    }

    let status = child.wait().await?;
    Ok(status.code().unwrap_or(-1))
}

#[tokio::main]
async fn main() {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let default_prefix = format!("{}/.wine-grandline", home_dir);

    if !check_wine_installed().await {
        if let Err(e) = try_install_wine().await {
            eprintln!("Erro ao tentar instalar o Wine: {}", e);
            return;
        }
    }

    let mut game_exe = match find_installer_exe(&home_dir) {
        Some(path) => path,
        None => {
            println!("Não foi possível localizar o arquivo 'gla_installer.exe' em ~/Downloads.");
            ask_user_for_path("Digite o caminho completo do instalador: ").await
        }
    };

    if !Path::new(&game_exe).exists() {
        println!("Arquivo não encontrado: {}", game_exe);
        game_exe = ask_user_for_path("Digite novamente o caminho válido para o .exe: ").await;
    }

    println!("Usando WINEPREFIX padrão: {}", default_prefix);
    let custom_prefix = ask_user_for_path("Pressione Enter para usar o padrão ou digite outro WINEPREFIX: ").await;
    let wine_prefix = if custom_prefix.trim().is_empty() {
        default_prefix
    } else {
        custom_prefix
    };

    println!("Iniciando o instalador...");
    match start_game(&wine_prefix, &game_exe).await {
        Ok(code) => println!("Processo finalizou com código de saída: {}", code),
        Err(err) => eprintln!("Erro ao iniciar o instalador: {}", err),
    }
}
