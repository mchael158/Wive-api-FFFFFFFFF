# Grand Line Adventures Wine Launcher

Um launcher escrito em Rust para executar o instalador do jogo "Grand Line Adventures" no Linux usando Wine.

## Menu de Navegação
- **Editina**  
  - Download  
  - Grand Line Adventures  

## Recursos
- Detecção automática da distribuição Linux
- Instalação automática do Wine
- Execução do instalador do jogo
- Suporte para múltiplas distribuições Linux
- Configuração de WINEPREFIX personalizado

## Pré-requisitos
- Sistema Linux
- Acesso sudo para instalação de pacotes (se o Wine não estiver instalado)
- Tokio (para execução assíncrona)

## Instalação

1. Instale o Rust (caso ainda não tenha):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

git clone https://github.com/mchael158/Wive-api-FFFFFFFFF.git
cd Wive-api-FFFFFFFFF

cargo build --release
```

## Como Usar
Coloque o instalador do jogo (gla_installer.exe) na pasta ~/Downloads ou tenha o caminho completo do arquivo .exe disponível.

Execute o launcher:

```bash
./target/release/grandline-wine-launcher
```

## Siga as instruções na tela:

O programa verificará automaticamente se o Wine está instalado

Pode ser necessário instalar o Wine (será solicitada permissão sudo)

Você pode especificar um caminho diferente para o instalador se não estiver em ~/Downloads

É possível definir um WINEPREFIX personalizado (padrão: ~/.wine-grandline)

## Distribuições Suportadas
Ubuntu/Debian/Linux Mint (apt)

Arch Linux/Manjaro (pacman)

Fedora (dnf)

openSUSE (zypper)

## Para outras distribuições, será necessário instalar o Wine manualmente.
