# FAQ

## What makes this different than `gget`?

`ggetrs` takes advantage of rust's powerful powerful asynchronous features
and lets you perform a large numbers of HTTP requests without increasing wait times.

Since it is a compiled program as well there is no start-up time between commands
and you can run your favorite tool in a `for loop` with no overhead.

However `ggetrs` stays true to the original `gget` mindset and tries to make usage
as simple as possible no matter the interface (from python to CLI).

## Does this have functions that `gget` doesn't?

We're working on having both tools mirror functionality - but currently this includes the
[Chembl](./chembl.md) bioactivity database,
more endpoints from the [Ensembl API](./ensembl.md),
and direct queries to [NCBI](./ncbi.md) and [Uniprot](./uniprot.md).

## Does `gget` have functions that `ggetrs` does not?

`ggetrs` will likely not support the `ggetrs muscle` and `ggetrs alphafold`
functionalities for the time being.
The reasoning being that these are wrappers around existing binaries
and not HTTP requests.

## Do I need to know rust to use this tool?

This tool is written fully in rust - but allows for a python interface using
[pyo3](https://github.com/PyO3/pyo3).
Currently not all tools have a python API - but they are planned to be
implemented eventually.

All of the currently supported `gget` modules have their python API implemented.
