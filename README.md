# mintcash-bindings
Mint Cash bindings smart contracts for CosmWasm/wasmd v1.1.0+

[Checkout all submodules under `deps/*`:](https://stackoverflow.com/questions/63974493/why-would-git-submodule-update-skip-a-submodule)

```bash
git submodule update --checkout
```

Generate code for `mintcash-rust`:

```bash
cargo run --bin mintcash-rust-generator
```

This will produce wasm file for `mintcash-bindings`:

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.8
```
