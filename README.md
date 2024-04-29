# mintcash-bindings
CosmWasm bindings to custom [Mint Cash](https://mintca.sh) features.

## Codegen

Fetch all submodules located in the `deps/*` directory. Our `.gitmodules` file contains `update = none`, [which instructs Cargo to skip cloning submodules](https://github.com/rust-lang/cargo/issues/4247#issuecomment-1149178736) when crates are imported into Cargo projects via Git. That's why we need to [explicitly override this behavior using the `--checkout` option](https://stackoverflow.com/questions/63974493/why-would-git-submodule-update-skip-a-submodule) when running the `mintcash-rust-generator`:

```bash
git submodule update --init --checkout --recursive
```

Generate code for `mintcash-rust`, a proto library for interacting with the Mint Cash blockchain:

```bash
cargo run --bin mintcash-rust-generator
```
> Note: If you have trouble in running above command, you have to install cmake and buf: 
```bash
brew install cmake bufbuild/buf/buf
```

You can now update `MintcashMsg` (`CustomMsg`) and `MintcashQuery` (`CustomQuery`) in `mintcash-bindings`.

## Build Tester Contracts

This will produce wasm file for `contracts/reflect` and `contracts/stargate-tester`:

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.8
```
