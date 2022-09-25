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
                            let u256_token = U256::from_token(i.value).unwrap();
                            let encoded_u256_token = u256_token.encode();
                            let byte_u256_token: &[u8] = &encoded_u256_token;
                            let byte_u256_token: &[u8;32] = byte_u256_token[0..32].try_into().unwrap();
                            let fname = parse_bytes32_string(byte_u256_token).unwrap();
                            println!("{}", fname);
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
