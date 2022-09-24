use ethers::{core::{types::Address, abi::Abi}, providers::{Provider, Http}};

pub struct Farcaster {
    pub address: Address,
    pub abi: Abi,
    pub provider: Provider<Http>
}

impl Farcaster {
    pub fn new(client: String) -> Self {
        let address = "0xe3Be01D99bAa8dB9905b33a3cA391238234B79D1".parse::<Address>().unwrap();
        let abi: Abi = serde_json::from_str(r#"[{"name":"getDirectoryUrl","inputs":[{"internalType":"bytes32","name":"username","type":"bytes32"}],"outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"","type":"address"}],"name":"addressToUsername","outputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"stateMutability":"view","type":"function"}]"#).unwrap();
        let client = Provider::<Http>::try_from(client).unwrap();

        Farcaster {
            address,
            abi,
            provider: client
        }
    }
}