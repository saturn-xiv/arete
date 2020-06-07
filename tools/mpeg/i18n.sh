#!/bin/sh

declare -a languages=(
    "en_US"
    "zh_CN"
)

for i in "${languages[@]}"
do 
    pylupdate5 gmpg.py -ts locales/$i.ts
    lrelease locales/$i.ts
done