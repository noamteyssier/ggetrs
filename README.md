# ggetrs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/ggetrs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/noamteyssier/ggetrs/branch/main/graph/badge.svg?token=CEQWH6MMCV)](https://codecov.io/gh/noamteyssier/ggetrs)

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

Installation instructions can be found [here](https://noamteyssier.github.io/ggetrs/install.html).

## Documentation

All [`documentation`](https://noamteyssier.github.io/ggetrs/) and usecases can be
found at the website.

## Contributing

This project is intended to be open-source and contributions are very welcome!

All new additions must pass and follow current testing standards.
