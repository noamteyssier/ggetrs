# Database

Prints all available databases on Ensembl's SQL server.

This is used if you are interested in querying a specific database and can be passed into [`ggetrs search`](./search.md).

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Filter | `-f` | `--filter` | Provides a substring filter to only return databases which contain the substring |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# show all databases in the SQL server
ggetrs ensembl database

# filter for databases with the `sapiens` substring
ggetrs ensembl database -f sapiens

# filter for databases with the `cerevisiae` substring
ggetrs ensembl database -f cerevisiae
```

## Python

```python
import ggetrs

# show all databases in the SQL server
ggetrs.ensembl.database()

# filter for databases with the `sapiens` substring
ggetrs.ensembl.database("sapiens")

# filter for databases with the `cerevisiae` substring
ggetrs.ensembl.database("cerevisiae")
```
