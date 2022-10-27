# Structure

## Help

```text
Retrieves pdb structure for a provided ID

Usage: ggetrs pdb structure [OPTIONS] <PDB_ID>

Arguments:
  <PDB_ID>  PDB id to retrieve structure

Options:
  -m, --header-only      Retrieve only the PDB header
  -f, --format <FORMAT>  Specify the structure format [default: pdb] [possible values: pdb, cif]
  -o, --output <OUTPUT>  Optional filepath to write output to [default=stdout]
  -h, --help             Print help information
  -V, --version          Print version information
```

## Usage

```bash
# return the pdb structure for AP2S1 (6URI)
ggetrs pdb info 6URI
```
