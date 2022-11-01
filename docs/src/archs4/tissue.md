# Tissue

Performs a tissue-correlation analysis using [ARCHS4](https://maayanlab.cloud/archs4).

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Species | `-s` | `--species` | species of organism to recover [default: human] [possible values: human, mous] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |


## Command Line Interface

```bash
# Find tissue-level expression for AP2S1 in Humans
ggetrs archs4 tissue AP2S1

# Find tissue-level expression for AP2S1 in Mice
ggetrs archs4 tissue -s mouse AP2S1
```

## Python

```python
import ggetrs

# perform a tissue-correlation analysis for AP2S1 in Humans
ggetrs.archs4.tissue("AP2S1", "human")

# perform a tissue-correlation analysis for AP2S1 in Mice
ggetrs.archs4.tissue("AP2S1", "mouse")
```
