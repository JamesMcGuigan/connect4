# Maturin Rust + Kaggle AI ConnectX
- DOCS: https://github.com/PyO3/maturin
- EXAMPLE: https://github.com/deepgreenAN/pyo3_error_handling
- KAGGLE:  https://www.kaggle.com/competitions/connectx/

Install
```
pip install cffi maturin 
maturin new maturin_kaggle  # cffi bindings are more compatable
python3 -m venv venv
pip-compile -v
pip install -r requirements.txt
```

Buildchain
```
$ source  ./python/venv/bin/activate
$ python3 ./python/main.py
$ python3 ./python/test.py

cargo check               # WORKS 
cargo build               # ERROR: linker failure
poetry install            # reads pyproject.toml
poetry run maturin develop
poetry run maturin build --release
docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin build --release # manylinux build
./submission.sh
kaggle competitions submit -c connectx -f submission/submission.tar.gz -m ''
```