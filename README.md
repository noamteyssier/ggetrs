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

##### Command Line Interface

For querying `Enrichr`

```bash
ggetrs enrichr -l GO_Biological_Process_2021 AP2S1 NSD1 RFX3 LDB1
```

##### Python Interface

For querying `Enrichr`

```python3
import ggetrs
ggetrs.enrichr("GO_Biological_Process_2021", ["AP2S1", "NSD1", "RFX3", "LDB1"])
```
