use crate::Farcaster;
use crate::abi::get_registry_abi::Registry;
use crate::types::user::user::Root;
use crate::types::{logs::events::Events};

impl Farcaster {
    pub async fn get_user_by_username(&self, username: String) -> Result<Root, Box<dyn std::error::Error>> {
        let name_registry_logs = self.get_name_registry_logs().await.unwrap();

        for name in name_registry_logs {
            let parsed_log = &self.parse_log(name, Registry::NAME, Events::Transfer).await.unwrap();
            let token_id = parsed_log.params.get(2).unwrap();
            let fname = Farcaster::token_to_fname(token_id.value.clone()).await.unwrap();
            if fname == username {
                let address = parsed_log.params.get(1).unwrap();
                let address = format!("0x{}", address.value.to_string());
                let user = reqwest::get(format!("https://api.farcaster.xyz/v1/profiles/{}", address)).await?.text().await?;
                let user: Root = serde_json::from_str(&user).unwrap();
                return Ok(user)
            }
        }

        Err(Box::from("Unable to fetch Name Registry Logs - Fatal Error"))
    }
}