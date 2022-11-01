# Enrichr

Perform an enrichment analysis on a list of genes using [Enrichr](https://maayanlab.cloud/Enrichr).

This requires at minimum a database name (listed [here](https://maayanlab.cloud/Enrichr/#libraries))
and any number of gene symbols to perform enrichment analysis on.

## Library Shorthands

Some shorthands for the library are built into the program for convenience.
These can be used in the command line interface or in the python interface.

| Alias | Library |
|-------|---------|
| pathway               | KEGG_2021_Human | 
| transcription         | ChEA_2016 | 
| ontology              | GO_Biological_Processes_2021 | 
| diseases_drugs        | GWAS_Catalog_2019 | 
| celltypes             | PangloaDB_Augmented_2021 | 
| kinase_interactions   | KEA_2015 | 

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Library | `-l` | `--library` | a [library shorthand](#library-shorthands) or any [Enrichr library](https://maayanlab.cloud/Enrichr/#libraries) |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |


## Command Line Interface

```bash
# Perform an enrichment analysis using Enrichr
ggetrs enrichr enrichr -l GO_Biological_Process_2021 AP2S1 NSD1 RFX3

# Perform an enrichment analysis with a shorthand library
# this is equivalent to the above search
ggetrs enrichr enrichr -l ontology AP2S1 NSD1 RFX3

# Perform an enrichment analysis on pathway
ggetrs enrichr enrichr -l pathway AP2S1 NSD1 RFX3
```

## Python

```python
import ggetrs

# Search using the ontology shorthand
ggetrs.enrichr("ontology", ["AP2S1", "RFX3", "NSD1"])

# Search using the kinase_interactions shorthand
ggetrs.enrichr("kinase_interactions", ["AP2S1", "RFX3", "NSD1"])
```
