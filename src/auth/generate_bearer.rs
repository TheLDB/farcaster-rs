use chrono::{DateTime, Utc};
use ethers::{
    prelude::k256::ecdsa::SigningKey,
    signers::{Signer, Wallet},
    types::Signature,
};

use crate::{
    types::auth::bearer::{Bearer, Params, Payload},
    Farcaster,
};

use serde_json::{json, Value};

impl Farcaster {
    pub async fn generate_bearer(
        wallet: &Wallet<SigningKey>,
    ) -> Result<Bearer, Box<dyn std::error::Error>> {
        // Get the current unix timestamp (non-leap seconds since January 1, 1970 00:00:00 UTC)
        let dt: DateTime<Utc> = Utc::now();
        let timestamp: i64 = dt.timestamp_millis();
        let expires_at: i64 = timestamp + 300000; // Create an expiration timestamp 5 minutes in the future

        // Initialize a bearer payload using serde_json
        let payload: Value = json!({
            "method": "generateToken",
            "params": {
                "timestamp": timestamp,
                "expiresAt": expires_at
            }
        });

        // Sign the payload using our ethers wallet
        let signature: Signature = wallet.sign_message(payload.to_string()).await?;

        // Convert our signature to a Vec<u8>
        let arrayify: Vec<u8> = hex::decode(signature.to_string())?;

        // Encode the signature to a base64 format
        let base64_signature: String = base64::encode(arrayify);

        // Format our signature
        let bearer = format!("Bearer eip191:{}", base64_signature);

        let params = Params {
            timestamp,
            expires_at,
        };

        let payload = Payload {
            method: "generateToken".to_string(),
            params,
        };

        let bearer = Bearer { bearer, payload };

        Ok(bearer)
    }
}
