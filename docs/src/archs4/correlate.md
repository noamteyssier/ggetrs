# Correlate

Performs a gene-correlation analysis using [ARCHS4](https://maayanlab.cloud/archs4).

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Count | `-c` | `--count` | number of values to recover [default: 100] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# Perform a gene-correlation analysis with ARCHS4
ggetrs archs4 correlate AP2S1

# Perform a gene-correlation analysis with ARCHS4
# But only return the top 10 results
ggetrs archs4 correlate -c 10 AP2S1
```

## Python

```python
import ggetrs

# Perform a gene-correlation analysis for AP2S1
# and return the top 10 results
ggetrs.archs4.correlate("AP2S1", 10)

# Perform a gene-correlation analysis for AP2S1
# and return the top 100 results
ggetrs.archs4.correlate("AP2S1", 100)
```
