# Enrichr

Perform an enrichment analysis on a list of genes using [Enrichr](https://maayanlab.cloud/Enrichr).

This requires at minimum a database name (listed [here](https://maayanlab.cloud/Enrichr/#libraries))
and any number of gene symbols to perform enrichment analysis on.

## Enrichr Help

```text
Perform an enrichment analysis on a list of genes using Enrichr

Usage: ggetrs enrichr [OPTIONS] --library <LIBRARY> <GENE_LIST>...

Arguments:
  <GENE_LIST>...  list of gene symbols to perform enrichment analysis on

Options:
  -l, --library <LIBRARY>  any database listed at: https://maayanlab.cloud/Enrichr/#libraries some shorthands include: pathway, transcription, ontology, diseases_drugs, celltypes, and kinase_interactions
  -o, --output <OUTPUT>    optional filepath to write output to [default=stdout]
  -h, --help               Print help information
  -V, --version            Print version information
```

### Library Shorthands

Some shorthands for the library are built into the program for convenience:

```text
pathway               KEGG_2021_Human
transcription         ChEA_2016
ontology              GO_Biological_Processes_2021
diseases_drugs        GWAS_Catalog_2019
celltypes             PangloaDB_Augmented_2021
kinase_interactions   KEA_2015
```

## Enrichr Usage

```bash
# Perform an enrichment analysis using Enrichr
ggetrs enrichr -l GO_Biological_Process_2021 AP2S1 NSD1 RFX3

# Perform an enrichment analysis with a shorthand library
# this is equivalent to the above search
ggetrs enrichr -l ontology AP2S1 NSD1 RFX3

# Perform an enrichment analysis on pathway
ggetrs enrichr -l pathway AP2S1 NSD1 RFX3
```
