# Species

Returns the available species in the Ensembl FTP site

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Release | `-r` | `--release` | Ensembl release version to use - will default to latest release |
| Data Type | `-d` | `--datatype` | Datatype to query species list [default: dna] [possible values: cdna, cds, dna, gff3, gtf, ncrna, pep] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# return all species where there is a genome (i.e. dna)
ggetrs ensembl species

# return all species where there is a transcriptome (i.e. cdna)
ggetrs ensembl species -d cdna

# return all species where there is a transcriptome (i.e. cdna)
# for an older release
ggetrs ensembl species -d cdna -r 60
```

## Python

```python
import ggetrs

# return all species where there is a genome (i.e. dna)
ggetrs.ensembl.species()

# return all species where there is a transcriptome (i.e. cdna)
ggetrs.ensembl.species(dataype="dna")

# return all species where there is a transcriptome (i.e. cdna)
# for an older release
ggetrs.ensembl.species(dataype="cdna", release=60)
```
