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

## Summary

`ggetrs` currently consists of the following modules:

- [`ggetrs enrichr`](#enrichr)

  Perform an enrichment analysis on a list of genes using [Enrichr](https://maayanlab.cloud/Enrichr)

- [`ggetrs archs4`](#archs4)

  Find the most correlated genes to a gene of interest or find the gene's tissue
  expression atlas using [ARCHS4](https://maayanlab.cloud/archs4)

- [`ggetrs blast`](#blast)

  BLAST a nucleotide or amino acid sequence to any
  [BLAST](https://blast.ncbi.nlm.nih.gov/Blast.cgi) database

- [`ggetrs chembl`](#chembl)

  Perform a bioactivity search for any protein of interest using [Chembl](https://ebi.ac.uk/chembl)

- [`ggetrs search`](#search)

  Fetch genes and transcripts from [Ensembl](https://ensembl.org)
  using free-form search terms.

- [`ggetrs info`](#info)

  Fetch extensive gene and transcript metadata from [Ensembl](https://ensembl.org),
  [Uniprot](https://uniprot.org), and [NCBI](https://ncbi.nlm.nih.gov).

- [`ggetrs seq`](#seq)

  Fetch nucleotide or amino acid sequences of genes or transcripts from
  [Ensembl](https://ensembl.org) or [Uniprot](https://uniprot.org) respectively.

- [`ggetrs ensembl`](#ensembl)

  Perform [Ensembl](https://ensembl.org) related queries from their public
  [API](https://rest.ensembl.org).

- [`ggetrs uniprot`](#uniprot)

  Query [Uniprot](https://uniprot.org) directly for gene/protein information.

- [`ggetrs ncbi`](#ncbi)

  Query [NCBI](https://ncbi.nlm.nih.gov) directly for gene/protein information.

- [`ggetrs pdb`](#pdb)

  Get structure and metadata of a protein from the [RCSB Protein Data Bank](https://rcsb.org)

- [`ggetrs ucsc`](#ucsc)

  Perform a [BLAT](https://genome.ucsc.edu/cgi-bin/hgBlat) search using the
  UCSC Genome Browser.

- [`ggetrs autocomplete`](#autocomplete)

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

### Using the built-in help menus

Every tool has an extensive documentation built into the commandline.
This tool is still under production and some command usages will be variable
until the release version reaches (1.x.x).
Until that point, unless you are consistently updating, your version may be different
than the current release.
As such it is highly recommended that you use each commands builtin help menu.

This goes not only for subcommands but for `ggetrs` itself! 

```bash
# Help menu for ggetrs
ggetrs --help

# Help menu for a subcommand (like ensembl)
ggetrs ensembl --help

# Help menu for a subsubcommand (like ensembl lookup-symbols)
ggetrs ensembl lookup-symbols --help
```

### Enrichr

#### Enrichr Help

Perform an enrichment analysis on a list of genes using [Enrichr](https://maayanlab.cloud/Enrichr).

This requires at minimum a database name (listed [here](https://maayanlab.cloud/Enrichr/#libraries))
and any number of gene symbols to perform enrichment analysis on.

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

#### Enrichr Usage

```bash
# Perform an enrichment analysis using Enrichr
ggetrs enrichr -l GO_Biological_Process_2021 AP2S1 NSD1 RFX3
```

### ARCHS4

Queries gene-specific information from the [ARCHS4](https://maayanlab.cloud/archs4)
database.

This tool has two submodules - `correlate` and `tissue`.

#### ARCHS4 Correlate

##### ARCHS4 Correlate Help

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

##### ARCHS4 Correlate Usage

```bash
# Perform a gene-correlation analysis with ARCHS4
ggetrs archs4 correlate -c 10 AP2S1
```

#### ARCHS4 Tissue

##### ARCHS4 Tissue Help

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

##### ARCHS4 Tissue Usage

```bash
# Find tissue-level expression using ARCHS4
ggetrs archs4 tissue AP2S1
```

### BLAST

#### BLAST Help

The BLAST program can be determined from the provided input (will assign either `blastn` or `blastp`) and the appropriate database will be used: `nt` and `nr` respectively. You may override these though by using their arguments

```text
Performs a BLAST query for a given sequence

Usage: ggetrs blast [OPTIONS] <QUERY>

Arguments:
  <QUERY>  query sequence to BLAST

Options:
  -p, --program <PROGRAM>    blast program to use [possible values: blastn, blastp, blastx, tblastn, tblastx]
  -d, --database <DATABASE>  blast database to use [possible values: nt, nr, refseq-rna, refseq-protein, swissprot, pdbaa, pdbnt]
  -l, --limit <LIMIT>        Number of hits to return [default: 50]
  -e, --expect <EXPECT>      Minimum expected value to consider [default: 10.0]
  -f, --low-comp-filter      Whether to use a complexity filter (default = false)
  -m, --megablast            Whether to use MEGABLAST (default = true)
  -o, --output <OUTPUT>      optional filepath to write output to [default=stdout]
  -h, --help                 Print help information
  -V, --version              Print version information
```

#### BLAST Usage

```bash
# Perform BLAST with a nucleotide sequence
ggetrs blast ATACTCAGTCACACAAGCCATAGCAGGAAACAGCGAGCTTGCAGCCTCACCGACGAGTCTCAACTAAAAGGGACTCCCGGAGCTAGGGGTGGGGACTCGGCCTCACACAGTGAGTGCCGG

# Perform BLAST with an amino acid sequence
ggetrs blast MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE

# Perform BLAST with an amino acid sequence using the PDBAA database
ggetrs blast -d pdbaa MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE
```

### Chembl

This module is used to query the [Chembl](https://ebi.ac.uk/chembl) database.

Currently the query-APIs available are:

- Activity

  Queries for chemical bioactivity for a provided protein-target.

#### Chembl Activity

##### Chembl Activity Help

```text
Queries chemical activity for a provided item

Usage: ggetrs chembl activity [OPTIONS] <QUERY>

Arguments:
  <QUERY>  Query to retrieve bioactivity

Options:
  -l, --limit <LIMIT>    Number of results to return [default: 500]
  -o, --output <OUTPUT>  Optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

##### Chembl Activity Usage

```bash
# Query the Chembl database for small molecules with bioactivity targeting NSD1
ggetrs chembl activity NSD1
```

### Search

#### Search Help

```text
Searches through descriptions on ENSEMBL

Usage: ggetrs search [OPTIONS] <SEARCH_TERMS>...

Arguments:
  <SEARCH_TERMS>...  Search terms to query

Options:
  -d, --database <DATABASE>  database
  -s, --species <SPECIES>    species used in database [default: homo_sapiens]
  -t, --db-type <DB_TYPE>    database type specied by Ensembl [default: core]
  -r, --release <RELEASE>    release number to use for database [default: 108]
  -a, --assembly <ASSEMBLY>  assembly to use for species [default: 38]
  -o, --output <OUTPUT>      optional filepath to write output to [default=stdout]
  -h, --help                 Print help information
  -V, --version              Print version information
```

#### Search Usage

```bash
# searches Ensembl for all genes with `clathrin` in the description
ggetrs search clathrin

# searches Ensembl for all genes with `clathrin` OR `heavy` in the description
ggetrs search clathrin heavy
```

### Info

Fetch extensive gene and transcript metadata from [Ensembl](https://ensembl.org), [Uniprot](https://uniprot.org), and [NCBI](https://ncbi.nlm.nih.gov).

#### Info Help

```text
Queries symbols or Ensembl IDs across multiple databases and aggregates results

Usage: ggetrs info [OPTIONS] <SEARCH_TERMS>...

Arguments:
  <SEARCH_TERMS>...  Search terms to query

Options:
  -s, --species <SPECIES>    Taxon ID to use: currently this MUST match the taxon_id [default: homo_sapiens]
  -t, --taxon-id <TAXON_ID>  Taxon ID to use: currently this MUST match the species [default: 9606]
  -o, --output <OUTPUT>      optional filepath to write output to [default=stdout]
  -h, --help                 Print help information
  -V, --version              Print version information
```

#### Info Usage

```bash
# Queries information for AP2S1
ggetrs info AP2S1

# Queries information for multiple terms: AP2S1, RFX3, NSD1
ggetrs info AP2S1 RFX3 NSD1
```

### Seq

Returns nucleotide or amino acid sequence for a provided ensembl ID or gene symbol.

If gene symbols are provided instead of ensembl IDs for nucleotide sequences
those symbols will first be matched to an ensembl ID with the same functionality
of [`ggetrs ensembl lookup-symbol`](#ensembl-lookup-symbol).

All returned sequences are guaranteed to be in the same order as provided ids/symbols.

#### Seq Help

```text
Queries sequences from ensembl and UniProt

Usage: ggetrs seq [OPTIONS] <SEARCH_TERMS>...

Arguments:
  <SEARCH_TERMS>...  Search terms to query (can be Ensembl IDs or Gene Symbols)

Options:
  -t, --transcribe         Return the amino acid sequence instead of nucleotide sequence
  -s, --species <SPECIES>  Species/alias to specify [default: homo_sapiens]
  -o, --output <OUTPUT>    optional filepath to write output to [default=stdout]
  -h, --help               Print help information
  -V, --version            Print version information
```

#### Seq Usage

```bash
# recover nucleotide sequence for AP2S1 (ENSG00000042753)
ggetrs seq ENSG00000042753

# recover nucleotide sequence for AP2S1
ggetrs seq AP2S1

# recover nucleotide sequence for AP2S1 (ENSG00000042753) and NSD1
ggetrs seq ENSG00000042753 NSD1

# recover amino acid sequence for AP2S1 (ENSG00000042753)
ggetrs seq -t ENSG00000042753

# recover amino acid sequence for AP2S1
ggetrs seq -t AP2S1

# recover amino acid sequences for AP2S1 and NSD1 and RFX3
ggetrs seq -t AP2S1 NSD1 RFX3
```

### Ensembl

This is a collection of modules to query the ensembl database.

Currently there are the following APIs covered:

```text
search         Searches through descriptions on ENSEMBL
database       Prints all available databases on Ensembl's SQL database
lookup-id      Lookup information for genes/transcripts providing ensembl ids
lookup-symbol  Lookup information for genes/transcripts providing symbols and species
release        Retrieves the latest ensembl release version
ref            Retrieves reference files from Ensembl FTP site
species        Retrieves the list of species from ENSEMBL FTP site
```

#### Ensembl Search

This is another way to access [`ggetrs search`](#search).

#### Ensembl Database

Prints all available databases on Ensembl's SQL server.

This is used if you are interested in querying a specific database and can be passed into [`ggetrs search`](#search).

##### Ensembl Database Help

```text
Prints all available databases on Ensembl's SQL database

Usage: ggetrs ensembl database [OPTIONS]

Options:
  -f, --filter <FILTER>  Provides a substring filter to only return databases which contain the substring
  -o, --output <OUTPUT>  optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

##### Ensembl Database Usage

```bash
# show all databases in the SQL server
ggetrs ensembl database

# filter for databases with the `sapiens` substring
ggetrs ensembl database -f sapiens

# filter for databases with the `cerevisiae` substring
ggetrs ensembl database -f cerevisiae
```

#### Ensembl Lookup-Id

##### Ensembl Lookup-Id Help

```text
Lookup information for genes/transcripts providing ensembl ids

Usage: ggetrs ensembl lookup-id [OPTIONS] <ENSEMBL_IDS>...

Arguments:
  <ENSEMBL_IDS>...  Ensembl IDS to query

Options:
  -o, --output <OUTPUT>  optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

##### Ensembl Lookup-Id Usage

```bash
# Query information for AP2S1 (ENSG00000042753)
ggetrs ensembl lookup-id ENSG00000042753

# Query Information for AP2S1 (ENSG00000042753) and NSD1 (ENSG00000165671)
ggetrs ensembl lookup-id ENSG00000042753 ENSG00000165671
```

#### Ensembl Lookup-Symbol

##### Ensembl Lookup-Symbol Help

```text
Lookup information for genes/transcripts providing symbols and species

Usage: ggetrs ensembl lookup-symbol [OPTIONS] <SYMBOLS>...

Arguments:
  <SYMBOLS>...  Gene symbols to query

Options:
  -s, --species <SPECIES>  Species/alias to specify [default: homo_sapiens]
  -o, --output <OUTPUT>    optional filepath to write output to [default=stdout]
  -h, --help               Print help information
  -V, --version            Print version information
```

##### Ensembl Lookup-Symbol Usage

```bash
# Query information for AP2S1
ggetrs ensembl lookup-symbol AP2S1

# Query Information for AP2S1 and NSD1
ggetrs ensembl lookup-symbol AP2S1 NSD1

# Query information for AP2S1 and NSD1 in mice
ggetrs ensembl lookup-symbol -s mus_musculus AP2S1 NSD1
```

#### Ensembl Release

Returns the latest Ensembl release version

```bash
ggetrs ensembl release
```

#### Ensembl Ref

Retrieves reference files from the Ensembl FTP site.

##### Ensembl Ref Help

```text
Retrieves reference files from Ensembl FTP site

Usage: ggetrs ensembl ref [OPTIONS] --datatype <DATATYPE>

Options:
  -s, --species <SPECIES>    Species to query data for [default: homo_sapiens]
  -r, --release <RELEASE>    Release to use - will default to latest release [default: 108]
  -d, --datatype <DATATYPE>  Datatype to query for, provided as a comma-separated list (example: cdna,dna,gtf) [possible values: cdna, cds, dna, gff3, gtf, ncrna, pep]
  -o, --output <OUTPUT>      Optional filepath to write output to [default=stdout]
  -h, --help                 Print help information
  -V, --version              Print version information
```

##### Ensembl Ref Usage

```bash
# returns the url for the human cdna transcriptome
ggetrs ensembl ref -d cdna

# returns the url for the human cdna transcriptome and genome
ggetrs ensembl ref -d cdna,dna

# returns the url for the mouse cdna transcriptome and genome
ggetrs ensembl ref -d cdna,dna -s mus_musculus
```

#### Ensembl Species

Returns the available species in the Ensembl FTP site

##### Ensembl Species Help

```text
Retrieves the list of species from ENSEMBL FTP site

Usage: ggetrs ensembl species [OPTIONS]

Options:
  -r, --release <RELEASE>    Release to use - will default to latest release [default: 108]
  -o, --output <OUTPUT>      Optional filepath to write output to [default=stdout]
  -d, --datatype <DATATYPE>  Datatype to query species list [default: dna] [possible values: cdna, cds, dna, gff3, gtf, ncrna, pep]
  -h, --help                 Print help information
  -V, --version              Print version information
```

##### Ensembl Species Usage

```bash
# return all species where there is a genome (i.e. dna)
ggetrs ensembl species

# return all species where there is a transcriptome (i.e. cdna)
ggetrs ensembl species -d cdna


# return all species where there is a transcriptome (i.e. cdna)
# for an older release
ggetrs ensembl species -d cdna -r 60
```

### Uniprot

This a module for direct querying on the [Uniprot](https://uniprot.org) database.
Currently there is a single module: `query` but more modules are expected to be created in the future and so this command was created as a submodule.

This provides nearly all information as `ggetrs info` but is significantly faster.

#### Uniprot Query Help

```text
Searches through descriptions on Uniprot

Usage: ggetrs uniprot query [OPTIONS] <SEARCH_TERMS>...

Arguments:
  <SEARCH_TERMS>...  Search terms to query

Options:
  -t, --taxon <TAXON>    Taxon to filter results (human: 9606, mouse: 10090)
  -o, --output <OUTPUT>  optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

#### Uniprot Query Usage

```bash
# Query uniprot for AP2S1
ggetrs uniprot query AP2S1

# Query uniprot for AP2S1, RFX3, NSD1
ggetrs uniprot query AP2S1 RFX3 NSD1
```

### NCBI

This module allows for direct access to APIs provided by NCBI.
Currently the following submodules are provided:

```text
taxons         Retrieves taxon information from NCBI from a query string
query-ids      Retrieves information for a list of IDs
query-symbols  Retrieves information for a list of symbols (must provide taxon)
```

#### NCBI Taxons

This retrieves possible taxons from an incomplete query string. 

##### NCBI Taxons Help

```text
Retrieves taxon information from NCBI from a query string

Usage: ggetrs ncbi taxons [OPTIONS] <QUERY>

Arguments:
  <QUERY>  taxon name to query

Options:
  -l, --limit <LIMIT>    number of search results to return [default: 5]
  -o, --output <OUTPUT>  optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

##### NCBI Taxons Usage

```bash

# return all taxons that contain the substring 'sapiens'
ggetrs ncbi taxons sapiens

# return the first 3 taxons that contain the substring 'sapi'
ggetrs ncbi taxons -l 3 sapi
```

#### NCBI Query-IDs

##### NCBI Query-IDs Help

```text
Retrieves information for a list of IDs

Usage: ggetrs ncbi query-ids [OPTIONS] <IDS>...

Arguments:
  <IDS>...  NCBI ids to query

Options:
  -o, --output <OUTPUT>  optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

##### NCBI Query-IDs Usage

```bash
# query NCBI for AP2S1 (ncbi ID: 1175)
ggetrs ncbi query-ids 1175

# query NCBI for AP2S1 and NSD1 (1175 and 64324 respectively)
ggetrs ncbi query-ids 1175 64324
```

#### NCBI Query-Symbols

Query NCBI for gene symbols and with a provided taxon ID.
You can determine taxon IDs for your organism of choice with [`ggetrs ncbi taxons`](#ncbi-taxons).

##### NCBI Query-Symbols Help

```text
Retrieves information for a list of symbols (must provide taxon)

Usage: ggetrs ncbi query-symbols [OPTIONS] <SYMBOLS>...

Arguments:
  <SYMBOLS>...  NCBI ids to query

Options:
  -t, --taxon-id <TAXON_ID>  Taxon ID (human: 9606, mouse: 10090) [default: 9606]
  -o, --output <OUTPUT>      optional filepath to write output to [default=stdout]
  -h, --help                 Print help information
  -V, --version              Print version information
```

##### NCBI Query-Symbols Usage

```bash
# query NCBI for the symbol AP2S1
ggetrs ncbi query-symbols AP2S1

# query NCBI for the symbol AP2S1 in mice
ggetrs ncbi query-symbols -t 10090 AP2S1
```

### PDB

Get structure and metadata of a protein from the [RCSB Protein Data Bank](https://rcsb.org)

There are currently two submodules in PDB: `structure` and `info`.

#### PDB Structure Help

```text
Retrieves pdb structure for a provided ID

Usage: ggetrs pdb structure [OPTIONS] <PDB_ID>

Arguments:
  <PDB_ID>  PDB id to retrieve structure

Options:
  -m, --header-only      Retrieve only the PDB header
  -f, --format <FORMAT>  Specify the structure format [default: pdb] [possible values: pdb, cif]
  -o, --output <OUTPUT>  Optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

#### PDB Structure Usage

```bash
# return the pdb structure for AP2S1 (6URI)
ggetrs pdb info 6URI
```

#### PDB Info Help

```text
Retrieves pdb information for a provided ID and resource

Usage: ggetrs pdb info [OPTIONS] <PDB_ID>

Arguments:
  <PDB_ID>  PDB Id to request info

Options:
  -r, --resource <RESOURCE>      Specify the structure format [default: entry] [possible values: entry, pubmed, assembly, branched-entity, nonpolymer-entity, polymer-entity, uniprot, branched-entity-instance, polymer-entity-instance, nonpolymer-entity-instance]
  -i, --identifier <IDENTIFIER>  Specifies the Entry or Chain Identifier
  -o, --output <OUTPUT>          Optional filepath to write output to [default=stdout]
  -h, --help                     Print help information
  -V, --version                  Print version information
```

#### PDB Info Usage

```bash
# return information for AP2S1 (6URI)
ggetrs pdb info 6URI
```

### UCSC

This module is used to interact with the UCSC genome browser.
Currently there is only the BLAT API which is implemented.

#### UCSC BLAT

Perform a [BLAT](https://genome.ucsc.edu/cgi-bin/hgBlat) search using the UCSC Genome Browser.

##### UCSC BLAT Help

```text
Performs a BLAT sequence search on a provided database

Usage: ggetrs ucsc blat [OPTIONS] <SEQUENCE>

Arguments:
  <SEQUENCE>  PDB Id to request info

Options:
  -s, --seqtype <SEQTYPE>  Specify the structure format [default: dna] [possible values: dna, protein, translated-rna, translated-dna]
  -d, --db-name <DB_NAME>  Specifies the database name to query [default: hg38]
  -o, --output <OUTPUT>    Optional filepath to write output to [default=stdout]
  -h, --help               Print help information
  -V, --version            Print version information
```

##### UCSC BLAT Usage

```bash
# query UCSC genome browser for the first 121 bp of AP2S1
ggetrs ucsc blat GGGCCCTACAACTGCACCCTGAGCCGGAGCTGCCCAGTCGCCGCGGGACCGGGGCCGCTGGGGTCTGGACGGGGGTCGCCATGGTAACGGGGGAGCGCTACGCCGGGGACTGGCGGAGGG
```

### Autocomplete

This is used to generate autocomplete information for your terminal shell.

#### Autocomplete Help

```text
Set up autocomplete for various shells

Usage: ggetrs autocomplete --shell <SHELL>

Options:
  -s, --shell <SHELL>  Shell to generate autocompletions for [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help           Print help information
  -V, --version        Print version information
```

#### Autocomplete Usage

```bash
# generate autocompletions for the fish shell
ggetrs autocomplete -s fish

# write autocomplete directly to fish shell config
ggetrs autocomplete -s fish > ~/.config/fish/completions/ggetrs.fish
```
