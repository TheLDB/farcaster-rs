
use ethers::{types::Log, abi::RawLog};

use crate::{Farcaster, types::abi::registry::Registry};

use crate::types::logs::events::Events;

impl Farcaster {
    /// ## Parse an Ethers log to get the inner arguments for the event
    /// 
    /// ## Arguments
    /// 
    /// * `&self`: &Farcaster
    ///     - Farcaster is the struct type created with ::new({client});
    /// 
    /// * `log`: Log
    ///     - The Log to parse
    /// 
    /// * `abi`: Registry
    ///     - Registry is a custom type defined in types::abi::Registry
    ///         - Registry represents the registry you got the log from, either ``Registry::ID`` or ``Registry::NAME``
    /// 
    /// * `event`: Events
    ///     - Events is a custom type defined in types::logs::Events
    ///         - Events represents the type of the event the log is, either a Register or a Transfer. Can be ``Events::Register`` or ``Events::Transfer``
    /// 
    /// ## Return Type
    /// 
    /// * ``-> Result<ethers::abi::Log, Box<dyn std::error::Error>>``
    /// 
    ///     - Success: ethers::abi::Log
    ///         - It's like a ethers::core::types::Log, but represents the inner arguments of the event instead
    ///             - It holds the from, to, and tokenId in this scenario
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("client".to_string());
    /// 
    /// let logs = farcaster.get_id_registry_logs().await.unwrap();
    /// 
    /// let log = logs.get(0).unwrap();
    /// 
    /// let parsed_log = farcaster.parse_log(log, Registry::ID, Events::Register).await.unwrap();
    /// 
    /// println!("{:#?}", parsed_log);
    /// ```
    pub async fn parse_log(&self, log: Log, abi: Registry, event: Events) -> Result<ethers::abi::Log, Box<dyn std::error::Error>> {
        match abi {
            Registry::ID => {
                let raw_log = RawLog {
                    topics: log.topics,
                    data: log.data.to_vec()
                };

                let log_desc = self.id_registry_abi.event(event.to_string().as_str())?.parse_log(raw_log)?;
                Ok(log_desc)
            }
            Registry::NAME => {
                let raw_log = RawLog {
                    topics: log.topics,
                    data: log.data.to_vec()
                };

                let log_desc = self.name_registry_abi.event(event.to_string().as_str())?.parse_log(raw_log)?;
                Ok(log_desc)
            }
        }
    }
}