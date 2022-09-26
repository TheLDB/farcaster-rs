use ethers::{
    providers::{Middleware, ProviderError},
    types::{Address, Filter, Log, H256}
};

use crate::Farcaster;

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
    /// ### Used to get all V2 ID Registry Logs from the smart contract on Goerli
    /// Smart Contract Address: ``0x4b1db9d8fcb29f3b1c33942b27ad4cbbb0806f9f``
    ///
    /// ## Example
    /// ```no_run
    /// fn name_registry_example() -> Result<(), ProviderError> {
    ///     let farcaster = Farcaster::new("Infura Connection String.to_string()");
    ///     let name_registry = farcaster.get_name_registry_logs.await?;
    /// }
    /// ```
    ///
    /// ## Return Type
    /// ``Vec<NameRegistry>``
    ///
    /// Name Registry Struct:
    ///
    /// ```no_run
    /// pub struct NameRegistry {
    ///     pub event: ethers::core::types::Log,
    ///     pub log_desc: ethers::abi::Log,
    ///     pub fname: String
    /// }
    /// ```
    pub async fn get_id_registry_logs(self) -> Result<Vec<Log>, ProviderError> {
        let id_registry = "0xda107a1caf36d198b12c16c7b6a1d1c795978c42"
            .parse::<Address>()
            .unwrap();
            
        let register_topic = "0x3cd6a0ffcc37406d9958e09bba79ff19d8237819eb2e1911f9edbce656499c87"
            .parse::<H256>()
            .unwrap();

        let transfer_topic = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            .parse::<H256>()
            .unwrap();

        let register_filter = Filter::new()
            .select(1337u64..)
            .address(id_registry)
            .topic0(register_topic);

        let transfer_filter = Filter::new()
            .select(1337u64..)
            .address(id_registry)
            .topic0(transfer_topic);

        let register_logs = self.provider.get_logs(&register_filter).await?;
        let transfer_logs = self.provider.get_logs(&transfer_filter).await?;

        let mut id_logs: Vec<Log> = vec![];
        
        for log in register_logs {
            id_logs.push(log);
        }

        for log in transfer_logs {
            id_logs.push(log);
        }

        Ok(id_logs)
    }
}
