//! Contains all of the log related functions (get/parse)

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
    /// let logs = farcaster.get_id_registry_logs().await.unwrap();
    /// 
    /// for log in logs {
    ///     println!("{:#?}", log);
    /// }
    /// ```
pub mod get_id_registry_logs;

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
pub mod get_name_registry_logs;

/// ## Parse an Ethers log to get the inner arguments for the event
    /// 
    /// ## Arguments
    /// 
    /// * `&self`: &Farcaster
    ///     - Farcaster is the struct type created with ::new({client});
    /// 
    /// * `log`: Log
    ///     - The Log to parse
    /// 
    /// * `abi`: Registry
    ///     - Registry is a custom type defined in types::abi::Registry
    ///         - Registry represents the registry you got the log from, either ``Registry::ID`` or ``Registry::NAME``
    /// 
    /// * `event`: Events
    ///     - Events is a custom type defined in types::logs::Events
    ///         - Events represents the type of the event the log is, either a Register or a Transfer. Can be ``Events::Register`` or ``Events::Transfer``
    /// 
    /// ## Return Type
    /// 
    /// * ``-> Result<ethers::abi::Log, Box<dyn std::error::Error>>``
    /// 
    ///     - Success: ethers::abi::Log
    ///         - It's like a ethers::core::types::Log, but represents the inner arguments of the event instead
    ///             - It holds the from, to, and tokenId in this scenario
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("client".to_string());
    /// 
    /// let logs = farcaster.get_id_registry_logs().await.unwrap();
    /// 
    /// let log = logs.get(0).unwrap();
    /// 
    /// let parsed_log = farcaster.parse_log(log, Registry::ID, Events::Register).await.unwrap();
    /// 
    /// println!("{:#?}", parsed_log);
    /// ```
pub mod parse_log;