use ethers::{
    providers::{Middleware, ProviderError},
    types::{Address, Filter, Log, H256},
};

use crate::{Farcaster};

/// ### NameRegistry Struct
/// Used to return info for the ``get_nme_registry_logs()`` function
///
/// ## Example
/// ```no_run
/// let new_name_registry = NameRegistry {
///     event: Some Log Here,
///     log_desc: Some Parsed Log here,
///     fname: A string
/// }
/// ```
#[derive(Debug)]
pub struct NameRegistry {
    pub event: Log,
    pub log_desc: ethers::abi::Log,
    pub fname: String,
}

impl Farcaster {
    pub async fn get_name_registry_logs(&self) -> Result<Vec<Log>, ProviderError> {
        let name_registry = "0xe3be01d99baa8db9905b33a3ca391238234b79d1"
            .parse::<Address>()
            .unwrap();

        let transfer_topic = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            .parse::<H256>()
            .unwrap();

        let filter = Filter::new()
            .select(1337u64..)
            .address(name_registry)
            .topic0(transfer_topic);

        let logs = self.provider.get_logs(&filter).await?;

        let mut log_vec: Vec<Log> = vec![];

        for event in logs {
            log_vec.push(event);
        }

        Ok(log_vec)
    }
}
