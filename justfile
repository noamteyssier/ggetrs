build: 
  cargo build --release

clean:
  cargo clean

enrichr_query: build
  time target/release/ggetrs enrichr \
    -l GO_Biological_Process_2021 \
    -g AP2S1 NSD1 RFX3 LDB2
