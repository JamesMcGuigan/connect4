# Python Rust Module with maturin
- DOCS: https://github.com/PyO3/maturin
- EXAMPLE: https://github.com/deepgreenAN/pyo3_error_handling

Install
```
pip install cffi maturin 
maturin new maturin_kaggle  # cffi bindings are more compatable
python3 -m venv venv 
```

Maturin LifeCycle
```
cargo clean
maturin build   --release  # build into ./target/wheels/
maturin develop --release  # build + install in local venv
maturin publish --release  # publish to pypi 
```

Buildchain
```
cargo check               # WORKS 
cargo build               # ERROR: linker failure
poetry install            # reads pyproject.toml
poetry run maturin develop
poetry run maturin build --release  && ./submission.sh
```

Usage
```
$ source  ./python/venv/bin/activate
$ python3 ./python/main.py

import maturin_kaggle
maturin_kaggle.double(42)     = 84
maturin_kaggle.double_f64(42) = 84.0
```

Kaggle Deployment
- https://www.kaggle.com/competitions/connectx/submissions
```
poetry run maturin build --release  && ./submission.sh
kaggle competitions submit -c connectx -f submission/submission.tar.gz -m ''
```

Manylinux Docker Build
- https://stackoverflow.com/questions/62838212/whats-the-correct-way-to-compile-maturin-packages
- https://github.com/PyO3/maturin
```
sudo pacman -S docker
systemctl enable docker
systemctl start docker

sudo rm -rf ./target/
sudo docker run --rm -v $(pwd):/io konstin2/maturin build --release
sudo chmod a+rw -R ./target/
sudo bash ./submission.sh

kaggle competitions submit -c connectx -f submission/submission.tar.gz -m ''
```