#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use mintcash_bindings::{MintcashQuery, MintcashStargateQuerier};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stargate-tester";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: MintcashQuery) -> StdResult<Binary> {
    let querier = MintcashStargateQuerier::new(&deps.querier);

    match msg {
        MintcashQuery::Swap {
            offer_coin,
            ask_denom,
        } => to_json_binary(&querier.query_swap(offer_coin, ask_denom)?),
        MintcashQuery::TaxCap { denom } => to_json_binary(&querier.query_tax_cap(denom)?),
        MintcashQuery::TaxRate {} => to_json_binary(&querier.query_tax_rate()?),
        MintcashQuery::ExchangeRates {
            base_denom,
            quote_denoms,
        } => to_json_binary(&querier.query_exchange_rates(base_denom, quote_denoms)?),
    }
}

#[cfg(test)]
mod tests {}
