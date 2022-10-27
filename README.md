# ggetrs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/ggetrs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/noamteyssier/ggetrs/branch/main/graph/badge.svg?token=CEQWH6MMCV)](https://codecov.io/gh/noamteyssier/ggetrs)

## Introduction

`ggetrs` is a free, open-source command-line tool that enables efficient querying
of genomic databases.
`ggetrs` consists of a collection of separate but interoperable modules, each designed
to facilitate one type of database querying in a single line of code.

This is a rust reimplentation of the original python-based program
[gget](https://github.com/pachterlab/gget) and was rewritten to be significantly
faster.
There are some minor semantic changes between function calls from the python version
but a description for each tool is provided below.

This tool is written fully in rust - but allows for a python interface using
[pyo3](https://github.com/PyO3/pyo3).

## Summary

`ggetrs` currently consists of the following modules:

- `ggetrs enrichr`
  Perform an enrichment analysis on a list of genes using [Enrichr](https://maayanlab.cloud/Enrichr)

- `ggetrs archs4`
  Find the most correlated genes to a gene of interest or find the gene's tissue
  expression atlas using [ARCHS4](https://maayanlab.cloud/archs4)

- `ggetrs blast`
  BLAST a nucleotide or amino acid sequence to any
  [BLAST](https://blast.ncbi.nlm.nih.gov/Blast.cgi) database

- `ggetrs chembl`
  Perform a bioactivity search for any protein of interest using [Chembl](https://ebi.ac.uk/chembl)

- `ggetrs search`
  Fetch genes and transcripts from [Ensembl](https://ensembl.org)
  using free-form search terms.

- `ggetrs info`
  Fetch extensive gene and transcript metadata from [Ensembl](https://ensembl.org),
  [Uniprot](https://uniprot.org), and [NCBI](https://ncbi.nlm.nih.gov).

- `ggetrs seq`
  Fetch nucleotide or amino acid sequences of genes or transcripts from
  [Ensembl](https://ensembl.org) or [Uniprot](https://uniprot.org) respectively.

- `ggetrs ensembl`
  Perform [Ensembl](https://ensembl.org) related queries from their public
  [API](https://rest.ensembl.org).

- `ggetrs uniprot`
  Query [Uniprot](https://uniprot.org) directly for gene/protein information.

- `ggetrs ncbi`
  Query [NCBI](https://ncbi.nlm.nih.gov) directly for gene/protein information.

- `ggetrs pdb`
  Get structure and metadata of a protein from the [RCSB Protein Data Bank](https://rcsb.org)

- `ggetrs ucsc`
  Perform a [BLAT](https://genome.ucsc.edu/cgi-bin/hgBlat) search using the
  UCSC Genome Browser.

- `ggetrs autocomplete`
  Generates an autocompletion file for your shell of choice for a better
  terminal experience.

## Installation

### Command-Line Only

```bash
git clone https://github.com/noamteyssier/ggetrs
cd ggetrs
cargo install --path .
```

### Python Module

Before this is fully packaged you will need to install this separately using `maturin`

```bash
pip install maturin
```

#### Conda or Venv

If you are in a conda environment or a virtual environment you can use the
`maturin develop` interface.

```bash
maturin develop
```

#### No environment

Without a python environment you will need to build the package and subsequently
install the wheel.

```bash
maturin build
pip install target/wheels/<your_config>.whl
```

## Modules

### Enrichr

Perform an enrichment analysis on a list of genes using [Enrichr](https://maayanlab.cloud/Enrichr).

This requires at minimum a database name (listed [here](https://maayanlab.cloud/Enrichr/#libraries))
and any number of gene symbols to perform enrichment analysis on.

#### Example

```bash
ggetrs enrichr -l GO_Biological_Process_2021 AP2S1 NSD1 RFX3
```

For a full list of arguments use the help flag!

```bash
ggetrs --help
```

```text
Perform an enrichment analysis on a list of genes using Enrichr

Usage: ggetrs enrichr [OPTIONS] --library <LIBRARY> <GENE_LIST>...

Arguments:
  <GENE_LIST>...  list of gene symbols to perform enrichment analysis on

Options:
  -l, --library <LIBRARY>  any database listed at: https://maayanlab.cloud/Enrichr/#libraries
  -o, --output <OUTPUT>    optional filepath to write output to [default=stdout]
  -h, --help               Print help information
  -V, --version            Print version information
```

### ARCHS4

Queries gene-specific information from the [ARCHS4](https://maayanlab.cloud/archs4)
database.

This tool has two submodules - `correlate` and `tissue`.

#### Correlate

```text
Performs a gene-correlation analysis

Usage: ggetrs archs4 correlate [OPTIONS] <GENE_NAME>

Arguments:
  <GENE_NAME>  Gene name to query for correlation

Options:
  -c, --count <COUNT>    number of values to recover [default: 100]
  -o, --output <OUTPUT>  output filepath to write to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

```bash
ggetrs archs4 correlate -c 10 AP2S1
```

#### Tissue

```text
Perform a tissue-enrichment analysis

Usage: ggetrs archs4 tissue [OPTIONS] <GENE_NAME>

Arguments:
  <GENE_NAME>  Gene name to query for tissue

Options:
  -s, --species <SPECIES>  number of values to recover [default: human] [possible values: human, mouse]
  -o, --output <OUTPUT>    output filepath to write to [default=stdout]
  -h, --help               Print help information
  -V, --version            Print version information
```

```bash
ggetrs archs4 tissue AP2S1
```

### BLAST

### Chembl

### Search

### Info

### Seq

### Ensembl

### Uniprot

### NCBI

### PDB

### UCSC

### Autocomplete

## Python Usage

#### Enrichr

##### Enrichr :: CLI

For querying `Enrichr`

```bash
ggetrs enrichr -l GO_Biological_Process_2021 AP2S1 NSD1 RFX3 LDB1
```

##### Enrichr :: Python

For querying `Enrichr`

```python3
import ggetrs
ggetrs.enrichr("GO_Biological_Process_2021", ["AP2S1", "NSD1", "RFX3", "LDB1"])
```

#### ARCHS4

The `ARCHS4` API currently has two submodules:

1. correlate
2. tissue

##### ARCHS4 :: CLI

```bash
# correlation module
ggetrs archs4 correlate AP2S1

# tissue module
ggetrs archs4 tissue AP2S1
```

##### ARCHS4 :: Python

```python3
import ggetrs

# correlation module
ggetrs.archs4.correlate("AP2S1", 10)

# tissue module
ggetrs.archs4.tissue("AP2S1", "human")
```

#### Search

This is part of the ensembl submodule but is accessible at the top level as well.

##### Search :: CLI

```bash
# free-form searching (single term)
ggetrs search AP2S1

# free-form searching (multiple terms)
ggetrs search AP2S1 GABA RFX3
```

##### Search :: Python

```python3
import ggetrs

# search module (single term)
ggetrs.search(["AP2S1"])

# search module (multiple terms)
ggetrs.search(["AP2S1", "GABA", "RFX3"])
```

#### Ensembl

This is a submodule for `Ensembl` related queries

This submodule currently has 2 commands:

1. search
2. database

`search` performs a free-form search of ensembl descriptions for a set of search
terms.

`database` returns all available databases which can be searched in Enesembl.
This is used to select which databases to search with the `search` command.

#### Ensembl :: CLI

```bash
# Free-form searching
ggetrs ensembl search AP2S1

# Database listing
ggetrs ensembl database 

# Database listing with filter
ggetrs ensembl database -f mus_musculus
```

### Ensembl :: Python

```python3
import ggetrs

# search module (single term)
ggetrs.ensembl.search(["AP2S1"])

# search module (multiple terms)
ggetrs.ensembl.search(["AP2S1", "GABA", "RFX3"])

# database module (no filter)
ggetrs.ensembl.database()

# database module (with filter)
ggetrs.ensembl.database("mus_musculus")
```
