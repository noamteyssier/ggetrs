# Lookup-Symbol

Lookup information for genes/transcripts providing symbols and species

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Species | `-s` | `--species` | Species to specify [default: homo_sapiens] |
| IDs | `-i` | `--ids` | Return a minimal output of only the found Ensembl IDs |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Usage

```bash
# Query information for AP2S1
ggetrs ensembl lookup-symbol AP2S1

# Query information for AP2S1 and NSD1
ggetrs ensembl lookup-symbol AP2S1 NSD1

# Query information for AP2S1 and NSD1 in mice
ggetrs ensembl lookup-symbol -s mus_musculus AP2S1 NSD1

# Query information for AP2S1 and NSD1 but only return Ensembl IDs
# (useful for translating between Ensembl IDs and gene symbols)
ggetrs ensembl lookup-symbol -i AP2S1 NSD1
```
