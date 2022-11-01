# BLAST

## Help

The BLAST program can be determined from the provided input (will assign
either `blastn` or `blastp`) and the appropriate database will be used:
`nt` and `nr` respectively.

You may override these though by using their argument flags.
Keep in mind that there is no logic built into validating your inputs.
All non-default arguments will be passed to the BLAST API as is.

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Program | `-p` | `--program` | blast program to use [possible values: blastn, blastp, blastx, tblastn, tblastx] |
| Database | `-d` | `--database` | blast database to use [possible values: nt, nr, refseq-rna, refseq-protein, swissprot, pdbaa, pdbnt] |
| Limit | `-l` | `--limit` | Number of hits to return [default: 50] |
| Expect | `-e` | `--expect` | Minimum expected value to consider [default: 10.0] |
| Low Complexity Filter | `-f` | `--low-comp-filter` | Include flag to use a complexity filter [default = false] |
| MEGABLAST | `-m` | `--megablast` | Whether to use MEGABLAST (default = false) |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# Perform BLAST with a nucleotide sequence
ggetrs blast ATACTCAGTCACACAAGCCATAGCAGGAAACAGCGAGCTTGCAGCCTCACCGACGAGTCTCAACTAAAAGGGACTCCCGGAGCTAGGGGTGGGGACTCGGCCTCACACAGTGAGTGCCGG

# Perform BLAST with an amino acid sequence
ggetrs blast MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE

# Perform BLAST with an amino acid sequence using the PDBAA database
ggetrs blast -d pdbaa MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE
```

## Python

```python
import ggetrs

# Perform BLAST with a nucleotide sequence
ggetrs.blast(
  "ATACTCAGTCACACAAGCCATAGCAGGAAACAGCGAGCTTGCAGCCTCACCGACGAGTCTCAACTAAAAGGGACTCCCGGAGCTAGGGGTGGGGACTCGGCCTCACACAGTGAGTGCCGG"
)

# Perform BLAST with an amino acid sequence
ggetrs.blast(
  "MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE"
)

# Perform BLAST with an amino acid sequence using the PDBAA database
ggetrs.blast(
  "MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE",
  database = "pdbaa"
)

# Perform BLAST with an amino acid sequence using the PDBAA database with a low complexity filter and a limit
ggetrs.blast(
  "MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE",
  database = "pdbaa",
  limit = 10,
  low_comp_filter=True,
)
```
