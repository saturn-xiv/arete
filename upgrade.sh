#!/bin/sh

set -e 

echo "Upgrade rust & cargo..."
rustup update stable
rustup component add rustfmt clippy rls rust-analysis rust-src
cargo update
cargo install --force --git https://github.com/kbknapp/cargo-outdated

echo "Upgrade node & npm..."
nvm install node
npm install -g npm

exit 0