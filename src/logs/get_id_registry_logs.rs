use ethers::{
    providers::Middleware,
    types::{Address, Filter, Log, H256}
};

use crate::Farcaster;

impl Farcaster {
    pub async fn get_id_registry_logs(&self) -> Result<Vec<Log>, Box<dyn std::error::Error>> {
        let id_registry = "0xda107a1caf36d198b12c16c7b6a1d1c795978c42"
            .parse::<Address>()?;
            
        let register_topic = "0x3cd6a0ffcc37406d9958e09bba79ff19d8237819eb2e1911f9edbce656499c87"
            .parse::<H256>()?;

        let transfer_topic = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            .parse::<H256>()?;

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
