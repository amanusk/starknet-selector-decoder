#!/bin/bash

awk '!seen[$0]++' ./wordlist.txt > ./temp
cat ./temp > ./wordlist.txt
rm ./temp
