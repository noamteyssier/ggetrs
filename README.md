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
