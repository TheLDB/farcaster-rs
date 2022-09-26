use crate::Farcaster;
use crate::types::user;
impl Farcaster {
    pub async fn get_user_by_username(self, username: String) -> Result<String, Box<dyn std::error::Error>> {
        let name_registry_logs = self.get_name_registry_logs().await.unwrap();
 
        for i in name_registry_logs {
            if i.fname == username {
                let address = i.log_desc.params.get(1).unwrap();
                let address = format!("0x{}", address.value.to_string());
                println!("{}", format!("https://api.farcaster.xyz/v1/profiles/{}", address));
                let user = reqwest::get(format!("https://api.farcaster.xyz/v1/profiles/{}", address)).await?.text().await?;
                let test: user::Root = serde_json::from_str(&user).unwrap();
                println!("{:?}", test);
                return Ok(address)
            }
        }

        Err(Box::from("No logs found?"))
    }
}