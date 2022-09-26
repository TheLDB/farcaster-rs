use ethers::{core::{types::Address, abi::Abi}, providers::{Provider, Http}};
pub mod logs;
pub mod abi;
pub mod users;
use abi::get_registry_abi::{Registry, get_registry_abi};

#[derive(Debug)]
pub struct Farcaster {
    pub address: Address,
    pub abi: Abi,
    pub provider: Provider<Http>
}

impl Farcaster {
    pub fn new(client: String) -> Self {
        let address = "0xe3be01d99baa8db9905b33a3ca391238234b79d1".parse::<Address>().unwrap();
        let name_abi_str = get_registry_abi(Registry::NAME).unwrap();
        let name_abi: Abi = serde_json::from_str(name_abi_str).unwrap();
        let client = Provider::<Http>::try_from(client).unwrap();

        Farcaster {
            address,
            abi: name_abi,
            provider: client
        }
    }
}