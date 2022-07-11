#!/usr/bin/env bash
# DOCS: https://www.kaggle.com/c/halite/discussion/177686
cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"  # cd current directory
source venv/bin/activate
set -x

# rm -rf ./target/    || sudo rm   -rf ./target/
rm -rf ./submission/  || sudo rm   -rf ./submission/

### ManyLinux build for Kaggle AI Games submission
# sudo chmod 666 /var/run/docker.sock
time maturin develop  # requires bindings="pyo3"
time docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin build --release

rm   -rf ./submission/
mkdir -p ./submission/
cp -vf ./python/main.py ./submission/
cp -vf ./python/*.py    ./submission/
cp -vf target/release/maturin/libconnectx.so  ./submission/connectx.so
python3 ./submission/test.py

cd ./submission/
tar cvfz connectx.tar.gz *
cd ..
set +x
echo
echo "TODO Submit Manually: https://www.kaggle.com/competitions/connectx/submissions"
echo
echo "kaggle competitions submit -c connectx -f submission/connectx.tar.gz -m ''"
echo