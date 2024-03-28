//! Build Classic proto files. This build script clones the CosmosSDK and Classic version
//! specified in the COSMOS_SDK_REV and CLASSIC_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{env, path::PathBuf};

use classic_rust_generator::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.46.14-terra.4";

/// The classic commit or tag to be cloned and used to build the proto files
const CLASSIC_REV: &str = "v2.3.2";

/// The wasmd commit or tag to be cloned and used to build the proto files
const WASMD_REV: &str = "v0.30.0-terra.3";

/// The cometbft commit or tag to be cloned and used to build the proto files
const COMETBFT_REV: &str = "v0.34.29-terra.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../proto/src/types";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../deps/cosmos-sdk";
/// Directory where the classic submodule is located
const CLASSIC_DIR: &str = "../deps/classic";
/// Directory where the wasmd submodule is located
const WASMD_DIR: &str = "../deps/wasmd";
/// Directory where the cometbft submodule is located
const COMETBFT_DIR: &str = "../deps/cometbft";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn generate() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodule(COSMOS_SDK_DIR, COSMOS_SDK_REV);
        git::update_submodule(CLASSIC_DIR, CLASSIC_REV);
        git::update_submodule(WASMD_DIR, WASMD_REV);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let classic_project = CosmosProject {
        name: "terra".to_string(),
        version: CLASSIC_REV.to_string(),
        project_dir: CLASSIC_DIR.to_string(),
        include_mods: vec![
            "market".to_string(),
            "oracle".to_string(),
            "treasury".to_string()
        ],
    };
    let wasmd_project = CosmosProject {
        name: "wasmd".to_string(),
        version: WASMD_REV.to_string(),
        project_dir: WASMD_DIR.to_string(),
        include_mods: vec![],
    };
    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: COSMOS_SDK_REV.to_string(),
        project_dir: COSMOS_SDK_DIR.to_string(),
        include_mods: vec![
            "auth".to_string(),
            "authz".to_string(),
            "bank".to_string(),
            "base".to_string(),
            "gov".to_string(),
            "feegrant".to_string(),
            "staking/v1beta1/genesis.proto".to_string(),
            "staking/v1beta1/staking.proto".to_string(),
            "staking/v1beta1/tx.proto".to_string(),
            "base/abci/v1beta1/abci.proto".to_string()
        ],
    };
    let cometbft_project = CosmosProject {
        name: "tendermint".to_string(),
        version: COMETBFT_REV.to_string(),
        project_dir: COMETBFT_DIR.to_string(),
        include_mods: vec![
            "types".to_string(),
            "crypto".to_string(),
            "version".to_string(),
            "abci".to_string(),
        ],
    };

    let classic_code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        classic_project,
        vec![cometbft_project, cosmos_project, wasmd_project],
    );

    classic_code_generator.generate();
}

fn main() {
    pretty_env_logger::init();
    generate();
}
