use std::error::Error;
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::reactions::liked_casts::ManyLikedCastsRoot;

impl Farcaster {
    pub async fn get_likes_by_cast_hash(&self, cast_hash: &str, limit: Option<i32>, cursor: Option<&str>) -> Result<ManyLikedCastsRoot, Box<dyn Error>> {
        let mut url = format!("{}/v2/cast-likes?castHash={}", API_ROOT, cast_hash);

        if limit.is_some() {
            url.push_str(format!("&limit={}", limit.unwrap()).as_str())
        }

        if cursor.is_some() {
            url.push_str(format!("&cursor={}", cursor.unwrap()).as_str())
        }

        let likes_reqwest = &self.reqwest_get(&url).await?;

        let likes: ManyLikedCastsRoot = serde_json::from_str(&likes_reqwest)?;

        Ok(likes)
    }
}