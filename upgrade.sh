#!/bin/sh

set -e

cargo update
cargo check
cargo build

yarn set version latest
yarn up
cd dashboard && yarn up

exit 0