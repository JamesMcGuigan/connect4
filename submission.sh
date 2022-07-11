#!/usr/bin/env bash
# DOCS: https://www.kaggle.com/c/halite/discussion/177686
cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"  # cd current directory
set -x

# rm -rf ./target/    || sudo rm   -rf ./target/
rm -rf ./submission/  || sudo rm   -rf ./submission/

### ManyLinux build for Kaggle AI Games submission
# sudo chmod 666 /var/run/docker.sock
# time maturin develop   # This doesn't seem to work correctly
time docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin build --release

rm   -rf ./submission/
mkdir -p ./submission/
cp -vf ./python/main.py ./submission/
cp -vf ./python/*.py    ./submission/
cp -vf target/release/maturin/libmaturin_kaggle.so  ./python/maturin_kaggle.so
cp -vf target/release/maturin/libmaturin_kaggle.so  ./submission/maturin_kaggle.so
python3 ./submission/test.py

cd ./submission/
tar cvfz maturin_kaggle.tar.gz *
cd ..

echo -e "To Submit:\n\nkaggle competitions submit -c connectx -f submission/maturin_kaggle.tar.gz -m ''"