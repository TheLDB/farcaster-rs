use crate::constants::merkle::API_ROOT;
use crate::types::follows::followers::FollowersRoot;
use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    pub async fn get_following_by_fid(
        &self,
        fid: i64,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<FollowersRoot, Box<dyn Error>> {
        let mut url = format!("{}/v2/following?fid={}", API_ROOT, fid);

        if limit.is_some() {
            url.push_str(format!("&limit={}", limit.unwrap()).as_str());
        }

        if cursor.is_some() {
            url.push_str(format!("&cursor={}", cursor.unwrap()).as_str());
        }

        let following_reqwest = &self.reqwest_get(&url).await?;

        let following: FollowersRoot = serde_json::from_str(&following_reqwest)?;

        Ok(following)
    }

    pub async fn get_following_by_username(
        &self,
        username: &str,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<FollowersRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_username(username).await?;

        let following = &self.get_following_by_fid(fid.fid, limit, cursor).await?;

        Ok(following.clone())
    }

    pub async fn get_following_by_address(
        &self,
        address: &str,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<FollowersRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_address(address).await?;

        let following = &self.get_following_by_fid(fid.fid, limit, cursor).await?;

        Ok(following.clone())
    }
}
