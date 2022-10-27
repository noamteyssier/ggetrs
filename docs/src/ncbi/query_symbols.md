# Query Symbols

Query NCBI for gene symbols and with a provided taxon ID.
You can determine taxon IDs for your organism of choice with
[`ggetrs ncbi taxons`](./taxons.md).

## Help

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

## Usage

```bash
# query NCBI for the symbol AP2S1
ggetrs ncbi query-symbols AP2S1

# query NCBI for the symbol AP2S1 in mice
ggetrs ncbi query-symbols -t 10090 AP2S1
```
