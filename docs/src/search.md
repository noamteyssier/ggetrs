# Search

Searches through descriptions on ENSEMBL using free-form search terms.

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Database | `-d` | `--database` | Name of Ensembl database to use. |
| Species | `-s` | `--species` | Species used in database [default: homo_sapiens] |
| Database Type | `-t` | `--db-type` | Database type specified by Ensembl [default: core] |
| Release | `-r` | `--release` | release version number to use for database |
| Assembly | `-a` | `--assembly` | Assembly to use for species [default: 38] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# searches Ensembl for all genes with `clathrin` in the description
ggetrs search clathrin

# searches Ensembl for all genes with `clathrin` OR `heavy` in the description
ggetrs search clathrin heavy

# searchs Ensembl for all genes with `clathrin heavy` in the description
ggetrs search "clathrin heavy"
```

## Python

```python
import ggetrs

# searches Ensembl for all genes with `clathrin` in the description
ggetrs.search(["clathrin"])

# searches Ensembl for all genes with `clathrin` or `heavy` in the description
ggetrs.search(["clathrin", "heavy"])

# searchs Ensembl for all genes with `clathrin heavy` in the description
ggetrs.search(["clathrin heavy"])
```
