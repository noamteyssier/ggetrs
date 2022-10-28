# List

Lists available libraries and their statistics available on [Enrichr](https://maayanlab.cloud/Enrichr).

## Help

```text
List all available libraries and their descriptions

Usage: ggetrs enrichr list [OPTIONS]

Options:
  -m, --minimal              Return library names in plaintext
  -t, --list-categories      List the categorization of libraries
  -c, --category <CATEGORY>  Filter to a category ID
  -o, --output <OUTPUT>      optional filepath to write output to [default=stdout]
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Usage

```bash
# List all available libraries and their metadata
ggetrs enrichr list

# List all available libraries and their metadata in a minimal format
ggetrs enrichr list -m

# List the categorization of libraries
ggetrs enrichr list -t

# Filter libraries an their metadata that belong to a category ID
# example category ID 2 = pathways
ggetrs enrichr list -c 2

# Filter libraries and print in a minimal format
ggetrs enrichr list -c 2 -m
```
