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

install: build
  cargo install --path .

enrichr_query: build
  time target/release/ggetrs enrichr \
    -l GO_Biological_Process_2021 \
    -g AP2S1 NSD1 RFX3 LDB2

