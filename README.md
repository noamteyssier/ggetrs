# ggetrs

A rust implementation of [gget](https://github.com/pachterlab/gget).

## Installation

```bash
git clone https://github.com/noamteyssier/ggetrs
cd ggetrs
cargo install --path .
```

### Modules

#### Enrichr

For querying `Enrichr`

```bash
ggetrs enrichr -l <library_name> -g <gene_name_1> <...> <gene_name_2>
```
