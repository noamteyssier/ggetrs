# UCSC

This module is used to interact with the UCSC genome browser.
Currently there is only the BLAT API which is implemented.

## BLAT

Perform a [BLAT](https://genome.ucsc.edu/cgi-bin/hgBlat) search using the UCSC Genome Browser.

### BLAT Help

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

### BLAT Usage

```bash
# query UCSC genome browser for the first 121 bp of AP2S1
ggetrs ucsc blat GGGCCCTACAACTGCACCCTGAGCCGGAGCTGCCCAGTCGCCGCGGGACCGGGGCCGCTGGGGTCTGGACGGGGGTCGCCATGGTAACGGGGGAGCGCTACGCCGGGGACTGGCGGAGGG
```
