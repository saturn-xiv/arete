#!/bin/sh

set -e

if [ ! -d "node_modules" ]
then
	yarn set version latest
	echo "nodeLinker: node-modules" >> .yarnrc.yml
	yarn install
fi

echo "Done."

exit 0
