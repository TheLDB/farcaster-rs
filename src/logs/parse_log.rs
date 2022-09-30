
use ethers::{types::Log, abi::RawLog};

use crate::{Farcaster, types::abi::registry::Registry};

use crate::types::logs::events::Events;

impl Farcaster {
    pub async fn parse_log(&self, log: Log, abi: Registry, event: Events) -> Result<ethers::abi::Log, Box<dyn std::error::Error>> {
        match abi {
            Registry::ID => {
                let raw_log = RawLog {
                    topics: log.topics,
                    data: log.data.to_vec()
                };

                let log_desc = self.id_registry_abi.event(event.to_string().as_str()).unwrap().parse_log(raw_log).unwrap();
                Ok(log_desc)
            }
            Registry::NAME => {
                let raw_log = RawLog {
                    topics: log.topics,
                    data: log.data.to_vec()
                };

                let log_desc = self.name_registry_abi.event(event.to_string().as_str()).unwrap().parse_log(raw_log).unwrap();
                Ok(log_desc)
            }
        }
    }
}