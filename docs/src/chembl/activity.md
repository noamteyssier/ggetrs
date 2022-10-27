# Activity

## Help

```text
Queries chemical activity for a provided item

Usage: ggetrs chembl activity [OPTIONS] <QUERY>

Arguments:
  <QUERY>  Query to retrieve bioactivity

Options:
  -l, --limit <LIMIT>    Number of results to return [default: 500]
  -o, --output <OUTPUT>  Optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

## Usage

```bash
# Query the Chembl database for small molecules with bioactivity targeting NSD1
ggetrs chembl activity NSD1
```
