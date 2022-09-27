use ethers::{signers::Wallet, prelude::k256::ecdsa::SigningKey};

use crate::Farcaster;

async fn _auth_header() -> String {
    "aaaa".to_string()
}
impl Farcaster {
    pub async fn publish_cast(wallet: Wallet<SigningKey>) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}", wallet);
        Ok(())
    }
}