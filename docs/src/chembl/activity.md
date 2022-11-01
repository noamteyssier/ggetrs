# Activity

Queries chemical bioactivity for a provided protein target and return all small molecules.

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Limit | `-l` | `--limit` | Number of results to return [default: 500] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Usage

```bash
# Query the Chembl database for small molecules with bioactivity targeting NSD1
ggetrs chembl activity NSD1

# Query the Chembl database for the top 20 bioactive molecules for NSD1
ggetrs chembl activity -l 20 NSD1
```
