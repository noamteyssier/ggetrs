# ggetrs

A rust implementation of [gget](https://github.com/pachterlab/gget).

## Installation

### Command-Line Only

```bash
git clone https://github.com/noamteyssier/ggetrs
cd ggetrs
cargo install --path .
```

### Python Module

Before this is fully packaged you will need to install this separately using `maturin`

```bash
pip install maturin
```

#### Conda or Venv

If you are in a conda environment or a virtual environment you can use the
`maturin develop` interface.

```bash
maturin develop
```

#### No environment

Without a python environment you will need to build the package and subsequently
install the wheel.

```bash
maturin build
pip install target/wheels/<your_config>.whl
```

### Modules

#### Enrichr

##### Enrichr :: CLI

For querying `Enrichr`

```bash
ggetrs enrichr -l GO_Biological_Process_2021 AP2S1 NSD1 RFX3 LDB1
```

##### Enrichr :: Python

For querying `Enrichr`

```python3
import ggetrs
ggetrs.enrichr("GO_Biological_Process_2021", ["AP2S1", "NSD1", "RFX3", "LDB1"])
```

#### ARCHS4

The `ARCHS4` API currently has two submodules:

1. correlate
2. tissue

##### ARCHS4 :: CLI

```bash
# correlation module
ggetrs archs4 correlate AP2S1

# tissue module
ggetrs archs4 tissue AP2S1
```

##### ARCHS4 :: Python

```python3
import ggetrs

# correlation module
ggetrs.archs4.correlate("AP2S1", 10)

# tissue module
ggetrs.archs4.tissue("AP2S1", "human")
```
