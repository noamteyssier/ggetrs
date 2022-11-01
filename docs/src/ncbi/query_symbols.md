# Query Symbols

Query NCBI for gene symbols and with a provided taxon ID.
You can determine taxon IDs for your organism of choice with
[`ggetrs ncbi taxons`](./taxons.md).

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Taxon ID | `-t` | `--taxon-id` | Taxon ID (human: 9606, mouse: 10090) [default: 9606] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# query NCBI for the symbol AP2S1
ggetrs ncbi query-symbols AP2S1

# query NCBI for the symbol AP2S1 in mice
ggetrs ncbi query-symbols -t 10090 AP2S1
```
