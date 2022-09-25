use ethers::{
    providers::{Middleware, ProviderError},
    types::{Address, Filter, H256}, utils::{parse_bytes32_string, hex::encode}, abi::{RawLog, Token::Uint, Tokenizable, AbiEncode},
};
use ethers::types::U256;

use crate::Farcaster;

impl Farcaster {
    pub async fn get_logs(self) -> Result<(), ProviderError> {
        let name_registry = "0x4b1db9d8fcb29f3b1c33942b27ad4cbbb0806f9f"
            .parse::<Address>()
            .unwrap();
        let transfer_topic = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".parse::<H256>().unwrap();
        let filter = Filter::new().select(1337u64..).address(name_registry).topic0(transfer_topic);
        let test = self.provider.get_logs(&filter).await?;
        println!("{}", test.len());
        for event in test {
            let raw_log = RawLog {
                topics: event.topics,
                data: event.data.to_vec()
            };

            let log_desc = self.abi.event("Transfer").unwrap().parse_log(raw_log);
            match log_desc {
                Ok(success) => {
                    for i in success.params {
                        if i.name == "tokenId" {
                            let ree = U256::from_token(i.value).unwrap();
                            let aa = ree.encode();
                            let aaa: &[u8] = &aa;
                            let ree: &[u8;32] = aaa[0..32].try_into().unwrap();
                            let test = parse_bytes32_string(ree).unwrap();
                            println!("{}", test);
                            // let value = U256::from(i.value.to_string().as_str());
                            // println!("Token: {:?}", i.value);
                            // println!("String: {}", i.value.to_string())
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        }

        Ok(())
    }
}
