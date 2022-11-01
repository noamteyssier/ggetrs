# Taxons

This retrieves possible taxons from an incomplete query string.

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Limit | `-l` | `--limit` | Number of search results to return [default: 5] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# return all taxons that contain the substring 'sapiens'
ggetrs ncbi taxons sapiens

# return the first 3 taxons that contain the substring 'sapi'
ggetrs ncbi taxons -l 3 sapi
```
