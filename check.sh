#!/bin/sh

set -e

declare -a features=(
    "theme-bootstrap redis sodium mysql" 
    "theme-bootstrap redis sodium sqlite"    
)

echo "check default features"
cargo check
cargo build

for i in "${features[@]}"
do
    echo "check features: $i..."
    cargo check --no-default-features --features "$i"
    cargo check --no-default-features --features "$i"
done

exit 0