# Search

## Help

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

## Usage

```bash
# searches Ensembl for all genes with `clathrin` in the description
ggetrs search clathrin

# searches Ensembl for all genes with `clathrin` OR `heavy` in the description
ggetrs search clathrin heavy
```
