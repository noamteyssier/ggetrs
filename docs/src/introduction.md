# ggetrs

## Introduction

`ggetrs` is a free, open-source command-line tool that enables efficient querying
of genomic databases.
It consists of a collection of separate but interoperable modules, each designed
to facilitate one type of database querying in a single line of code.

This is a rust reimplentation of the original python-based program [gget](https://github.com/pachterlab/gget)
and was rewritten to be significantly faster.
There are some minor semantic changes between function calls from the python
version but a description for each tool is provided here.

This tool is written fully in rust - but allows for a python interface using [pyo3](https://github.com/PyO3/pyo3).

## Installation

`ggetrs` can be installed easily using `cargo`.

See alternative methods and details for [python installation](./install.md)

```bash
cargo install ggetrs
```

## Module Overview

Here is a [list of currently supported modules](./modules.md)

## External Links

- [Github Repo](https://github.com/noamteyssier/ggetrs)
- [crates.io](https://crates.io/crates/ggetrs)
- [docs.rs](https://docs.rs/ggetrs)
