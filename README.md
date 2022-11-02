# ggetrs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/ggetrs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/noamteyssier/ggetrs/branch/main/graph/badge.svg?token=CEQWH6MMCV)](https://codecov.io/gh/noamteyssier/ggetrs)
[![Crates.io](https://img.shields.io/crates/d/ggetrs?color=orange&label=crates.io)](https://crates.io/crates/ggetrs)
[![docs.rs](https://img.shields.io/docsrs/ggetrs?color=green&label=docs.rs)](https://docs.rs/ggetrs/latest/ggetrs/)

## Introduction

`ggetrs` is a free, open-source command-line tool that enables efficient querying
of genomic databases.
It consists of a collection of separate but interoperable modules, each designed
to facilitate one type of database querying in a single line of code.

This is a rust reimplentation of the original python-based program
[gget](https://github.com/pachterlab/gget) and was rewritten to be significantly
faster.
There are some minor semantic changes between function calls from the python version
but a description for each tool is provided below.

This tool is written fully in rust - but allows for a python interface using
[pyo3](https://github.com/PyO3/pyo3).

## Installation

The command line tool is distributed via [crates.io](https://crates.io/crates/ggetrs/)
and can be installed via the rust package manager cargo.

```bash
cargo install ggetrs
```

Alternative installation instructions as well as the python module installation
can be found on the homepage [here](https://noamteyssier.github.io/ggetrs/install.html).

## Documentation

### Command Line and Python

All usage and function documentation for the command line and python utilities
can be found on the [ggetrs homepage](https://noamteyssier.github.io/ggetrs/).

### Rust

`ggetrs` is implemented as a rust library as well as a standalone binary so all
documentation for API calls and Data Structures can be found on
[docs.rs](https://docs.rs/ggetrs).

## Contributing

This project is intended to be open-source and contributions are very welcome!

If you are new to rust or open source in general but still want to contribute
please don't hesitate to reach out!
I would be more than happy to help guide you through building your first module.

All new additions must pass and follow current testing standards.
