use crate::constants::merkle::API_ROOT;
use crate::types::notifications::notifications::NotificationsRoot;
use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    /// Fetch your notifications
    ///
    /// # Params
    /// limit: Option<i64> - How many casts to get (max 100)
    /// cursor: Option<&str) - For pagination
    ///
    /// # Example
    /// ```no_run
    /// let notifications = farcaster.get_notifications(None, None).await?;
    /// ```
    pub async fn get_notifications(
        &self,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<NotificationsRoot, Box<dyn Error>> {
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
