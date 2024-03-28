use cosmwasm_schema::write_api;

use stargate_tester::msg::{ExecuteMsg, InstantiateMsg};
use mintcash_bindings::MintcashQuery;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: MintcashQuery,
    }
}
