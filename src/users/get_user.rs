use crate::Farcaster;
use crate::abi::get_registry_abi::Registry;
use crate::types::{user, events::Events};
impl Farcaster {
    pub async fn get_user_by_username(self, _username: String) -> Result<user::Root, Box<dyn std::error::Error>> {
        let name_registry_logs = self.get_name_registry_logs().await.unwrap();

        for name in name_registry_logs {
            let parsed_log = &self.parse_log(name, Registry::NAME, Events::Transfer).await.unwrap();
            let address = parsed_log.params.get(1).unwrap();
            let address = format!("0x{}", address.value.to_string());
            let user = reqwest::get(format!("https://api.farcaster.xyz/v1/profiles/{}", address)).await?.text().await?;
            let user: user::Root = serde_json::from_str(&user).unwrap();
            return Ok(user)
        }

        Err(Box::from("Unable to fetch Name Registry Logs - Fatal Error"))
    }
}