use ethers::{signers::{Wallet, Signer}, prelude::k256::{ecdsa::SigningKey}};
use chrono::offset;
use crate::Farcaster;


async fn auth_header(wallet: Wallet<SigningKey>) {
    let address = wallet.address();
    let now = offset::Utc::now();
    let timestamp = now.timestamp();
    let expiry = timestamp + 60;
    let signer = wallet.signer();
    let signer = signer.verifying_key();
    // Feature on hold- can't figure out how to create a valid JWT in rust :(
}

impl Farcaster {
    pub async fn publish_cast(wallet: Wallet<SigningKey>) -> Result<(), Box<dyn std::error::Error>> {
        let auth = auth_header(wallet).await;
        Ok(())
    }
}