#!/bin/sh

set -e

if [ ! -d "node_modules" ]
then
	yarn set version berry
	yarn set version latest
	echo "nodeLinker: node-modules" >> .yarnrc.yml
fi

yarn install

cd dashboard
yarn install
cd -

echo "Done."

exit 0
