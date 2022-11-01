# BLAT

Perform a [BLAT](https://genome.ucsc.edu/cgi-bin/hgBlat) search using the UCSC Genome Browser.

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Sequence Type | `-s` | `--seqtype` | Specify the structure format [default: dna] [possible values: dna, protein, translated-rna, translated-dna] |
| Database Name | `-d` | `--db-name` | Specifies the database name to query [default: hg38] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# query UCSC genome browser for the first 121 bp of AP2S1
ggetrs ucsc blat GGGCCCTACAACTGCACCCTGAGCCGGAGCTGCCCAGTCGCCGCGGGACCGGGGCCGCTGGGGTCTGGACGGGGGTCGCCATGGTAACGGGGGAGCGCTACGCCGGGGACTGGCGGAGGG
```

## Python

```python
import ggetrs

# query UCSC genome browser for the first 121 bp of AP2S1
ggetrs.ucsc.blat(
  "GGGCCCTACAACTGCACCCTGAGCCGGAGCTGCCCAGTCGCCGCGGGACCGGGGCCGCTGGGGTCTGGACGGGGGTCGCCATGGTAACGGGGGAGCGCTACGCCGGGGACTGGCGGAGGG"
)

# query UCSC genome browser with amino acid sequence
ggetrs.ucsc.blat(
  "MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE",
  seqtype="protein"
)
```
