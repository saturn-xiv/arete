#!/bin/sh

set -e

RUSTFLAGS="-C target-feature=-crt-static"
PKG_CONFIG_ALLOW_CROSS=1
export RUSTFLAGS PKG_CONFIG_ALLOW_CROSS

cargo build --release --target x86_64-unknown-linux-musl 

exit 0
