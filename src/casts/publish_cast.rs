use ethers::{signers::{Wallet, Signer}, prelude::k256::ecdsa::SigningKey};
use chrono::offset;
use crate::Farcaster;

async fn auth_header(wallet: Wallet<SigningKey>) {
    // let _address = wallet.address();
    // let now = offset::Utc::now();
    // let timestamp = now.timestamp();
    // let _expiry = timestamp + 60;
    // let aaa = wallet.signer();
}

impl Farcaster {
    pub async fn publish_cast(wallet: Wallet<SigningKey>) -> Result<(), Box<dyn std::error::Error>> {
        let _auth = auth_header(wallet).await;
        Ok(())
    }
}