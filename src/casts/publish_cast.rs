use ethers::{signers::{Wallet, Signer}, prelude::k256::ecdsa::SigningKey};
use chrono::offset::Utc;
use crate::Farcaster;

async fn auth_header(wallet: Wallet<SigningKey>) -> Result<String, Box<dyn std::error::Error>> {
    // * Retrieve Address
    let _address = wallet.address();

    // * Create an expiry date 60 seconds ahead
    let current_time = Utc::now().timestamp();
    let _expiry_time = current_time + 60;

    // *Retrieve the private key - ??????
    let key: &SigningKey = wallet.signer();
    println!("{:?}", key);

    // * Sign the private key with ES256K - ???????

    // * Sign a JWT with the signer from above, and the ES256 header - ???????

    // * Return JWT
    Ok(String::from("???????"))
}

impl Farcaster {
    pub async fn publish_cast(wallet: Wallet<SigningKey>, _content: String) -> Result<(), Box<dyn std::error::Error>> {
        let _auth = auth_header(wallet).await;
        Ok(())
    }
}