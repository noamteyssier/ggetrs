run: enrichr archs4

enrichr: enrichr_query

archs4: archs4_correlate archs4_tissue

develop: build
  maturin develop --release

build: 
  cargo build --release

clean:
  cargo clean

doc:
  cargo doc \
    --verbose

lint: build
  cargo clippy -- \
    -W clippy::pedantic \
    -A clippy::missing_errors_doc \
    -A clippy::module_name_repetitions

install: develop
  cargo install --path .


enrichr_query: build
  time target/release/ggetrs enrichr \
    -l GO_Biological_Process_2021 \
    AP2S1 NSD1 RFX3 LDB2

archs4_correlate: build
  time target/release/ggetrs archs4 correlate \
    AP2S1
    
archs4_tissue: build
  time target/release/ggetrs archs4 tissue \
    AP2S1
    

