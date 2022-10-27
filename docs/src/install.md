# Installation

## Installing via Cargo

This can be installed easily through the rust package manager `cargo`.
If you have never used rust before it is easily installed with a single command [here](https://rustup.rs/).

```bash
# install ggetrs from crates.io
cargo install ggetrs
```

## Installing via Github

```bash
git clone https://github.com/noamteyssier/ggetrs
cd ggetrs
cargo install --path .
```

## Installing the Python Module

If you are also interested in using the python interface for `ggetrs` you will first need to install [`maturin`](https://github.com/PyO3/maturin) and then install `ggetrs`.

```bash
# clone the repo
git clone https://github.com/noamteyssier/ggetrs
cd ggetrs

# install maturin
pip install maturin

# install ggetrs to your current environment
maturin develop
```

### No conda / venv environment

Currently `maturin develop` requires a conda or venv environment to be active before installing a python module, but you can install it manually by first building the wheel then manually pip installing the wheel.

```bash
# clone the repo
git clone https://github.com/noamteyssier/ggetrs
cd ggetrs

# install maturin
pip install maturin

# build the python wheel
maturin build

# install the python wheel manually
pip install target/wheels/*.whl
```
