use ethers::{
    providers::Middleware,
    types::{Address, Filter, Log, H256},
};

use crate::Farcaster;

impl Farcaster {
    /// ## Get all of the Name Registry Logs from the FC V2 smart contracts on Goerli
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

        Ok(logs)
    }
}
