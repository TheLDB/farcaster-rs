use ethers::{
    abi::RawLog,
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
    pub async fn get_id_registry_logs(self) -> Result<(), ProviderError> {
        let id_registry = "0xda107a1caf36d198b12c16c7b6a1d1c795978c42"
            .parse::<Address>()
            .unwrap();
            
        let register_topic = "0x3cd6a0ffcc37406d9958e09bba79ff19d8237819eb2e1911f9edbce656499c87"
            .parse::<H256>()
            .unwrap();

        let transfer_topic = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            .parse::<H256>()
            .unwrap();

        let filter = Filter::new()
            .select(1337u64..)
            .address(id_registry)
            .topic0(register_topic);
            // .topic1(transfer_topic);

        let logs = self.provider.get_logs(&filter).await?;

        println!("{:?}", logs.len());
        for event in logs {
            println!("hi!");
            let register_log = event.clone();
            // let transfer_log = event.clone();

            let register_log = RawLog {
                topics: register_log.topics,
                data: register_log.data.to_vec()
            };

            // let transfer_log = RawLog {
            //     topics: transfer_log.topics,
            //     data: transfer_log.data.to_vec()
            // };

            let register_logs = self.id_registry_abi.event("Register").unwrap().parse_log(register_log).unwrap();
            println!("=========== Register Log ===========");
            println!("{:?}", register_logs);

            // let transfer_logs = self.abi.event("Transfer").unwrap().parse_log(transfer_log).unwrap();
            // println!("=========== Transfer Log ===========");
            // println!("{:?}", transfer_logs);
            // let log_desc = self.abi.event("Transfer").unwrap().parse_log(raw_log);
            // match log_desc {
            //     Ok(success) => {
            //         for i in &success.params {
            //             if i.name == "tokenId" {
            //                 let u256_token = U256::from_token(i.value.clone()).unwrap();
            //                 let encoded_u256_token = u256_token.encode();
            //                 let byte_u256_token: &[u8] = &encoded_u256_token;
            //                 let byte_u256_token: &[u8; 32] =
            //                     byte_u256_token[0..32].try_into().unwrap();
            //                 let fname = parse_bytes32_string(byte_u256_token).unwrap();
            //                 let new_name_registry = NameRegistry {
            //                     event: event.clone(),
            //                     log_desc: success.clone(),
            //                     fname: fname.to_string(),
            //                 };

            //                 name_registry_vec.push(new_name_registry);
            //             }
            //         }
            //     }
            //     Err(e) => {
            //         println!("{}", e)
            //     }
            // }
        }

        Ok(())
    }
}
