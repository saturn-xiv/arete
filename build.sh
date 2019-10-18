#!/bin/sh

set -e

if [ $# -ne 1 ] ; then
    echo 'Please specify your libc type: glibc musl'
    exit 1
fi 

export WORKSPACE=$PWD

cd $WORKSPACE/dashboard
if [ ! -d "node_modules" ]
then 
    npm install
fi
npm run build

cd $WORKSPACE

if [ $1 = 'glibc' ];
then 
    cargo build --release
else if [ $1 = 'musl' ]
    RUSTFLAGS="-C target-feature=-crt-static" cargo build --release
else
    echo "bad libc type $1"
    exit 2
fi

strip -s target/release/arete

exit 0