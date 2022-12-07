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
