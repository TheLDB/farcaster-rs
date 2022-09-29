use ethers::{signers::{Wallet, Signer}, prelude::k256::ecdsa::SigningKey};

use crate::Farcaster;

impl Farcaster {
    pub async fn get_notifications(&self, wallet: Wallet<SigningKey>) -> String {
        let address = wallet.address();
        println!("https://api.farcaster.xyz/v1/notifications?address={:#0x}", address);
        let req = reqwest::get(format!("https://api.farcaster.xyz/v1/notifications?address={:#0x}", address)).await.unwrap().text().await.unwrap();
        // Can't be parsed into a struct without the creation of a seperate macro crate which creates & fills a new struct at runtime
        // Might be able to do that in the future, return String for now
        String::from(req)
    }

}
