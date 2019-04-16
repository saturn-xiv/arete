#!/bin/sh

set -e

cargo check
cargo check --no-default-features --features "theme-bootstrap mysql"
cargo check --no-default-features --features "theme-bootstrap sqlite"

exit 0