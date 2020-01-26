#!/bin/sh

set -e 

echo "Upgrade rust & cargo..."
rustup update stable
cargo update
cargo install --force --git https://github.com/kbknapp/cargo-outdated

echo "Upgrade node & npm..."
nvm install node
npm install -g npm

exit 0