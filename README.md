# Maturin Rust + Kaggle AI ConnectX

This codebase demonstrates how to implement a Kaggle AI agent, 
as a Rust module using Maturin, passing in full configuration,
and accessible to a python script.  

- [./src/lib.rs](./src/lib.rs) 
  - Implementations for Observation + Configuration structs 
  - random_move_column() | pass in single argument
  - modulo_move_args()   | pass in all arguments as primitives
  - modulo_move_struct() | map arguments to Rust structs Observation + Configuration

- [./python/main.rs](./python/main.rs)
  - Main Agent Submission File

- [./python/test.rs](./python/test.rs)
  - Localhost Play Loop for Testing

- [pyproject.toml](pyproject.toml)
  - `bindings="pyo3"` is required for `maturin develop` to work  
  - `bindings="cffi"` will work with Kaggle, but breaks `maturin develop`
  - Kaggle Runtime requires a manylinux2014 compile
    - `compatibility="manylinux2014"` is not manylinux_2_17 (aka manylinux2014) compliant due to libc.so.6

- [./submission.sh](./submission.sh)
  - Manylinux Compile Script via Docker - needed to run on Kaggle
  - `docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin build --release` is required for a manylinux build
 
---

## Links
- DOCS: https://github.com/PyO3/maturin
- EXAMPLE: https://github.com/deepgreenAN/pyo3_error_handling
- KAGGLE:  https://www.kaggle.com/competitions/connectx/

## Install
```
pip install cffi maturin 
maturin new maturin_kaggle  # cffi bindings are more compatable
python3 -m venv venv
pip-compile -v
pip install -r requirements.txt
```

## Buildchain
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