# mintcash-bindings
CosmWasm bindings to custom Mint Cash features.

Fetch all submodules located in the `deps/*` directory. Our `.gitmodules` file contains `update = none`, which instructs Cargo to skip cloning submodules when crates are imported into Cargo projects via Git. That's why we need to [explicitly override this behavior using the `--checkout` option](Learn more about this override process here](https://stackoverflow.com/questions/63974493/why-would-git-submodule-update-skip-a-submodule) when running the `mintcash-rust-generator`:

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
