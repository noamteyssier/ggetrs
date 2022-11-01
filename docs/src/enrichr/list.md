# List

Lists available libraries and their statistics available on [Enrichr](https://maayanlab.cloud/Enrichr).

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Minimal | `-m` | `--minimal` | Return only library names in results |
| List Categories | `-t` | `--list-categories` | List the categorization of libraries |
| Categories | `-c` | `--category` | Filter libraries with a specified category ID |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

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
