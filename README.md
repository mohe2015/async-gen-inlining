# async-gen-inlining

```bash
cargo build | tee type-sizes.txt
# `{async gen fn
# 24 bytes

cargo build --release | tee type-sizes.txt

cargo build --release && ls -lh target/x86_64-unknown-linux-gnu/release/
```