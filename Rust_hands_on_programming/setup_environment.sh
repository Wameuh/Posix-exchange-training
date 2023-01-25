#!/usr/bin/sh
set -x
#ask for sudo et elevate privilege
[ "$(whoami)" != "root" ] && exec sudo -- "$0" "$@"

#Force time update
sudo timedatectl set-ntp off
sudo timedatectl set-ntp on


#Try to find Curl
if ! [ -x "$(command -v curl)" ]; then
    sudo apt install curl
fi

#Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#Configure current shell
source "$HOME/.cargo/env"

cargo +stable install cargo-llvm-cov

set +x