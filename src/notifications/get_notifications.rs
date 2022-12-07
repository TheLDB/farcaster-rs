use std::error::Error;
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::notifications::notifications::NotificationsRoot;

impl Farcaster {
    pub async fn get_notifications(&self, limit: Option<i64>, cursor: Option<&str>) -> Result<NotificationsRoot, Box<dyn Error>> {
        let mut url = format!("{}/v2/mention-and-reply-notifications", API_ROOT);

        if limit.is_some() {
            url.push_str(format!("&limit={}", limit.unwrap()).as_str())
        }

        if cursor.is_some() {
            url.push_str(format!("&cursor={}", cursor.unwrap()).as_str())
        }

        let notifications_reqwest = &self.reqwest_get(&url).await?;

        let notifications: NotificationsRoot = serde_json::from_str(&notifications_reqwest)?;

        Ok(notifications)
    }
}