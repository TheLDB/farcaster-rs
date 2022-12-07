use crate::types::casts::published_cast::{Reactions, Recasts, Replies, ViewerContext, Watches};
use crate::types::shared::pfp::SharedPfp;
use crate::types::shared::shared_profile::SharedProfile;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationsRoot {
    pub result: NotificationsResult,
    pub next: Option<NotificationsNext>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationsResult {
    pub notifications: Vec<Notification>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "type")]
    pub notif_type: String,
    pub id: String,
    pub timestamp: i64,
    pub actor: NotificationActor,
    pub content: NotificationContent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationActor {
    pub fid: i64,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub pfp: SharedPfp,
    pub profile: SharedProfile,
    #[serde(rename = "followerCount")]
    pub follower_count: i64,
    #[serde(rename = "followingCount")]
    pub following_name: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationContent {
    pub cast: NotificationCast,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationCast {
    pub hash: String,
    #[serde(rename = "threadHash")]
    pub thread_hash: String,
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    #[serde(rename = "parentAuthor")]
    pub parent_author: NotificationParentAuthor,
    pub author: NotificationParentAuthor,
    pub text: String,
    pub timestamp: i64,
    pub replies: Replies,
    pub reactions: Reactions,
    pub recasts: Recasts,
    pub watches: Watches,
    #[serde(rename = "viewerContext")]
    pub viewer_context: ViewerContext,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationParentAuthor {
    pub fid: i64,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub pfp: SharedPfp,
    pub profile: SharedProfile,
    #[serde(rename = "followerCount")]
    pub follower_count: i64,
    #[serde(rename = "followingCount")]
    pub following_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationsNext {
    pub cursor: Option<String>,
}
