# Info

## Help

```text
Retrieves pdb information for a provided ID and resource

Usage: ggetrs pdb info [OPTIONS] <PDB_ID>

Arguments:
  <PDB_ID>  PDB Id to request info

Options:
  -r, --resource <RESOURCE>      Specify the structure format [default: entry] [possible values: entry, pubmed, assembly, branched-entity, nonpolymer-entity, polymer-entity, uniprot, branched-entity-instance, polymer-entity-instance, nonpolymer-entity-instance]
  -i, --identifier <IDENTIFIER>  Specifies the Entry or Chain Identifier
  -o, --output <OUTPUT>          Optional filepath to write output to [default=stdout]
  -h, --help                     Print help information
  -V, --version                  Print version information
```

## Usage

```bash
# return information for AP2S1 (6URI)
ggetrs pdb info 6URI
```

