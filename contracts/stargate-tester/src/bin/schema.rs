use cosmwasm_schema::write_api;

use mintcash_bindings::MintcashQuery;
use stargate_tester::msg::{ExecuteMsg, InstantiateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: MintcashQuery,
    }
}
