use crate::{Farcaster, types::casts::casts::{Root}};

#[allow(unreachable_code)]
impl Farcaster {
    pub async fn get_casts(&self, username: String, casts_per_page: i64, page: i64) -> Result<Root, Box<dyn std::error::Error>> {
        let user = Farcaster::get_user_by_username(self, username).await.unwrap();
        let address = user.result.user.address;

        let user = reqwest::get(format!("https://api.farcaster.xyz/v1/profiles/{}/casts?per_page={}", address, casts_per_page)).await?.text().await?;
        let casts: Root = serde_json::from_str(&user).unwrap();
        if page > 0 {
            let mut loopy = 0;
            let mut new_user: String = String::from("");
            while loopy != page {
                new_user = reqwest::get(casts.meta.next.clone()).await?.text().await?;
                loopy += 1;
            }
            
            let casts: Root = serde_json::from_str(&new_user).unwrap();
            return Ok(casts)
        }
        else {
            return Ok(casts)
        }

        Err(Box::from("This error should be unreachable - if you got here either I fucked up or you fucked up"))
    }
}