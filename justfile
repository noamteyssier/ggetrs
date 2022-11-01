run: enrichr archs4 ensembl

enrichr: enrichr_query

archs4: archs4_correlate archs4_tissue

ensembl: ensembl_search ensembl_database ensembl_release

install: 
  cargo install --path .

develop:
  maturin develop --release

build: 
  cargo build --release

clean:
  cargo clean

doc:
  cargo doc \
    --verbose

test:
  cargo nextest run --retries 10

pytest:
  pytest -v --reruns 10 --reruns-delay 1

lint: build
  cargo clippy -- \
    -W clippy::pedantic \
    -A clippy::missing_errors_doc \
    -A clippy::module_name_repetitions \
    -A clippy::cast_precision_loss \
    -A clippy::cast_sign_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::missing_panics_doc \
    -A clippy::used_underscore_binding \
    -A clippy::borrow_deref_ref

lint-fix: build
  cargo clippy --fix -- \
    -W clippy::pedantic \
    -A clippy::missing_errors_doc \
    -A clippy::module_name_repetitions \
    -A clippy::cast_precision_loss \
    -A clippy::cast_sign_loss \
    -A clippy::cast_possible_truncation


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

ensembl_search: build
  time target/release/ggetrs search \
    RFX3 AP2S1

ensembl_database: build
  time target/release/ggetrs ensembl database \
    -f homo_sapiens

ensembl_release: build
  time target/release/ggetrs ensembl release

ensembl_lookup_id: build
  time target/release/ggetrs ensembl lookup-id \
    ENSG00000080298

uniprot_query_ensembl: build
  time target/release/ggetrs uniprot query \
    ENSG00000080298 \
    ENSG00000042753 

uniprot_query_genes: build
  time target/release/ggetrs uniprot query \
    RFX3 \
    AP2S1 

seq_symbol: build
  time target/release/ggetrs seq AP2S1
