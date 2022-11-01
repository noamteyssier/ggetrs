# Structure

Retrieves pdb structure for a provided ID

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Header Only | `-m` | `--header-only` | Retrieve only the PDB Header |
| Format | `-f` | `--format` | Specify the structure format [default: pdb] [possible values : pdb, cif] |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Usage

```bash
# return the pdb structure for AP2S1 (6URI)
ggetrs pdb structure 6URI

# return the pdb structure for AP2S1 (6URI) as a `.cif`
ggetrs pdb structure -f cif 6URI

# return the header for AP2S1 (6URI)
ggetrs pdb structure -m 6URI
```
