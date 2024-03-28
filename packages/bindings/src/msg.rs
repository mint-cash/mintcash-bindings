use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg};

/// A number of Custom messages that can call into the Mint Cash bindings
#[cw_serde]
pub enum MintcashMsg {
    // swap
    Swap {
        offer_coin: Coin,
        ask_denom: String
    },
    // swap send
    SwapSend {   
        to_address: String,
        offer_coin: Coin,
        ask_denom: String
    }
}

impl MintcashMsg {

    // create swap msg
    pub fn create_swap_msg(offer_coin: Coin, ask_denom: String) -> Self {
        MintcashMsg::Swap {
            offer_coin,
            ask_denom,
        }
    }

    // create swap send msg
    pub fn create_swap_send_msg(to_address: String, offer_coin: Coin, ask_denom: String) -> Self {
        MintcashMsg::SwapSend {
            to_address,
            offer_coin,
            ask_denom,
        }
    }
}

impl From<MintcashMsg> for CosmosMsg<MintcashMsg> {
    fn from(msg: MintcashMsg) -> CosmosMsg<MintcashMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for MintcashMsg {}
