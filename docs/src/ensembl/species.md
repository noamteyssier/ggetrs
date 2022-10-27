# Species

Returns the available species in the Ensembl FTP site

## Help

```text
Retrieves the list of species from ENSEMBL FTP site

Usage: ggetrs ensembl species [OPTIONS]

Options:
  -r, --release <RELEASE>    Release to use - will default to latest release [default: 108]
  -o, --output <OUTPUT>      Optional filepath to write output to [default=stdout]
  -d, --datatype <DATATYPE>  Datatype to query species list [default: dna] [possible values: cdna, cds, dna, gff3, gtf, ncrna, pep]
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Usage

```bash
# return all species where there is a genome (i.e. dna)
ggetrs ensembl species

# return all species where there is a transcriptome (i.e. cdna)
ggetrs ensembl species -d cdna


# return all species where there is a transcriptome (i.e. cdna)
# for an older release
ggetrs ensembl species -d cdna -r 60
```
