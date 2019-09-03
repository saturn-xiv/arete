#!/bin/sh

set -e

export WORKSPACE=$PWD

cd $WORKSPACE/dashboard
if [ ! -d "node_modules" ]
then 
    npm install
fi
npm run build

cd $WORKSPACE
cargo build --release
strip -s target/release/arete

exit 0