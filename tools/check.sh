#!/bin/sh

set -e

echo "Check default features..."
cargo check
cargo clippy
echo "Build default features..."
cargo build
echo "Done."

declare -a features=(
    "mysql" 
    "sqlite"    
)

for i in "${features[@]}"
do
    echo "Check features: $i..."
    cargo check --no-default-features --features "$i"
    cargo clippy --no-default-features --features "$i"
    echo "Build features: $i..."
    cargo build --no-default-features --features "$i"
    echo "Done."
done

exit 0