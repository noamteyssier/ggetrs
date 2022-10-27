# Tissue

## Help

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

## Usage

```bash
# Find tissue-level expression using ARCHS4
ggetrs archs4 tissue AP2S1
```
