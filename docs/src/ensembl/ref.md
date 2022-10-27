# Ref

Retrieves reference files from the Ensembl FTP site.

## Help

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

## Usage

```bash
# returns the url for the human cdna transcriptome
ggetrs ensembl ref -d cdna

# returns the url for the human cdna transcriptome and genome
ggetrs ensembl ref -d cdna,dna

# returns the url for the mouse cdna transcriptome and genome
ggetrs ensembl ref -d cdna,dna -s mus_musculus
```
