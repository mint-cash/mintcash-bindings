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
#[proto_message(type_url = "/terra.oracle.v1beta1.Params")]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub vote_period: u64,
    #[prost(string, tag = "2")]
    pub vote_threshold: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub reward_band: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub reward_distribution_window: u64,
    #[prost(message, repeated, tag = "5")]
    pub whitelist: ::prost::alloc::vec::Vec<Denom>,
    #[prost(string, tag = "6")]
    pub slash_fraction: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub slash_window: u64,
    #[prost(string, tag = "8")]
    pub min_valid_per_window: ::prost::alloc::string::String,
}
/// Denom - the object to hold configurations of each denom
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
#[proto_message(type_url = "/terra.oracle.v1beta1.Denom")]
pub struct Denom {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tobin_tax: ::prost::alloc::string::String,
}
/// struct for aggregate prevoting on the ExchangeRateVote.
/// The purpose of aggregate prevote is to hide vote exchange rates with hash
/// which is formatted as hex string in SHA256("{salt}:{exchange rate}{denom},...,{exchange rate}{denom}:{voter}")
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
#[proto_message(type_url = "/terra.oracle.v1beta1.AggregateExchangeRatePrevote")]
pub struct AggregateExchangeRatePrevote {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub submit_block: u64,
}
/// MsgAggregateExchangeRateVote - struct for voting on
/// the exchange rates of Luna denominated in various Terra assets.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.AggregateExchangeRateVote")]
pub struct AggregateExchangeRateVote {
    #[prost(message, repeated, tag = "1")]
    pub exchange_rate_tuples: ::prost::alloc::vec::Vec<ExchangeRateTuple>,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
}
/// ExchangeRateTuple - struct to store interpreted exchange rates data to store
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
#[proto_message(type_url = "/terra.oracle.v1beta1.ExchangeRateTuple")]
pub struct ExchangeRateTuple {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exchange_rate: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/terra.oracle.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub feeder_delegations: ::prost::alloc::vec::Vec<FeederDelegation>,
    #[prost(message, repeated, tag = "3")]
    pub exchange_rates: ::prost::alloc::vec::Vec<ExchangeRateTuple>,
    #[prost(message, repeated, tag = "4")]
    pub miss_counters: ::prost::alloc::vec::Vec<MissCounter>,
    #[prost(message, repeated, tag = "5")]
    pub aggregate_exchange_rate_prevotes: ::prost::alloc::vec::Vec<AggregateExchangeRatePrevote>,
    #[prost(message, repeated, tag = "6")]
    pub aggregate_exchange_rate_votes: ::prost::alloc::vec::Vec<AggregateExchangeRateVote>,
    #[prost(message, repeated, tag = "7")]
    pub tobin_taxes: ::prost::alloc::vec::Vec<TobinTax>,
}
/// FeederDelegation is the address for where oracle feeder authority are
/// delegated to. By default this struct is only used at genesis to feed in
/// default feeder addresses.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.FeederDelegation")]
pub struct FeederDelegation {
    #[prost(string, tag = "1")]
    pub feeder_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MissCounter defines an miss counter and validator address pair used in
/// oracle module's genesis state
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
#[proto_message(type_url = "/terra.oracle.v1beta1.MissCounter")]
pub struct MissCounter {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub miss_counter: u64,
}
/// TobinTax defines an denom and tobin_tax pair used in
/// oracle module's genesis state
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
#[proto_message(type_url = "/terra.oracle.v1beta1.TobinTax")]
pub struct TobinTax {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tobin_tax: ::prost::alloc::string::String,
}
/// QueryExchangeRateRequest is the request type for the Query/ExchangeRate RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryExchangeRateRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/ExchangeRate",
    response_type = QueryExchangeRateResponse
)]
pub struct QueryExchangeRateRequest {
    /// denom defines the denomination to query for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryExchangeRateResponse is response type for the
/// Query/ExchangeRate RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryExchangeRateResponse")]
pub struct QueryExchangeRateResponse {
    /// exchange_rate defines the exchange rate of Luna denominated in various Terra
    #[prost(string, tag = "1")]
    pub exchange_rate: ::prost::alloc::string::String,
}
/// QueryExchangeRatesRequest is the request type for the Query/ExchangeRates RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryExchangeRatesRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/ExchangeRates",
    response_type = QueryExchangeRatesResponse
)]
pub struct QueryExchangeRatesRequest {}
/// QueryExchangeRatesResponse is response type for the
/// Query/ExchangeRates RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryExchangeRatesResponse")]
pub struct QueryExchangeRatesResponse {
    /// exchange_rates defines a list of the exchange rate for all whitelisted denoms.
    #[prost(message, repeated, tag = "1")]
    pub exchange_rates:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// QueryTobinTaxRequest is the request type for the Query/TobinTax RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryTobinTaxRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/TobinTax",
    response_type = QueryTobinTaxResponse
)]
pub struct QueryTobinTaxRequest {
    /// denom defines the denomination to query for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryTobinTaxResponse is response type for the
/// Query/TobinTax RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryTobinTaxResponse")]
pub struct QueryTobinTaxResponse {
    /// tobin_taxe defines the tobin tax of a denom
    #[prost(string, tag = "1")]
    pub tobin_tax: ::prost::alloc::string::String,
}
/// QueryTobinTaxesRequest is the request type for the Query/TobinTaxes RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryTobinTaxesRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/TobinTaxes",
    response_type = QueryTobinTaxesResponse
)]
pub struct QueryTobinTaxesRequest {}
/// QueryTobinTaxesResponse is response type for the
/// Query/TobinTaxes RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryTobinTaxesResponse")]
pub struct QueryTobinTaxesResponse {
    /// tobin_taxes defines a list of the tobin tax of all whitelisted denoms
    #[prost(message, repeated, tag = "1")]
    pub tobin_taxes: ::prost::alloc::vec::Vec<Denom>,
}
/// QueryActivesRequest is the request type for the Query/Actives RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryActivesRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/Actives",
    response_type = QueryActivesResponse
)]
pub struct QueryActivesRequest {}
/// QueryActivesResponse is response type for the
/// Query/Actives RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryActivesResponse")]
pub struct QueryActivesResponse {
    /// actives defines a list of the denomination which oracle prices aggreed upon.
    #[prost(string, repeated, tag = "1")]
    pub actives: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryVoteTargetsRequest is the request type for the Query/VoteTargets RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryVoteTargetsRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/VoteTargets",
    response_type = QueryVoteTargetsResponse
)]
pub struct QueryVoteTargetsRequest {}
/// QueryVoteTargetsResponse is response type for the
/// Query/VoteTargets RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryVoteTargetsResponse")]
pub struct QueryVoteTargetsResponse {
    /// vote_targets defines a list of the denomination in which everyone
    /// should vote in the current vote period.
    #[prost(string, repeated, tag = "1")]
    pub vote_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryFeederDelegationRequest is the request type for the Query/FeederDelegation RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryFeederDelegationRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/FeederDelegation",
    response_type = QueryFeederDelegationResponse
)]
pub struct QueryFeederDelegationRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryFeederDelegationResponse is response type for the
/// Query/FeederDelegation RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryFeederDelegationResponse")]
pub struct QueryFeederDelegationResponse {
    /// feeder_addr defines the feeder delegation of a validator
    #[prost(string, tag = "1")]
    pub feeder_addr: ::prost::alloc::string::String,
}
/// QueryMissCounterRequest is the request type for the Query/MissCounter RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryMissCounterRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/MissCounter",
    response_type = QueryMissCounterResponse
)]
pub struct QueryMissCounterRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryMissCounterResponse is response type for the
/// Query/MissCounter RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryMissCounterResponse")]
pub struct QueryMissCounterResponse {
    /// miss_counter defines the oracle miss counter of a validator
    #[prost(uint64, tag = "1")]
    pub miss_counter: u64,
}
/// QueryAggregatePrevoteRequest is the request type for the Query/AggregatePrevote RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryAggregatePrevoteRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/AggregatePrevote",
    response_type = QueryAggregatePrevoteResponse
)]
pub struct QueryAggregatePrevoteRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryAggregatePrevoteResponse is response type for the
/// Query/AggregatePrevote RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryAggregatePrevoteResponse")]
pub struct QueryAggregatePrevoteResponse {
    /// aggregate_prevote defines oracle aggregate prevote submitted by a validator in the current vote period
    #[prost(message, optional, tag = "1")]
    pub aggregate_prevote: ::core::option::Option<AggregateExchangeRatePrevote>,
}
/// QueryAggregatePrevotesRequest is the request type for the Query/AggregatePrevotes RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryAggregatePrevotesRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/AggregatePrevotes",
    response_type = QueryAggregatePrevotesResponse
)]
pub struct QueryAggregatePrevotesRequest {}
/// QueryAggregatePrevotesResponse is response type for the
/// Query/AggregatePrevotes RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryAggregatePrevotesResponse")]
pub struct QueryAggregatePrevotesResponse {
    /// aggregate_prevotes defines all oracle aggregate prevotes submitted in the current vote period
    #[prost(message, repeated, tag = "1")]
    pub aggregate_prevotes: ::prost::alloc::vec::Vec<AggregateExchangeRatePrevote>,
}
/// QueryAggregateVoteRequest is the request type for the Query/AggregateVote RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryAggregateVoteRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/AggregateVote",
    response_type = QueryAggregateVoteResponse
)]
pub struct QueryAggregateVoteRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryAggregateVoteResponse is response type for the
/// Query/AggregateVote RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryAggregateVoteResponse")]
pub struct QueryAggregateVoteResponse {
    /// aggregate_vote defines oracle aggregate vote submitted by a validator in the current vote period
    #[prost(message, optional, tag = "1")]
    pub aggregate_vote: ::core::option::Option<AggregateExchangeRateVote>,
}
/// QueryAggregateVotesRequest is the request type for the Query/AggregateVotes RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryAggregateVotesRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/AggregateVotes",
    response_type = QueryAggregateVotesResponse
)]
pub struct QueryAggregateVotesRequest {}
/// QueryAggregateVotesResponse is response type for the
/// Query/AggregateVotes RPC method.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryAggregateVotesResponse")]
pub struct QueryAggregateVotesResponse {
    /// aggregate_votes defines all oracle aggregate votes submitted in the current vote period
    #[prost(message, repeated, tag = "1")]
    pub aggregate_votes: ::prost::alloc::vec::Vec<AggregateExchangeRateVote>,
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/terra.oracle.v1beta1.Query/Params",
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
#[proto_message(type_url = "/terra.oracle.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgAggregateExchangeRatePrevote represents a message to submit
/// aggregate exchange rate prevote.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.MsgAggregateExchangeRatePrevote")]
pub struct MsgAggregateExchangeRatePrevote {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub feeder: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator: ::prost::alloc::string::String,
}
/// MsgAggregateExchangeRatePrevoteResponse defines the Msg/AggregateExchangeRatePrevote response type.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.MsgAggregateExchangeRatePrevoteResponse")]
pub struct MsgAggregateExchangeRatePrevoteResponse {}
/// MsgAggregateExchangeRateVote represents a message to submit
/// aggregate exchange rate vote.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.MsgAggregateExchangeRateVote")]
pub struct MsgAggregateExchangeRateVote {
    #[prost(string, tag = "1")]
    pub salt: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exchange_rates: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub feeder: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub validator: ::prost::alloc::string::String,
}
/// MsgAggregateExchangeRateVoteResponse defines the Msg/AggregateExchangeRateVote response type.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.MsgAggregateExchangeRateVoteResponse")]
pub struct MsgAggregateExchangeRateVoteResponse {}
/// MsgDelegateFeedConsent represents a message to
/// delegate oracle voting rights to another address.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.MsgDelegateFeedConsent")]
pub struct MsgDelegateFeedConsent {
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegate: ::prost::alloc::string::String,
}
/// MsgDelegateFeedConsentResponse defines the Msg/DelegateFeedConsent response type.
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
#[proto_message(type_url = "/terra.oracle.v1beta1.MsgDelegateFeedConsentResponse")]
pub struct MsgDelegateFeedConsentResponse {}
pub struct OracleQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> OracleQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn exchange_rate(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryExchangeRateResponse, cosmwasm_std::StdError> {
        QueryExchangeRateRequest { denom }.query(self.querier)
    }
    pub fn exchange_rates(&self) -> Result<QueryExchangeRatesResponse, cosmwasm_std::StdError> {
        QueryExchangeRatesRequest {}.query(self.querier)
    }
    pub fn tobin_tax(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryTobinTaxResponse, cosmwasm_std::StdError> {
        QueryTobinTaxRequest { denom }.query(self.querier)
    }
    pub fn tobin_taxes(&self) -> Result<QueryTobinTaxesResponse, cosmwasm_std::StdError> {
        QueryTobinTaxesRequest {}.query(self.querier)
    }
    pub fn actives(&self) -> Result<QueryActivesResponse, cosmwasm_std::StdError> {
        QueryActivesRequest {}.query(self.querier)
    }
    pub fn vote_targets(&self) -> Result<QueryVoteTargetsResponse, cosmwasm_std::StdError> {
        QueryVoteTargetsRequest {}.query(self.querier)
    }
    pub fn feeder_delegation(
        &self,
        validator_addr: ::prost::alloc::string::String,
    ) -> Result<QueryFeederDelegationResponse, cosmwasm_std::StdError> {
        QueryFeederDelegationRequest { validator_addr }.query(self.querier)
    }
    pub fn miss_counter(
        &self,
        validator_addr: ::prost::alloc::string::String,
    ) -> Result<QueryMissCounterResponse, cosmwasm_std::StdError> {
        QueryMissCounterRequest { validator_addr }.query(self.querier)
    }
    pub fn aggregate_prevote(
        &self,
        validator_addr: ::prost::alloc::string::String,
    ) -> Result<QueryAggregatePrevoteResponse, cosmwasm_std::StdError> {
        QueryAggregatePrevoteRequest { validator_addr }.query(self.querier)
    }
    pub fn aggregate_prevotes(
        &self,
    ) -> Result<QueryAggregatePrevotesResponse, cosmwasm_std::StdError> {
        QueryAggregatePrevotesRequest {}.query(self.querier)
    }
    pub fn aggregate_vote(
        &self,
        validator_addr: ::prost::alloc::string::String,
    ) -> Result<QueryAggregateVoteResponse, cosmwasm_std::StdError> {
        QueryAggregateVoteRequest { validator_addr }.query(self.querier)
    }
    pub fn aggregate_votes(&self) -> Result<QueryAggregateVotesResponse, cosmwasm_std::StdError> {
        QueryAggregateVotesRequest {}.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
