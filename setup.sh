if [ "${BASH_SOURCE[0]}" -ef "$0" ]
then
    echo "ERROR: This script should be sourced, not executed!"
    echo "Please, run: source setup.sh"
    exit 1
fi

set -e
cd -- "$(dirname -- "$0")"
pwd

if [ -d "$HOME/.cargo/bin" ]; then
    echo 'rustup is already installed'
else
    echo 'Installing rustup'
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

if rustup --version; then
    echo 'rustup is already in the path'
else
    echo "./cargo/bin is not in the path, adding it for you"
    export PATH="$HOME/.cargo/bin:$PATH"
fi

echo 'Checking for rustup updates'
rustup self update
rustc --version
cargo --version

rustup component add rustfmt
rustup component add clippy

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  echo 'Installing linux dependencies'
  sudo apt update
  sudo apt upgrade -y
  sudo apt install build-essential -y
  source ~/.bashrc
fi

echo "Rust configured!"
