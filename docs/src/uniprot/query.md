# Query

Searches through descriptions on Uniprot

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Taxon | `-t` | `--taxon` | Taxon to filter results (human: 9606, mouse: 10090) |
| Freeform | `-f` | `--freeform` | Include flag to perform a freeform search through uniprot. Not including will default to searching for gene symbols. |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# Query uniprot for single terms
ggetrs uniprot query AP2S1

# Query uniprot for multiple terms
ggetrs uniprot query AP2S1 RFX3 NSD1

# Query uniprot with freeform search
ggetrs uniprot query -f rifin
```
