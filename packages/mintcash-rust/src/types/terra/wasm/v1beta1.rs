use osmosis_std_derive::CosmwasmExt;
/// CodeInfo is data for the uploaded contract WASM code
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
#[proto_message(type_url = "/mintcash.wasm.v1beta1.CodeInfo")]
pub struct CodeInfo {
    /// CodeID is the sequentially increasing unique identifier
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// CodeHash is the unique identifier created by wasmvm
    #[prost(bytes = "vec", tag = "2")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    /// Creator address who initially stored the code
    #[prost(string, tag = "3")]
    pub creator: ::prost::alloc::string::String,
}
/// ContractInfo stores a WASM contract instance
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
#[proto_message(type_url = "/mintcash.wasm.v1beta1.ContractInfo")]
pub struct ContractInfo {
    /// Address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Creator is the contract creator address
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    /// Admin is who can execute the contract migration
    #[prost(string, tag = "3")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored Wasm code
    #[prost(uint64, tag = "4")]
    pub code_id: u64,
    /// InitMsg is the raw message used when instantiating a contract
    #[prost(bytes = "vec", tag = "5")]
    pub init_msg: ::prost::alloc::vec::Vec<u8>,
}
