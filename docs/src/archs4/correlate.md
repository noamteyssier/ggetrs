# Correlate

## Help

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

## Usage

```bash
# Perform a gene-correlation analysis with ARCHS4
ggetrs archs4 correlate -c 10 AP2S1
```
