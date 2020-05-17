#!/bin/sh

set -e

if [ $# -ne 1 ] ; then
    echo 'Please specify your database type: postgresql, mysql, sqlite'
    exit 1
fi

VERSION=$(`git describe --tags --always --dirty`)

echo "Build $VERSION ..."
if [ -d "tmp/$VERSION" ]
then
	echo "Target $VERSION already exists!"
	exit 2
fi


echo 'Install assets....'
if [ ! -d "node_modules" ]
then
	yarn set version berry
	yarn set version latest
	echo "nodeLinker: node-modules" >> .yarnrc.yml
	yarn install
fi

echo 'Build frontend....'
cd dashboard
if [ ! -d "node_modules" ]
then
	yarn install
fi
yarn build
cd -

echo 'Build backend....'
cargo build --release --no-default-features --features "$1"
strip -s target/release/arete

echo "Generate package $VERSION"
mkdir -p tmp/$VERSION
cp -r assets LICENSE  README.md node_modules target/release/arete tmp/$VERSION/
cp -r dashboard/dist tmp/$VERSION/dashboard
cd tmp
tar -cf - $VERSION/ | xz -9 -c - > arete-${VERSION}.tar.xz
cd -

echo "$VERSION Done."

exit 0
