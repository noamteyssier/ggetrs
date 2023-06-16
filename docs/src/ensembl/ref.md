# Ref

Retrieves reference files from the Ensembl FTP site.

## Help

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Species | `-s` | `--species` | Species to query data for [default: homo_sapiens] |
| Release | `-r` | `--release` | Release to use - will default to latest release |
| Data Type | `-d` | `--datatype` | Datatype to query for - provided as a comma-separated list |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |
| Download | `-D` | `--download` | Download all requested files to the current working directory |

## Command Line Interface

```bash
# returns the url for human genome (default)
ggetrs ensembl ref

# returns the url for the human cdna transcriptome
ggetrs ensembl ref -d cdna

# returns the url for the human cdna transcriptome and genome
ggetrs ensembl ref -d cdna,dna

# returns the url for the mouse cdna transcriptome and genome
ggetrs ensembl ref -d cdna,dna -s mus_musculus

# downloads the requested files to the current directory
ggetrs ensembl ref -d cdna,dna,gtf -s homo_sapiens
```

## Python

```python
import ggetrs

# returns the url for human genome (default)
ggetrs.ensembl.reference()

# returns the url for the human cdna transcriptome
ggetrs.ensembl.reference(
  datatype="cdna"
)

# returns the url for the human cdna transcriptome and genome
ggetrs.ensembl.reference(
  datatype=["cdna", "dna"]
)

# returns the url for the mouse cdna transcriptome and genome
ggetrs.ensembl.reference(
  datatype=["cdna", "dna"], 
  species="mus_musculus"
)
```
