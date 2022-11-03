use serde_json::json;

use crate::{types::auth::bearer::Bearer, Farcaster};

impl Farcaster {
    pub async fn get_session_token(bearer: Bearer) -> Result<(), Box<dyn std::error::Error>> {
        // let session_reqwest = reqwest::Client::new().put("https://api.farcaster.xyz/v2/auth").header("Content-Type", "application/json").header("Authorization", bearer).json(json)
        Ok(())
    }
}
