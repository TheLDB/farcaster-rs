use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRoot {
    pub result: Result,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Result {
    pub user: UserInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub fid: i64,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub pfp: Pfp,
    #[serde(rename = "followerCount")]
    pub follower_count: i64,
    #[serde(rename = "followingCount")]
    pub following_count: i64,
    #[serde(rename = "referrerUsername")]
    pub referrer_username: String,
    #[serde(rename = "viewerContext")]
    pub viewer_content: ViewerContext,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pfp {
    pub url: String,
    pub verified: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewerContext {
    pub following: bool,
    #[serde(rename = "followedBy")]
    pub followed_by: bool,
    #[serde(rename = "canSendDirectCasts")]
    pub can_send_direct_casts: bool,
}
