#!/usr/bin/env bash
# DOCS: https://www.kaggle.com/c/halite/discussion/177686
cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"  # cd current directory
set -x

rm   -rf ./submission/
mkdir -p ./submission/
cp ./python/main.py ./submission/
cp ./python/*.py    ./submission/
cp target/release/maturin/libmaturin_kaggle.so  ./submission/maturin_kaggle.so
cp /usr/lib/libc.so                             ./submission/libc.so

cd ./submission/
tar cvfz submission.tar.gz *