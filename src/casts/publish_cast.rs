use ethers::{signers::Wallet, prelude::k256::ecdsa::SigningKey};
use crate::Farcaster;

async fn auth_header(_wallet: Wallet<SigningKey>) {
    // * Retrieve Address

    // * Create an expiry date 60 seconds ahead

    // * Retrieve the private key

    // * Sign the private key with ES256K

    // * Sign a JWT with the signer from above, and the ES256 header

    // * Return JWT
}

impl Farcaster {
    pub async fn publish_cast(wallet: Wallet<SigningKey>, _content: String) -> Result<(), Box<dyn std::error::Error>> {
        let _auth = auth_header(wallet).await;
        Ok(())
    }
}