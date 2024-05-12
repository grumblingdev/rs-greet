#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
CARGO=$HOME/.cargo/

if [ ! -d "$CARGO" ]
then
    echo "Rust is not installed, installing via rustup..."
    if [ -e "/usr/bin/curl" ]
    then
        curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
        source "$CARGO/env"
        echo "Don't worry! The script handled setting the source for you!"
        cd $SCRIPT_DIR || return
        cargo build --release
        if [ -d "$HOME/.local/bin" ]
        then
            echo "Moving the rs-greet binary to $HOME/.local/bin
            adding the execution line in $HOME/.bashrc"
            mv target/release/rs-greet $HOME/.local/bin
            echo "rs-greet" >> $HOME/.bashrc
        else
            echo "$HOME/.local/bin does not exist,
            you need to create it and set it manually."
        fi
    else
        echo "Cannot install Rust because curl is missing from your system,
         please install it with your distribution's package manager!"
    fi
fi