# Taxons

This retrieves possible taxons from an incomplete query string.

## Help

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

## Usage

```bash

# return all taxons that contain the substring 'sapiens'
ggetrs ncbi taxons sapiens

# return the first 3 taxons that contain the substring 'sapi'
ggetrs ncbi taxons -l 3 sapi
```
