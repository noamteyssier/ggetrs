# BLAST

## Help

The BLAST program can be determined from the provided input (will assign
either `blastn` or `blastp`) and the appropriate database will be used:
`nt` and `nr` respectively.

You may override these though by using their argument flags.
Keep in mind that there is no logic built into validating your inputs.
All non-default arguments will be passed to the BLAST API as is.

```text
Performs a BLAST query for a given sequence

Usage: ggetrs blast [OPTIONS] <QUERY>

Arguments:
  <QUERY>  query sequence to BLAST

Options:
  -p, --program <PROGRAM>    blast program to use [possible values: blastn, blastp, blastx, tblastn, tblastx]
  -d, --database <DATABASE>  blast database to use [possible values: nt, nr, refseq-rna, refseq-protein, swissprot, pdbaa, pdbnt]
  -l, --limit <LIMIT>        Number of hits to return [default: 50]
  -e, --expect <EXPECT>      Minimum expected value to consider [default: 10.0]
  -f, --low-comp-filter      Whether to use a complexity filter (default = false)
  -m, --megablast            Whether to use MEGABLAST (default = true)
  -o, --output <OUTPUT>      optional filepath to write output to [default=stdout]
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Usage

```bash
# Perform BLAST with a nucleotide sequence
ggetrs blast ATACTCAGTCACACAAGCCATAGCAGGAAACAGCGAGCTTGCAGCCTCACCGACGAGTCTCAACTAAAAGGGACTCCCGGAGCTAGGGGTGGGGACTCGGCCTCACACAGTGAGTGCCGG

# Perform BLAST with an amino acid sequence
ggetrs blast MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE

# Perform BLAST with an amino acid sequence using the PDBAA database
ggetrs blast -d pdbaa MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE
```
