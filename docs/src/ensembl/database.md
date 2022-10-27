# Database

Prints all available databases on Ensembl's SQL server.

This is used if you are interested in querying a specific database and can be passed into [`ggetrs search`](./search.md).

## Help

```text
Prints all available databases on Ensembl's SQL database

Usage: ggetrs ensembl database [OPTIONS]

Options:
  -f, --filter <FILTER>  Provides a substring filter to only return databases which contain the substring
  -o, --output <OUTPUT>  optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

## Usage

```bash
# show all databases in the SQL server
ggetrs ensembl database

# filter for databases with the `sapiens` substring
ggetrs ensembl database -f sapiens

# filter for databases with the `cerevisiae` substring
ggetrs ensembl database -f cerevisiae
```
