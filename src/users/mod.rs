//! Documentation for all of the user-related functions

/// # Get the details of a user
    /// 
    /// ## Arguments
    /// 
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client".to_string());``
    /// 
    /// * `username: String`
    ///     - The username you want to fetch casts for
    ///     - Optional
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("".to_string());
    /// 
    /// let user = farcaster.get_user_by_username("lndnnft".to_string()).await.unwrap();
    /// 
    /// print
pub mod get_user;
 /// # Convert a FC V2 token to an fname (username)
    /// 
    /// ## Arguments
    /// 
    /// * `token: Token`
    ///     - Token is a type of ethers::abi::Token
    ///     - Retrieveable by parsing a log from the logs functions 
    /// 
    /// ## Usage
    /// ```
    /// let name_registry_logs = farcaster.get_name_registry_logs().await.unwrap();
    /// 
    /// for name in name_registry_logs {
    ///     let parsed_log = farcaster.parse_log(name, Registry::NAME, Events::Transfer).await.unwrap();
    ///     let token_id = parsed_log.params.get(2).unwrap();
    ///     let fname = Farcaster::token_to_fname(token_id.value).await.unwrap();
    /// 
    ///     println!("{}", fname);
    /// }
    /// ```
pub mod token_to_fname;
/// # Get the verified address of a user
    /// 
    /// ## Arguments
    /// 
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client".to_string());``
    /// 
    /// * `username: String`
    ///     - The username you want to fetch casts for
    ///     - Optional
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("".to_string());
    /// 
    /// let verified_address = farcaster.get_verified_address("0xen".to_string()).await.unwrap();
    /// 
    /// println!("{:#?}", verified_address);
    /// ```
pub mod get_verified_address;
/// # Get all Casts of a user
    /// With pagination!
    /// 
    /// ## Arguments
    /// 
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client".to_string());``
    /// 
    /// * `username: String`
    ///     - The username you want to fetch casts for
    /// 
    /// * `casts_per_page: i64`
    ///     - The amount of casts per page
    /// 
    /// * `page: i64`
    ///     - What page number you get.
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("".to_string());
    /// 
    /// let casts = farcaster.get_casts("dwr".to_string(), 30, 2).await.unwrap();
    /// 
    /// println!("{:#?}", casts);
    /// ```
pub mod get_casts;
/// # Create a new wallet
    /// 
    /// ## Arguments
    /// 
    /// * `key_type: KeyType`
    ///     - Either a PrivateKey or MnemonicPhrase type from the KeyType enum
    /// 
    /// * `key: String`
    ///     - Your private key/mnemonic phrase
    /// 
    /// * `mnemonic_word_count: Option<i64>`
    ///     - Allows you to set a custom word count for your mnemonic phrase
    ///     - Defaults to 12 if None
    /// 
    /// ## Usage
    /// ```
    /// let wallet = Farcaster::new_wallet(KeyType::MnemonicPhrase, "word x 12".to_string(), Some(24));
    /// 
    /// println!("{:#?}", wallet);
    /// ```
pub mod new_wallet;
/// # Get all notifications of a user
    /// 
    /// ## Arguments
    /// 
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client".to_string());``
    /// 
    /// * `username: Option<String>`
    ///     - The username you want to fetch casts for
    ///     - Optional
    /// 
    /// * `address: Option<String>`
    ///     - The address for the user you want to get
    ///     - Optional
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("".to_string());
    /// 
    /// let notifications = farcaster.get_notifications(Some("jayme".to_string()), None).await.unwrap();
    /// 
    /// println!("{:#?}", notifications);
    /// ```
pub mod get_notifications;
/// # Get all followers of a user
    /// 
    /// ## Arguments
    /// 
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client".to_string());``
    /// 
    /// * `username: Option<String>`
    ///     - The username you want to fetch casts for
    ///     - Optional
    /// 
    /// * `address: Option<String>`
    ///     - The address for the user you want to get
    ///     - Optional
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("".to_string());
    /// 
    /// let followers = farcaster.get_followers(Some("ace".to_string()), None).await.unwrap();
    /// 
    /// for follower in followers {
    ///     println!("{:#?}", follower);
    /// }
    /// ```
pub mod get_followers;