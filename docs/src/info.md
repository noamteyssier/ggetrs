# Info

Fetch extensive gene and transcript metadata from [Ensembl](https://ensembl.org),
[Uniprot](https://uniprot.org), and [NCBI](https://ncbi.nlm.nih.gov).

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Species | `-s` | `--species` | Species name to use: currently this MUST match the taxon_id [default: homo_sapiens] |
| Taxon ID | `-t` | `--taxon-id` | Taxon ID to use: currently this MUST match the taxon_id [default: homo_sapiens] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Usage

```bash
# Queries information for single term
ggetrs info AP2S1

# Queries information for multiple terms
ggetrs info AP2S1 RFX3 NSD1
```

## Python

```python
import ggetrs

# Queries information for single term
ggetrs.info(["AP2S1"])

# Queries information for multiple terms
ggetrs.info(["AP2S1", "RFX3", "NSD1"])
```
