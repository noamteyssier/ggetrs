# Query IDs

Retrieves information for a list of NCBI IDs

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Usage

```bash
# query NCBI for AP2S1 (NCBI ID: 1175)
ggetrs ncbi query-ids 1175

# query NCBI for AP2S1 and NSD1 (1175 and 64324 respectively)
ggetrs ncbi query-ids 1175 64324
```
