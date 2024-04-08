use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, CustomQuery, Decimal, Uint128};
use mintcash_rust::types::mintcash::tokenfactory::v1beta1::DenomAuthorityMetadata;

impl CustomQuery for MintcashQuery {}

// MintcashQuery for custom bindings
#[cw_serde]
#[derive(QueryResponses)]
pub enum MintcashQuery {
    #[returns(SwapResponse)]
    Swap { offer_coin: Coin, ask_denom: String },

    #[returns(TaxRateResponse)]
    TaxRate {},

    #[returns(TaxCapResponse)]
    TaxCap { denom: String },

    #[returns(ExchangeRatesResponse)]
    ExchangeRates {
        base_denom: String,
        quote_denoms: Vec<String>,
    },

    #[returns(DenomsFromCreatorResponse)]
    DenomsFromCreator { creator: String },

    #[returns(DenomAuthorityMetadataResponse)]
    DenomAuthorityMetadata { denom: String },
}

impl MintcashQuery {
    pub fn swap(offer_coin: Coin, ask_denom: String) -> Self {
        MintcashQuery::Swap {
            offer_coin,
            ask_denom,
        }
    }

    pub fn tax_rate() -> Self {
        MintcashQuery::TaxRate {}
    }

    pub fn tax_cap(denom: String) -> Self {
        MintcashQuery::TaxCap { denom }
    }

    pub fn exchange_rates(base_denom: String, quote_denoms: Vec<String>) -> Self {
        MintcashQuery::ExchangeRates {
            base_denom,
            quote_denoms,
        }
    }

    pub fn denoms_from_creator(creator: String) -> Self {
        MintcashQuery::DenomsFromCreator { creator }
    }

    pub fn denom_authority_metadata(denom: String) -> Self {
        MintcashQuery::DenomAuthorityMetadata { denom }
    }
}

/// SwapResponse is data format returned from SwapRequest::Simulate query
#[cw_serde]
pub struct SwapResponse {
    pub receive: Coin,
}

/// TaxRateResponse is data format returned from TreasuryRequest::TaxRate query
#[cw_serde]
pub struct TaxRateResponse {
    pub rate: Decimal,
}

/// TaxCapResponse is data format returned from TreasuryRequest::TaxCap query
#[cw_serde]
pub struct TaxCapResponse {
    pub cap: Uint128,
}

/// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[cw_serde]
pub struct ExchangeRateItem {
    pub quote_denom: String,
    pub exchange_rate: Decimal,
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[cw_serde]
pub struct ExchangeRatesResponse {
    pub base_denom: String,
    pub exchange_rates: Vec<ExchangeRateItem>,
}

#[cw_serde]
pub struct DenomsFromCreatorResponse {
    pub denoms: Vec<String>,
}

#[cw_serde]
pub struct DenomAuthorityMetadataResponse {
    pub authority_metadata: Option<DenomAuthorityMetadata>
}
