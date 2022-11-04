# [ `ggetrs` ]

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/noamteyssier/ggetrs/blob/main/LICENSE)
[![actions status](https://github.com/noamteyssier/ggetrs/workflows/CI/badge.svg)](https://github.com/noamteyssier/ggetrs/actions)
[![codecov](https://codecov.io/gh/noamteyssier/ggetrs/branch/main/graph/badge.svg?token=CEQWH6MMCV)](https://codecov.io/gh/noamteyssier/ggetrs)
[![Crates.io](https://img.shields.io/crates/d/ggetrs?color=orange&label=crates.io)](https://crates.io/crates/ggetrs)
[![docs.rs](https://img.shields.io/docsrs/ggetrs?color=green&label=docs.rs)](https://docs.rs/ggetrs/latest/ggetrs/)

## Introduction

`ggetrs` is a free, open-source command-line tool that enables efficient querying
of genomic databases.
It consists of a collection of separate but interoperable modules, each designed
to facilitate one type of database querying in a single line of code.

This is a rust reimplentation of the original python-based program
[`gget`](https://github.com/pachterlab/gget) and was rewritten to take advantage
of rust's powerful HTTP and asynchronous functionality for a faster user experience.

There are some minor syntactic changes between function calls from the original `gget`
and a description for each tool is provided on the [modules page](https://noamteyssier.github.io/ggetrs/modules.html).

This tool is written fully in rust - but allows for a python interface using [pyo3](https://github.com/PyO3/pyo3).

If you have questions please check out the [FAQ](./faq.md)

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
