pub mod account;
pub mod casts;
pub mod constants;
pub mod registry;
pub mod types;
pub mod users;
pub use types::account::FarcasterAccount;
pub use types::registry::Registry;

use std::error::Error;

/// The Farcaster type that holds the keys to the castle - so to speak :)
#[derive(Debug)]
pub struct Farcaster {
    #[allow(dead_code)]
    pub(crate) account: FarcasterAccount,
    pub registry: Registry,
}

impl Farcaster {
    pub async fn new(
        ethereum_provider: &str,
        account: FarcasterAccount,
    ) -> Result<Self, Box<dyn Error>> {
        let registry = Registry::new(ethereum_provider).await?;

        Ok(Farcaster { account, registry })
    }
}
