use std::convert::TryFrom;
use std::str::FromStr;

use cosmwasm_std::{
    Coin, ContractInfoResponse, Decimal, QuerierWrapper, QueryRequest, StdResult, Uint128,
};
use mintcash_rust::types::mintcash::{
    market::v1beta1::QuerySwapRequest,
    oracle::v1beta1::QueryExchangeRateRequest,
    treasury::v1beta1::{QueryTaxCapRequest, QueryTaxRateRequest},
    tokenfactory::v1beta1::{QueryDenomAuthorityMetadataRequest, QueryDenomsFromCreatorRequest}
};
use crate::{
    query::{MintcashQuery, ExchangeRatesResponse, SwapResponse, TaxCapResponse, TaxRateResponse, DenomAuthorityMetadataResponse, DenomsFromCreatorResponse},
    ExchangeRateItem,
};

/// This is a helper wrapper to easily use our custom queries
pub struct MintcashQuerier<'a> {
    querier: &'a QuerierWrapper<'a, MintcashQuery>,
}

impl<'a> MintcashQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<MintcashQuery>) -> Self {
        MintcashQuerier { querier }
    }

    pub fn query_swap<T: Into<String>>(
        &self,
        offer_coin: Coin,
        ask_denom: T,
    ) -> StdResult<SwapResponse> {
        let request = MintcashQuery::Swap {
            offer_coin,
            ask_denom: ask_denom.into(),
        };

        let request: QueryRequest<MintcashQuery> = MintcashQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_tax_cap<T: Into<String>>(&self, denom: T) -> StdResult<TaxCapResponse> {
        let request = MintcashQuery::TaxCap {
            denom: denom.into(),
        };

        let request: QueryRequest<MintcashQuery> = MintcashQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_tax_rate(&self) -> StdResult<TaxRateResponse> {
        let request = MintcashQuery::TaxRate {};

        let request: QueryRequest<MintcashQuery> = MintcashQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_exchange_rates<T: Into<String>>(
        &self,
        base_denom: T,
        quote_denoms: Vec<T>,
    ) -> StdResult<ExchangeRatesResponse> {
        let request = MintcashQuery::ExchangeRates {
            base_denom: base_denom.into(),
            quote_denoms: quote_denoms.into_iter().map(|x| x.into()).collect(),
        };

        let request: QueryRequest<MintcashQuery> = MintcashQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_denoms_from_creator<T: Into<String>>(
        &self,
        creator: T,
    ) -> StdResult<DenomsFromCreatorResponse> {
        let request = MintcashQuery::DenomsFromCreator {
            creator: creator.into(),
        };

        let request: QueryRequest<MintcashQuery> = MintcashQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_denom_authority_metadata<T: Into<String>>(
        &self,
        denom: T,
    ) -> StdResult<DenomAuthorityMetadataResponse> {
        let request = MintcashQuery::DenomAuthorityMetadata {
            denom: denom.into(),
        };

        let request: QueryRequest<MintcashQuery> = MintcashQuery::into(request);
        self.querier.query(&request)
    }

    pub fn query_contract_info<T: Into<String>>(
        &self,
        contract_address: T,
    ) -> StdResult<ContractInfoResponse> {
        self.querier
            .query_wasm_contract_info(contract_address.into())
    }
}

#[cfg(feature = "stargate")]
/// This is a helper wrapper to easily use our custom queries through stargate query
pub struct MintcashStargateQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

#[cfg(feature = "stargate")]
impl<'a> MintcashStargateQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper) -> Self {
        MintcashStargateQuerier { querier }
    }

    pub fn query_swap<T: Into<String>>(
        &self,
        offer_coin: Coin,
        ask_denom: T,
    ) -> StdResult<SwapResponse> {
        let response = QuerySwapRequest {
            offer_coin: offer_coin.to_string(),
            ask_denom: ask_denom.into(),
        }
        .query(self.querier)
        .unwrap();

        Ok(SwapResponse {
            receive: Coin::try_from(response.return_coin.unwrap()).unwrap(),
        })
    }

    pub fn query_tax_cap<T: Into<String>>(&self, denom: T) -> StdResult<TaxCapResponse> {
        let response = QueryTaxCapRequest {
            denom: denom.into(),
        }
        .query(self.querier)
        .unwrap();

        Ok(TaxCapResponse {
            cap: Uint128::from_str(&response.tax_cap).unwrap(),
        })
    }

    pub fn query_tax_rate(&self) -> StdResult<TaxRateResponse> {
        let response = QueryTaxRateRequest {}.query(self.querier).unwrap();

        Ok(TaxRateResponse {
            rate: Decimal::from_str(&response.tax_rate).unwrap(),
        })
    }

    pub fn query_exchange_rates<T: Into<String>>(
        &self,
        base_denom: T,
        quote_denoms: Vec<T>,
    ) -> StdResult<ExchangeRatesResponse> {
        let base_denom_str: String = base_denom.into();

        // LUNA / BASE_DENOM
        let base_denom_rate = Decimal::from_str(
            &QueryExchangeRateRequest {
                denom: base_denom_str.clone(),
            }
            .query(self.querier)
            .unwrap()
            .exchange_rate,
        )
        .unwrap();

        let exchange_rates = quote_denoms
            .into_iter()
            .map(|quote_denom| {
                let quote_denom_str: String = quote_denom.into();

                // LUNA / QUOTE_DENOM
                let quote_denom_rate = Decimal::from_str(
                    &QueryExchangeRateRequest {
                        denom: quote_denom_str.clone(),
                    }
                    .query(self.querier)
                    .unwrap()
                    .exchange_rate,
                )
                .unwrap();

                ExchangeRateItem {
                    quote_denom: quote_denom_str,
                    exchange_rate: quote_denom_rate.checked_div(base_denom_rate).unwrap(),
                }
            })
            .collect();

        Ok(ExchangeRatesResponse {
            base_denom: base_denom_str,
            exchange_rates,
        })
    }

    pub fn query_contract_info<T: Into<String>>(
        &self,
        contract_address: T,
    ) -> StdResult<ContractInfoResponse> {
        self.querier
            .query_wasm_contract_info(contract_address.into())
    }

    pub fn query_denoms_from_creator<T: Into<String>>(
        &self,
        creator: T,
    ) -> StdResult<DenomsFromCreatorResponse> {
        let response = QueryDenomsFromCreatorRequest {
            creator: creator.into(),
        }
        .query(self.querier)
        .unwrap();

        Ok(DenomsFromCreatorResponse {
            denoms: response.denoms,
        })
    }

    pub fn query_denom_authority_metadata<T: Into<String>>(
        &self,
        denom: T,
    ) -> StdResult<DenomAuthorityMetadataResponse> {
        let response = QueryDenomAuthorityMetadataRequest {
            denom: denom.into(),
        }
        .query(self.querier)
        .unwrap();

        Ok(DenomAuthorityMetadataResponse {
            authority_metadata: response.authority_metadata,
        })
    }
}
