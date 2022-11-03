use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRoot {
    result: Result,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Result {
    user: User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub fid: i64,

    pub username: String,

    #[serde(rename = "displayName")]
    pub display_name: String,

    pub avatar: Avatar,

    #[serde(rename = "followerCount")]
    pub follower_count: i64,

    #[serde(rename = "followingCount")]
    pub following_count: i64,

    #[serde(rename = "isViewerFollowing")]
    pub is_viewer_following: bool,

    #[serde(rename = "isFollowingViewer")]
    pub is_following_viewer: bool,

    pub profile: Profile,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Avatar {
    pub url: String,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub bio: Bio,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bio {
    pub text: String,
    pub mentions: Vec<Mention>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mention {
    pub avatar: Avatar,
    pub fid: i64,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}
