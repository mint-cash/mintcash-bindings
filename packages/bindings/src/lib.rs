mod msg;
mod query;
mod querier;

pub use msg::MintcashMsg;
pub use querier::MintcashQuerier;
#[cfg(feature = "stargate")]
pub use querier::MintcashStargateQuerier;
pub use query::{
    ExchangeRateItem, ExchangeRatesResponse, SwapResponse, TaxCapResponse,
    TaxRateResponse, MintcashQuery
};

// This export is added to all contracts that import this package, signifying that they require
// "mintcash" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_mintcash() {}