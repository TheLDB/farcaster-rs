use ethers::{
    providers::Middleware,
    types::{Address, Filter, Log, H256}
};

use crate::Farcaster;

impl Farcaster {
    /// ## Get all of the ID Registry Logs from the FC V2 smart contracts on Goerli
    /// 
    /// ## Arguments
    /// 
    /// * `&self`: &Farcaster
    ///     - Farcaster is the struct type created with ::new({client});
    /// 
    /// ## Return Type
    /// * ``-> Result<Vec<Log>, Box<dyn std::error::Error>>``
    ///     - Success: Vec<Log>
    ///         - A Log is a type defined in ethers::core::types::Log
    ///             - It holds all the info of that specific event, and can be parsed with the parse_log function for the inner args
    ///     - Error: Box<dyn std::error::Error>
    ///         - A Boxed error, most commonly a string slice throughout this crate
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("client".to_string());
    /// 
    /// let logs = farcaster.get_name_registry_logs().await.unwrap();
    /// 
    /// for log in logs {
    ///     println!("{:#?}", log);
    /// }
    /// ```
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

        let mut id_logs = vec![];

        id_logs.extend(self.provider.get_logs(&register_filter).await?);
        id_logs.extend(self.provider.get_logs(&transfer_filter).await?);

        Ok(id_logs)
    }
}
