# Lookup-Id

Lookup information for genes/transcripts providing ensembl ids

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Names | `-n` | `--names` | Returns a minimal output of only the found gene names |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Usage

```bash
# Query information for AP2S1 (ENSG00000042753)
ggetrs ensembl lookup-id ENSG00000042753

# Query Information for AP2S1 (ENSG00000042753) and NSD1 (ENSG00000165671)
ggetrs ensembl lookup-id ENSG00000042753 ENSG00000165671

# Query information for AP2S1 (ENSG00000042753) and NSD1 (ENSG00000165671)
# but only return their found gene names
# (useful for translating between ensembl IDs and gene symbols)
ggetrs ensembl lookup-id -n ENSG00000042753 ENSG00000165671
```
