//! # farcaster-rs
//! A Rust crate to interact with the [Farcaster](https://farcaster.xyz) smart contracrts & merkle APIs
//!
//! # Quickstart
//! To get started with the farcaster-rs crate, this should serve as a simple example of how to use it
//!
//! ```no_run
//! use farcaster_rs::{
//!     Account,
//!     Farcaster
//! };
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // Initialize a new Account w/ your key and an optional token duration (milliseconds)
//!     let account: Account = Account::from_mnemonic("super top secret phrase", None).await?;
//!
//!     // Connect to an ETH node
//!     let farcaster: Farcaster = Farcaster::new("node url", account).await?;
//!
//!     // Get Dan Romero's Casts with his Username, while specifying a limit, and a cursor
//!     let casts = farcaster.get_casts_by_username("dwr", Some(5), none).await?;
//!
//!     println!("{:#?}", casts);
//!
//!     Ok(())
//! }
//! ```
//! Quick explanation of all folders/modules in farcaster-rs
//!
//! # Documentation Notice
//! Pretty much all documentation for functions is [here](./struct.Farcaster.html)
//!
//! ---------------
//! ## `account`
//! The account module handles authentication w/ the Farcaster Merkle APIs. Generating tokens, revoking tokens, etc....
//! View documentation [here](./account/index.html)
//!
//! ## `api`
//! The API module has room for growth, but for now holds some reqwest wrappers to make my life easier
//! View documentation [here](./api/index.html)
//!
//! ## `assets`
//! The assets module holds functions to get NFT collection related data.
//! View documentation [here](./assets/index.html)
//!
//! ## `casts`
//! The casts module holds functions related to casts on Farcaster. Getting casts, publishing casts, etc...
//! View documentation [here](./casts/index.html)
//!
//! ## `follows`
//! The follows module holds function related to followers and following. Follow users, unfollow, view followers, etc..
//! View documentation [here](./follows/index.html)
//!
//! ## `misc`
//! The misc module holds functions that don't entirely have a place. The only one for now is API Health
//! View documentation [here](./misc/index.html)
//!
//! ## `notifications`
//! The notifications module holds functions relating to notifications, which right now allows you to fetch notifications
//! //! View documentation [here](./notifications/index.html)
//!
//! ## `reactions`
//! The reactions module allows you to view and modify your reactions. Like, recast, list likes/recasts, etc...
//! View documentation [here](./reactions/index.html)
//!
//! ## `registry`
//! The registry module holds important functions most importantly relating to getting users.
//!     - Convert addres/username to FID, get users via FID, etc......
//! View documentation [here](./registry/index.html)
//! ## `users`
//! The users module holds functions relating to users on Farcaster. Get users, get custody adress', etc...
//! //! View documentation [here](./users/index.html)
//!
//! ## `verifications`
//! The verifications module holds functions relating to cryptographic proofs such as getting verified address'
//! //! View documentation [here](./verifications/index.html)


pub mod account;
pub mod api;
pub mod casts;
pub mod constants;
pub mod registry;
pub mod types;
pub mod users;
pub mod reactions;
pub mod follows;
pub mod verifications;
pub mod notifications;
pub mod assets;
pub mod misc;

pub use types::account::Account;
pub use types::registry::Registry;

use std::error::Error;

/// The Farcaster type that holds the keys to the castle - so to speak :)
#[derive(Debug)]
pub struct Farcaster {
    #[allow(dead_code)]
    pub account: Account,
    pub registry: Registry,
}

impl Farcaster {
    pub async fn new(ethereum_provider: &str, account: Account) -> Result<Self, Box<dyn Error>> {
        let registry = Registry::new(ethereum_provider).await?;

        Ok(Farcaster { account, registry })
    }
}
