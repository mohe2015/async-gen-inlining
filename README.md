# async-gen-inlining

```bash
RUSTFLAGS="-Zprint-type-sizes" cargo build | tee type-sizes.txt
# `{async gen fn
# 24 bytes

RUSTFLAGS="-Zprint-type-sizes" cargo build --release | tee type-sizes.txt

```