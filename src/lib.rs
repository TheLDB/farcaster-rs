//! # farcaster-rs
//! By [Landon Boles](https://github.com/TheLDB)
//! 
//! farcaster-rs is a simple and easy way to interact with [Farcaster](https://farcaster.xyz) in Rust
//! 
//! # Example Usage
//! 
//! * To view Logs documentation, go:
//!     - [here](https://github.com/TheLDB/farcaster-rs/blob/main/docs/logs.md) for GitHub examples
//!     - [or here](./logs/index.html) for docs.rs documentation
//! 
//! * To view ABI documentation, go:
//!     - [here](https://github.com/TheLDB/farcaster-rs/blob/main/docs/abi.md) for GitHub examples
//!     - [or here](./abi/index.html) for docs.rs documentation
//! 
//! * To view Types documentation, go:
//!     - [here](./types/index.html) for docs.rs documentation
//! 
//! * To view Users documentation, go:
//!     - [here](https://github.com/TheLDB/farcaster-rs/blob/main/docs/users.md) for GitHub documentation
//!     - [or here](./users/index.html) for docs.rs documentation
//! 
//! # Get Started
//! 
//! ```
//! // First, you'll need a RPC node to connect to, I use infura.io
//! 
//! use farcaster_rs::Farcaster;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let farcaster = Farcaster::new("https://goerli.infura.io/v3/{key}");
//! 
//!     // Get a users info
//!     let user = farcaster.get_user_by_username("dwr".to_string()).await.unwrap();
//! 
//!     println!("{:#?}", user);
//! }


use ethers::{core::{types::Address, abi::Abi}, providers::{Provider, Http}};
use types::abi::registry::Registry;
pub mod logs;
pub mod abi;
pub mod types;
pub mod users;
pub mod casts;

/// The Farcaster type that holds the keys to the castle - so to speak :)
#[derive(Debug)]
pub struct Farcaster {
    pub address: Address,
    pub name_registry_abi: Abi,
    pub id_registry_abi: Abi,
    pub provider: Provider<Http>
}


impl Farcaster {
    pub fn new(client: String) -> Self {
        let address = "0xe3be01d99baa8db9905b33a3ca391238234b79d1".parse::<Address>().unwrap();
        let name_abi_str = Farcaster::get_registry_abi(Registry::NAME).unwrap();
        let name_abi: Abi = serde_json::from_str(name_abi_str).unwrap();
        let id_abi_str = Farcaster::get_registry_abi(Registry::ID).unwrap();
        let id_abi: Abi = serde_json::from_str(id_abi_str).unwrap();
        let client = Provider::<Http>::try_from(client).unwrap();

        Farcaster {
            address,
            name_registry_abi: name_abi,
            id_registry_abi: id_abi,
            provider: client
        }
    }
}