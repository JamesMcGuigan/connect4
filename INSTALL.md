# Install
BUG: miniconda missing kaggle_environments==1.13.0 - https://github.com/Kaggle/kaggle-environments/issues/239

Project Creation 
```
pip install cffi maturin 
maturin new connectx  # cffi bindings are more compatable
```

Virtualenv
```
conda deactivate               # maturin failed | unset one of: VIRTUAL_ENV, CONDA_PREFIX
/usr/bin/python3 -m venv venv  # use system python3 not miniconda
source ./venv/bin/activate
pip-compile -v; pip install -r requirements.txt
```
```
./submission.sh  # uses virtualenv
```

Poetry
```
poetry shell      # == source .../activate
poetry install    # reads pyproject.toml
conda deactivate  # WORKAROUND: maturin failed | unset one of: VIRTUAL_ENV, CONDA_PREFIX
```

Buildchain

- Automated (using virtualenv) via [./submission.sh](./submission.sh)  

```
conda deactivate          # WORKAROUND: maturin failed | unset one of: VIRTUAL_ENV, CONDA_PREFIX
cargo check               # WORKS 
cargo build               # WORKS

maturin develop           # poetry run maturin develop
maturin build --release   # poetry run maturin build --release
docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin build --release # manylinux build

cp -vf ./python/main.py                       ./submission/  # Kaggle Entrypoint
cp -vf ./python/*.py                          ./submission/
cp -vf target/release/maturin/libconnectx.so  ./submission/connectx.so
kaggle competitions submit -c connectx -f     ./submission/submission.tar.gz -m ''
```

Test Python
```
python3 ./python/main.py
python3 ./python/test.py
```