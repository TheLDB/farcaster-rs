use ethers::{signers::{Wallet, Signer}, prelude::k256::ecdsa::SigningKey};
use chrono::offset::Utc;
use crate::Farcaster;

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct CustomClaims {
//     pub issuer: &'static SigningKey
// }

async fn auth_header(wallet: Wallet<SigningKey>) -> Result<String, Box<dyn std::error::Error>> {
    // // * Retrieve Address
    let _address = wallet.address();

    // // * Create an expiry date 60 seconds ahead
    let current_time = Utc::now().timestamp();
    let _expiry_time = current_time + 60;

    // // *Retrieve the private key - ??????
    let _key: &SigningKey = wallet.signer();

    // // * Sign a JWT with the signer from above, and the ES256 header - ???????

    // let key = Es256k::new(key);
    // // * Return JWT
    Ok(String::from("???????"))
}

impl Farcaster {
    pub async fn publish_cast(wallet: Wallet<SigningKey>, _content: String) -> Result<(), Box<dyn std::error::Error>> {
        let _auth = auth_header(wallet).await;
        Ok(())
    }
}