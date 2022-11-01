# Modules

`ggetrs` currently consists of the following modules:

## Functional

These modules perform single line utility functions like performing a gene-set enrichment analysis or returning the PDB structure of an input protein.

| Module Name | Description |
|-------------|-------------|
| [`enrichr`](./enrichr.md)|Perform an enrichment analysis on a list of genes using [Enrichr](https://maayanlab.cloud/Enrichr)|
| [`archs4`](./archs4.md)|Find the most correlated genes to a gene of interest or find the gene's tissue expression atlas using [ARCHS4](https://maayanlab.cloud/archs4)|
| [`blast`](./blast.md)|BLAST a nucleotide or amino acid sequence to any [BLAST](https://blast.ncbi.nlm.nih.gov/Blast.cgi) database|
| [`search`](./search.md)|Fetch genes and transcripts from [Ensembl](https://ensembl.org) using free-form search terms.|
| [`info`](./info.md)|Fetch extensive gene and transcript metadata from [Ensembl](https://ensembl.org), [Uniprot](https://uniprot.org), and [NCBI](https://ncbi.nlm.nih.gov).|
| [`seq`](./seq.md)|Fetch nucleotide or amino acid sequences of genes or transcripts from [Ensembl](https://ensembl.org) or [Uniprot](https://uniprot.org) respectively.|
| [`ucsc`](./ucsc.md)|Perform a [BLAT](https://genome.ucsc.edu/cgi-bin/hgBlat) search using the UCSC Genome Browser.|
| [`pdb`](./pdb.md)|Get structure and metadata of a protein from the [RCSB Protein Data Bank](https://rcsb.org)|


## Database Queries

These modules perform descriptive searches by querying databases directly using their publicly available APIs.

| Module Name | Description |
|-------------|-------------|
|[`chembl`](./chembl.md)|Perform a bioactivity search for any protein of interest using [Chembl](https://ebi.ac.uk/chembl)|
|[`ensembl`](./ensembl.md)|Perform [Ensembl](https://ensembl.org) related queries from their public [API](https://rest.ensembl.org).|
|[`uniprot`](./uniprot.md)| Query [Uniprot](https://uniprot.org) directly for gene/protein information.|
|[`ncbi`](./ncbi.md)| Query [NCBI](https://ncbi.nlm.nih.gov) directly for gene/protein information.|

## Quality of Life

These modules improve the quality of life of anybody using a terminal.

| Module Name | Description |
|-------------|-------------|
|[`autocomplete`](./autocomplete.md)| Generates an autocompletion file for your shell of choice for a better terminal experience.|
