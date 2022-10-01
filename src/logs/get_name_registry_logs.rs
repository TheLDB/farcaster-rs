use ethers::{
    providers::Middleware,
    types::{Address, Filter, Log, H256},
};

use crate::Farcaster;

impl Farcaster {
    pub async fn get_name_registry_logs(&self) -> Result<Vec<Log>, Box<dyn std::error::Error>> {
        let name_registry = "0xe3be01d99baa8db9905b33a3ca391238234b79d1"
            .parse::<Address>()?;

        let transfer_topic = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            .parse::<H256>()?;

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
