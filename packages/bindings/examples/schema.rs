use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use mintcash_bindings::{
    ExchangeRateItem, ExchangeRatesResponse, MintcashMsg, MintcashQuery, SwapResponse,
    TaxCapResponse, TaxRateResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(MintcashMsg), &out_dir);
    export_schema(&schema_for!(MintcashQuery), &out_dir);
    export_schema(&schema_for!(ExchangeRateItem), &out_dir);
    export_schema(&schema_for!(ExchangeRatesResponse), &out_dir);
    export_schema(&schema_for!(TaxCapResponse), &out_dir);
    export_schema(&schema_for!(TaxRateResponse), &out_dir);
    export_schema(&schema_for!(SwapResponse), &out_dir);
}
