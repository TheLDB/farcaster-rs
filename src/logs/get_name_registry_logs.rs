use ethers::{
    abi::{AbiEncode, RawLog, Tokenizable},
    providers::{Middleware, ProviderError},
    types::{Address, Filter, Log, H256, U256},
    utils::parse_bytes32_string,
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
    /// ### Used to get all V2 Name Registry Logs from the smart contract on Goerli
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
    pub async fn get_name_registry_logs(self) -> Result<Vec<NameRegistry>, ProviderError> {
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

        let mut name_registry_vec: Vec<NameRegistry> = vec![];
        for event in logs {
            let raw_log_clone = event.clone();
            let raw_log = RawLog {
                topics: raw_log_clone.topics,
                data: raw_log_clone.data.to_vec(),
            };

            let log_desc = self.abi.event("Transfer").unwrap().parse_log(raw_log);
            match log_desc {
                Ok(success) => {
                    for i in &success.params {
                        if i.name == "tokenId" {

                            let u256_token = U256::from_token(i.value.clone()).unwrap();
                            let encoded_u256_token = u256_token.encode();
                            let byte_u256_token: &[u8] = &encoded_u256_token;
                            let byte_u256_token: &[u8; 32] =
                                byte_u256_token[0..32].try_into().unwrap();
                            let fname = parse_bytes32_string(byte_u256_token).unwrap();
                            let new_name_registry = NameRegistry {
                                event: event.clone(),
                                log_desc: success.clone(),
                                fname: fname.to_string(),
                            };

                            name_registry_vec.push(new_name_registry);
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        }

        Ok(name_registry_vec)
    }
}
