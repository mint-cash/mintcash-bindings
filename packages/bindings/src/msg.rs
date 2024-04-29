use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Uint128};

/// A number of Custom messages that can call into the Mint Cash bindings
#[cw_serde]
pub enum MintcashMsg {
    // swap
    Swap {
        offer_coin: Coin,
        ask_denom: String,
    },
    // swap send
    SwapSend {
        to_address: String,
        offer_coin: Coin,
        ask_denom: String,
    },

    /// From https://github.com/osmosis-labs/bindings/blob/531cf7bd58d90ce4681de84556cc9bda428aec68/packages/bindings/src/msg.rs
    ///
    /// CreateDenom creates a new factory denom, of denomination:
    /// {Denom}
    /// Denom can be of length at most 44 characters, in [0-9a-zA-Z./]
    /// Empty denoms are not valid in Mintcash contract.
    /// The (creating contract address, denom) pair must be unique.
    /// The created denom's admin is the creating contract address,
    /// but this admin can be changed using the UpdateAdmin binding.
    CreateDenom { denom: String },
    /// ChangeAdmin changes the admin for a factory denom.
    /// Can only be called by the current contract admin.
    /// If the NewAdminAddress is empty, the denom will have no admin.
    ChangeAdmin {
        denom: String,
        new_admin_address: String,
    },
    /// Contracts can mint native tokens for an existing factory denom
    /// that they are the admin of.
    MintTokens {
        denom: String,
        amount: Uint128,
        mint_to_address: String,
    },
    /// Contracts can burn native tokens for an existing factory denom
    /// that they are the admin of.
    /// Currently, the burn from address must be the admin contract.
    BurnTokens {
        denom: String,
        amount: Uint128,
        burn_from_address: String,
    },
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
