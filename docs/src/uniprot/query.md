# Query

## Help

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

## Usage

```bash
# Query uniprot for AP2S1
ggetrs uniprot query AP2S1

# Query uniprot for AP2S1, RFX3, NSD1
ggetrs uniprot query AP2S1 RFX3 NSD1
```
