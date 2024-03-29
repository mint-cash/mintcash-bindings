use osmosis_std_derive::CosmwasmExt;
/// Params defines the parameters for the oracle module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.Params")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub tax_policy: ::core::option::Option<PolicyConstraints>,
    #[prost(message, optional, tag = "2")]
    pub reward_policy: ::core::option::Option<PolicyConstraints>,
    #[prost(string, tag = "3")]
    pub seigniorage_burden_target: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub mining_increment: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub window_short: u64,
    #[prost(uint64, tag = "6")]
    pub window_long: u64,
    #[prost(uint64, tag = "7")]
    pub window_probation: u64,
    #[prost(string, tag = "8")]
    pub burn_tax_split: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub min_initial_deposit_ratio: ::prost::alloc::string::String,
}
/// PolicyConstraints - defines policy constraints can be applied in tax & reward policies
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.PolicyConstraints")]
pub struct PolicyConstraints {
    #[prost(string, tag = "1")]
    pub rate_min: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub rate_max: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub cap: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub change_rate_max: ::prost::alloc::string::String,
}
/// EpochTaxProceeds represents the tax amount
/// collected at the current epoch
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.EpochTaxProceeds")]
pub struct EpochTaxProceeds {
    #[prost(message, repeated, tag = "1")]
    pub tax_proceeds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// EpochInitialIssuance represents initial issuance
/// of the currrent epoch
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.EpochInitialIssuance")]
pub struct EpochInitialIssuance {
    #[prost(message, repeated, tag = "1")]
    pub issuance: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the oracle module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, tag = "2")]
    pub tax_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub reward_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub tax_caps: ::prost::alloc::vec::Vec<TaxCap>,
    #[prost(message, repeated, tag = "5")]
    pub tax_proceeds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "6")]
    pub epoch_initial_issuance:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "7")]
    pub epoch_states: ::prost::alloc::vec::Vec<EpochState>,
}
/// TaxCap is the max tax amount can be charged for the given denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.TaxCap")]
pub struct TaxCap {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tax_cap: ::prost::alloc::string::String,
}
/// EpochState is the record for each epoch state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.EpochState")]
pub struct EpochState {
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    #[prost(string, tag = "2")]
    pub tax_reward: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub seigniorage_reward: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub total_staked_luna: ::prost::alloc::string::String,
}
/// proposal request structure for adding burn tax exemption address(es)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.AddBurnTaxExemptionAddressProposal")]
pub struct AddBurnTaxExemptionAddressProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// proposal request structure for removing burn tax exemption address(es)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.RemoveBurnTaxExemptionAddressProposal")]
pub struct RemoveBurnTaxExemptionAddressProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryTaxRateRequest is the request type for the Query/TaxRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryTaxRateRequest")]
#[proto_query(
    path = "/terra.treasury.v1beta1.Query/TaxRate",
    response_type = QueryTaxRateResponse
)]
pub struct QueryTaxRateRequest {}
/// QueryTaxRateResponse is response type for the
/// Query/TaxRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryTaxRateResponse")]
pub struct QueryTaxRateResponse {
    #[prost(string, tag = "1")]
    pub tax_rate: ::prost::alloc::string::String,
}
/// QueryTaxCapRequest is the request type for the Query/TaxCap RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryTaxCapRequest")]
#[proto_query(
    path = "/terra.treasury.v1beta1.Query/TaxCap",
    response_type = QueryTaxCapResponse
)]
pub struct QueryTaxCapRequest {
    /// denom defines the denomination to query for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryTaxCapResponse is response type for the
/// Query/TaxCap RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryTaxCapResponse")]
pub struct QueryTaxCapResponse {
    #[prost(string, tag = "1")]
    pub tax_cap: ::prost::alloc::string::String,
}
/// QueryTaxCapsRequest is the request type for the Query/TaxCaps RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryTaxCapsRequest")]
#[proto_query(
    path = "/terra.treasury.v1beta1.Query/TaxCaps",
    response_type = QueryTaxCapsResponse
)]
pub struct QueryTaxCapsRequest {}
/// QueryTaxCapsResponseItem is response item type for the
/// Query/TaxCaps RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryTaxCapsResponseItem")]
pub struct QueryTaxCapsResponseItem {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tax_cap: ::prost::alloc::string::String,
}
/// QueryTaxCapsResponse is response type for the
/// Query/TaxCaps RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryTaxCapsResponse")]
pub struct QueryTaxCapsResponse {
    #[prost(message, repeated, tag = "1")]
    pub tax_caps: ::prost::alloc::vec::Vec<QueryTaxCapsResponseItem>,
}
/// QueryRewardWeightRequest is the request type for the Query/RewardWeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryRewardWeightRequest")]
#[proto_query(
    path = "/terra.treasury.v1beta1.Query/RewardWeight",
    response_type = QueryRewardWeightResponse
)]
pub struct QueryRewardWeightRequest {}
/// QueryRewardWeightResponse is response type for the
/// Query/RewardWeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryRewardWeightResponse")]
pub struct QueryRewardWeightResponse {
    #[prost(string, tag = "1")]
    pub reward_weight: ::prost::alloc::string::String,
}
/// QueryTaxProceedsRequest is the request type for the Query/TaxProceeds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryTaxProceedsRequest")]
#[proto_query(
    path = "/terra.treasury.v1beta1.Query/TaxProceeds",
    response_type = QueryTaxProceedsResponse
)]
pub struct QueryTaxProceedsRequest {}
/// QueryTaxProceedsResponse is response type for the
/// Query/TaxProceeds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryTaxProceedsResponse")]
pub struct QueryTaxProceedsResponse {
    #[prost(message, repeated, tag = "1")]
    pub tax_proceeds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QuerySeigniorageProceedsRequest is the request type for the Query/SeigniorageProceeds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QuerySeigniorageProceedsRequest")]
#[proto_query(
    path = "/terra.treasury.v1beta1.Query/SeigniorageProceeds",
    response_type = QuerySeigniorageProceedsResponse
)]
pub struct QuerySeigniorageProceedsRequest {}
/// QuerySeigniorageProceedsResponse is response type for the
/// Query/SeigniorageProceeds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QuerySeigniorageProceedsResponse")]
pub struct QuerySeigniorageProceedsResponse {
    #[prost(string, tag = "1")]
    pub seigniorage_proceeds: ::prost::alloc::string::String,
}
/// QueryIndicatorsRequest is the request type for the Query/Indicators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryIndicatorsRequest")]
#[proto_query(
    path = "/terra.treasury.v1beta1.Query/Indicators",
    response_type = QueryIndicatorsResponse
)]
pub struct QueryIndicatorsRequest {}
/// QueryIndicatorsResponse is response type for the
/// Query/Indicators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryIndicatorsResponse")]
pub struct QueryIndicatorsResponse {
    #[prost(string, tag = "1")]
    pub trl_year: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub trl_month: ::prost::alloc::string::String,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/terra.treasury.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryBurnTaxExemptionListRequest is the request type for the Query/BurnTaxExemptionList RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryBurnTaxExemptionListRequest")]
#[proto_query(
    path = "/terra.treasury.v1beta1.Query/BurnTaxExemptionList",
    response_type = QueryBurnTaxExemptionListResponse
)]
pub struct QueryBurnTaxExemptionListRequest {
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryBurnTaxExemptionListResponse is response type for the Query/BurnTaxExemptionList RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/terra.treasury.v1beta1.QueryBurnTaxExemptionListResponse")]
pub struct QueryBurnTaxExemptionListResponse {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
pub struct TreasuryQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TreasuryQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn tax_rate(&self) -> Result<QueryTaxRateResponse, cosmwasm_std::StdError> {
        QueryTaxRateRequest {}.query(self.querier)
    }
    pub fn tax_cap(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryTaxCapResponse, cosmwasm_std::StdError> {
        QueryTaxCapRequest { denom }.query(self.querier)
    }
    pub fn tax_caps(&self) -> Result<QueryTaxCapsResponse, cosmwasm_std::StdError> {
        QueryTaxCapsRequest {}.query(self.querier)
    }
    pub fn reward_weight(&self) -> Result<QueryRewardWeightResponse, cosmwasm_std::StdError> {
        QueryRewardWeightRequest {}.query(self.querier)
    }
    pub fn seigniorage_proceeds(
        &self,
    ) -> Result<QuerySeigniorageProceedsResponse, cosmwasm_std::StdError> {
        QuerySeigniorageProceedsRequest {}.query(self.querier)
    }
    pub fn tax_proceeds(&self) -> Result<QueryTaxProceedsResponse, cosmwasm_std::StdError> {
        QueryTaxProceedsRequest {}.query(self.querier)
    }
    pub fn indicators(&self) -> Result<QueryIndicatorsResponse, cosmwasm_std::StdError> {
        QueryIndicatorsRequest {}.query(self.querier)
    }
    pub fn burn_tax_exemption_list(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryBurnTaxExemptionListResponse, cosmwasm_std::StdError> {
        QueryBurnTaxExemptionListRequest { pagination }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
