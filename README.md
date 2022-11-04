# ggetrs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/ggetrs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/noamteyssier/ggetrs/branch/main/graph/badge.svg?token=CEQWH6MMCV)](https://codecov.io/gh/noamteyssier/ggetrs)
[![Crates.io](https://img.shields.io/crates/d/ggetrs?color=orange&label=crates.io)](https://crates.io/crates/ggetrs)
[![docs.rs](https://img.shields.io/docsrs/ggetrs?color=green&label=docs.rs)](https://docs.rs/ggetrs/latest/ggetrs/)

## Introduction

### What is `ggetrs`?

`ggetrs` is a free, open-source command-line tool that enables efficient querying
of genomic databases.
It consists of a collection of separate but interoperable modules, each designed
to facilitate one type of database querying in a single line of code.

This is a rust reimplentation of the original python-based program
[`gget`](https://github.com/pachterlab/gget) and was rewritten to take advantage
of rust's powerful HTTP and asynchronous functionality for a faster user experience.

There are some minor syntactic changes between function calls from the original `gget`
and a description for each tool is provided on the [modules page](https://noamteyssier.github.io/ggetrs/modules.html).

## Installation

The command line tool is distributed via [crates.io](https://crates.io/crates/ggetrs/)
and can be installed via the rust package manager cargo.

```bash
cargo install ggetrs
```

Alternative installation instructions as well as the python module installation
can be found on the [install page](https://noamteyssier.github.io/ggetrs/install.html).

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

## FAQ

### What makes this different than [`gget`](https://github.com/pachterlab/gget)?

`ggetrs` takes advantage of rust's powerful powerful asynchronous features
and lets you perform a large numbers of HTTP requests without increasing wait times.

Since it is a compiled program as well there is no start-up time between commands
and you can run your favorite tool in a `for loop` with no overhead.

However `ggetrs` stays true to the original `gget` mindset and tries to make usage
as simple as possible no matter the interface (from python to CLI).

### Does this have functions that [`gget`](https://github.com/pachterlab/gget) doesn't?

We're working on having both tools mirror functionality - but currently this includes the
[Chembl](https://noamteyssier.github.io/ggetrs/chembl/activity.html) bioactivity database,
more endpoints from the [Ensembl API](https://noamteyssier.github.io/ggetrs/ensembl.html),
and direct queries to [NCBI](https://noamteyssier.github.io/ggetrs/ncbi.html)
and [Uniprot](https://noamteyssier.github.io/ggetrs/uniprot/query.html).

### Does [`gget`](https://github.com/pachterlab/gget) have functions that `ggetrs` does not?

`ggetrs` will likely not support the `ggetrs muscle` and `ggetrs alphafold`
functionalities for the time being.
The reasoning being that these are wrappers around existing binaries
and not HTTP requests.

### Do I need to know rust to use this tool?

This tool is written fully in rust - but allows for a python interface using
[pyo3](https://github.com/PyO3/pyo3).
Currently not all tools have a python API - but they are planned to be
implemented eventually.

All of the currently supported `gget` modules have their python API implemented.
