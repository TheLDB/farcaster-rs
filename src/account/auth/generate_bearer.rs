use chrono::{DateTime, Utc};
use ethers::{
    prelude::k256::ecdsa::SigningKey,
    signers::{Signer, Wallet},
    types::Signature,
};

use crate::{
    types::account::auth::bearer::{Bearer, Params, Payload},
    Farcaster,
};

use serde_json::{json, Value};

impl Farcaster {
    /// # Best not to use this function directly
    /// Instead, use the [Account struct](../types/account/struct.Account.html) methods, such as from_mnemonic, and from_private_key, as that generates these automatically.
    ///
    /// However, if you'd really like to use these, go ahead.
    pub async fn generate_bearer(
        wallet: &Wallet<SigningKey>,
        duration_secs: Option<i64>,
    ) -> Result<Bearer, Box<dyn std::error::Error>> {
        // Get the current unix timestamp (non-leap seconds since January 1, 1970 00:00:00 UTC)
        let dt: DateTime<Utc> = Utc::now();
        let timestamp = dt.timestamp_millis();

        // fill in bearer token parameters
        let params = Params {
            timestamp,
            expires_at: match duration_secs {
                // with expiration time
                Some(secs) => Some(timestamp + (secs * 1000)),
                // without expiration time
                None => None,
            },
        };

        // Initialize a bearer payload using serde_json
        let payload: Value = json!({
                "method": "generateToken",
                "params": params,
        });

        // Sign the payload using our ethers wallet
        let signature: Signature = wallet.sign_message(payload.to_string()).await?;

        // Convert our signature to a Vec<u8>
        let arrayify: Vec<u8> = hex::decode(signature.to_string())?;

        // Encode the signature to a base64 format
        let base64_signature: String = base64::encode(arrayify);

        // Format our signature
        let bearer = format!("Bearer eip191:{}", base64_signature);

        let payload = Payload {
            method: "generateToken".to_string(),
            params,
        };

        let bearer = Bearer { bearer, payload };

        Ok(bearer)
    }
}
