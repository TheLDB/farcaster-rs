use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LikedCastRoot {
    pub result: LikedCastResult
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LikedCastResult {
    pub like: Like
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Like {
    #[serde(rename="type")]
    pub like_type: String,
    pub hash: String,
    pub reactor: Reactor,
    pub timestamp: i64,
    #[serde(rename="castHash")]
    pub cast_hash: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reactor {
    pub fid: i64,
    pub username: String,
    #[serde(rename="displayName")]
    pub display_name: String,
    pub pfp: PFP,
    pub profile: Profile,
    #[serde(rename="followerCount")]
    pub follower_count: i64,
    #[serde(rename="followingCount")]
    pub following_count: i64,
    #[serde(rename="referrerUsername")]
    pub referrer_username: Option<String>,
    #[serde(rename="viewerContext")]
    pub viewer_context: ViewerContext
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PFP {
    pub url: String,
    pub verified: bool
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub bio: Bio
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bio {
    pub text: String,
    pub mentions: Option<Vec<String>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerContext {
    pub following: bool,
    #[serde(rename="followedBy")]
    pub followed_by: bool
}