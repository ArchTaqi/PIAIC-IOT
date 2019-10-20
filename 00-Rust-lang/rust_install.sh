
sudo apt-get update -y
sudo apt-get upgrade -y
sudo apt-get install -y curl
sudo apt install -y gcc
curl  https://sh.rustup.rs  -sSf  |  sh
source $HOME/.cargo/env

#cargo new Hello --bin
#cargo run
#cargo build
cargo new Hello --bin
cargo check