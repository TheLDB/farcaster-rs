use crate::types::user::notifications::{CastReaction, NotifRoot};
use crate::Farcaster;
use std::collections::HashMap;
use std::error::Error;

impl Farcaster {
    /// fetches notifications for a given `address` up to `max_notifications`
    /// if `max_notifications` is zero fetches all available notifications
    /// TODO: implement trimming returned HashMap to `max_notifications` (without breaking ordering/timeline)
    pub async fn get_notifications_by_address_with_limit(
        &self,
        address: &str,
        max_notifications: usize,
    ) -> Result<HashMap<String, CastReaction>, Box<dyn Error>> {
        let mut notifications = HashMap::new();
        let mut fetch_url = format!(
            "https://api.farcaster.xyz/v1/notifications?address={}",
            address
        );

        // dirty hack for unlimited fetch
        let read_limit = match max_notifications {
            0 => usize::MAX,
            _ => max_notifications,
        };

        while notifications.len() < read_limit {
            // fetch notifications
            let response = reqwest::get(&fetch_url).await?.text().await?;
            let notif_root: NotifRoot = serde_json::from_str(&response)?;

            // store received notifications
            notifications.extend(notif_root.result.notifications);

            if let Some(next_url) = notif_root.meta.next {
                // update fetch_url
                fetch_url = next_url;
            } else {
                // no more notifications available
                break;
            }
        }

        Ok(notifications)
    }

    /// fetches notifications for a given `address`
    pub async fn get_notifications_by_address(
        &self,
        address: &str,
    ) -> Result<HashMap<String, CastReaction>, Box<dyn Error>> {
        self.get_notifications_by_address_with_limit(address, 0)
            .await
    }

    /// fetches notifications for a given `username` up to `max_notifications`
    /// if `max_notifications` is zero fetches all available notifications
    pub async fn get_notifications_by_username_with_limit(
        &self,
        username: &str,
        max_notifications: usize,
    ) -> Result<HashMap<String, CastReaction>, Box<dyn Error>> {
        let user = self.get_user_by_username(username).await?;
        self.get_notifications_by_address_with_limit(&user.result.user.address, max_notifications)
            .await
    }

    /// fetches notifications for a given `username`
    pub async fn get_notifications_by_username(
        &self,
        username: &str,
    ) -> Result<HashMap<String, CastReaction>, Box<dyn Error>> {
        self.get_notifications_by_username_with_limit(username, 0)
            .await
    }
}
