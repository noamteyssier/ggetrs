# Info

Fetch extensive gene and transcript metadata from [Ensembl](https://ensembl.org),
[Uniprot](https://uniprot.org), and [NCBI](https://ncbi.nlm.nih.gov).

## Info Help

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

## Info Usage

```bash
# Queries information for AP2S1
ggetrs info AP2S1

# Queries information for multiple terms: AP2S1, RFX3, NSD1
ggetrs info AP2S1 RFX3 NSD1
```
