#!/bin/bash
cd ~/Projects/my_rust_bar || exit
pkill my_rust_bar
sleep 0.1

export PATH="$HOME/.cargo/bin:$PATH"

cargo build --release 
nohup ./target/release/my_rust_bar > /dev/null 2>&1 &
