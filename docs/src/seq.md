# Seq

Returns nucleotide or amino acid sequence for a provided ensembl ID or gene symbol.

If gene symbols are provided instead of ensembl IDs for nucleotide sequences
those symbols will first be matched to an ensembl ID with the same functionality
of [`ggetrs ensembl lookup-symbol`](./ensembl/lookup-symbol.md).

All returned sequences are guaranteed to be in the same order as provided ids/symbols.

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Translate | `-t` | `--translate` | Return the amino acid sequence instead of nucleotide sequence |
| Species | `-s` | `--species` | Species to specify when not using an Ensembl ID [default: homo_sapiens] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# recover nucleotide sequence for AP2S1 (ENSG00000042753)
ggetrs seq ENSG00000042753

# recover nucleotide sequence for AP2S1
ggetrs seq AP2S1

# recover nucleotide sequence for AP2S1 (ENSG00000042753) and NSD1
ggetrs seq ENSG00000042753 NSD1

# recover amino acid sequence for AP2S1 (ENSG00000042753)
ggetrs seq -t ENSG00000042753

# recover amino acid sequence for AP2S1
ggetrs seq -t AP2S1

# recover amino acid sequences for AP2S1 and NSD1 and RFX3
ggetrs seq -t AP2S1 NSD1 RFX3
```

## Python

```python
import ggetrs

# recover nucleotide sequence for AP2S1 (ENSG00000042753)
ggetrs.seq(["ENSG00000042753"])

# recover nucleotide sequence for AP2S1
ggetrs.seq(["AP2S1"])

# recover nucleotide sequence for AP2S1 (ENSG00000042753) and NSD1
ggetrs.seq(["ENSG00000042753", "NSD1"])

# recover amino acid sequence for AP2S1 (ENSG00000042753)
ggetrs.seq(["ENSG00000042753"], translate=True)

# recover amino acid sequence for AP2S1
ggetrs.seq(["AP2S1"], translate=True)

# recover amino acid sequences for multiple transcripts
ggetrs.seq(["AP2S1", "NSD1", "RFX3"], translate=True)
```
