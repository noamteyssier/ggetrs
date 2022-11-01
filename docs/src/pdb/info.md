# Info

Retrieves pdb information for a provided ID and resource

## Arguments

| Name | Short | Long | Description |
|------|-------|------|-------------|
| Resource | `-r` | `--resource` | Specify the structure format [default: entry] [possible values: entry, pubmed, assembly, branched-entity, nonpolymer-entity, polymer-entity, uniprot, branched-entity-instance, polymer-entity-instance, nonpolymer-entity-instance] |
| Identifier | `-i` | `--identifier` | Specifies the Entry or Chain Identifier |
| Output | `-o` | `--output` | optional filepath to write output to [default=stdout] |

## Command Line Interface

```bash
# return information for AP2S1 (6URI)
ggetrs pdb info 6URI
```

