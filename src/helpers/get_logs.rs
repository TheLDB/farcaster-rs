use ethers::{
    providers::{Middleware, ProviderError},
    types::{Address, Filter, H256}, utils::{parse_bytes32_string, hex::encode},
};

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
            let topic = event.topics[3];
            let topic = encode("44065893158610579695442735846418141283404106108805717173113286591224259543040");
            let aaaa = topic.as_bytes();
            let ree: &[u8;32] = &aaaa[0..32].try_into().unwrap();
            let test = parse_bytes32_string(ree).unwrap();
            println!("{:?}", test);
        }

        Ok(())
    }
}
