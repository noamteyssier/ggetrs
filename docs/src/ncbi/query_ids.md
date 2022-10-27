# Query IDs

## Help

```text
Retrieves information for a list of IDs

Usage: ggetrs ncbi query-ids [OPTIONS] <IDS>...

Arguments:
  <IDS>...  NCBI ids to query

Options:
  -o, --output <OUTPUT>  optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

## Usage

```bash
# query NCBI for AP2S1 (ncbi ID: 1175)
ggetrs ncbi query-ids 1175

# query NCBI for AP2S1 and NSD1 (1175 and 64324 respectively)
ggetrs ncbi query-ids 1175 64324
```
