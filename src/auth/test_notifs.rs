use crate::{constants::api_root::API_ROOT, types::auth::secret::Secret, Farcaster};

impl Farcaster {
    pub async fn test_notifs(secret: Secret) -> Result<(), Box<dyn std::error::Error>> {
        let notifs = reqwest::Client::new()
            .get(format!("{}/v2/notifications?fid=5", API_ROOT))
            .header("Content-Type", "application/json")
            .header("Authorization", secret.result.token.secret)
            .send()
            .await?
            .text()
            .await?;

        println!("{}", notifs);

        Ok(())
    }
}
