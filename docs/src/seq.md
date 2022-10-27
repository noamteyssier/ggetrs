# Seq

Returns nucleotide or amino acid sequence for a provided ensembl ID or gene symbol.

If gene symbols are provided instead of ensembl IDs for nucleotide sequences
those symbols will first be matched to an ensembl ID with the same functionality
of [`ggetrs ensembl lookup-symbol`](./ensembl/lookup-symbol.md).

All returned sequences are guaranteed to be in the same order as provided ids/symbols.

## Help

```text
Queries sequences from ensembl and UniProt

Usage: ggetrs seq [OPTIONS] <SEARCH_TERMS>...

Arguments:
  <SEARCH_TERMS>...  Search terms to query (can be Ensembl IDs or Gene Symbols)

Options:
  -t, --transcribe         Return the amino acid sequence instead of nucleotide sequence
  -s, --species <SPECIES>  Species/alias to specify [default: homo_sapiens]
  -o, --output <OUTPUT>    optional filepath to write output to [default=stdout]
  -h, --help               Print help information
  -V, --version            Print version information
```

## Usage

```bash
# recover nucleotide sequence for AP2S1 (ENSG00000042753)
ggetrs seq ENSG00000042753

# recover nucleotide sequence for AP2S1
ggetrs seq AP2S1

# recover nucleotide sequence for AP2S1 (ENSG00000042753) and NSD1
ggetrs seq ENSG00000042753 NSD1

# recover amino acid sequence for AP2S1 (ENSG00000042753)
ggetrs seq -t ENSG00000042753

# recover amino acid sequence for AP2S1
ggetrs seq -t AP2S1

# recover amino acid sequences for AP2S1 and NSD1 and RFX3
ggetrs seq -t AP2S1 NSD1 RFX3
```
