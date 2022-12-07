use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecastersRoot {
    pub result: RecastersResult
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecastersResult {
    pub users: Vec<Recaster>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recaster {
    pub fid: i64,
    pub username: String,
    #[serde(rename="displayName")]
    pub display_name: String,
    pub pfp: Pfp,
    pub profile: Profile,
    #[serde(rename="followerCount")]
    pub follower_count: i64,
    #[serde(rename="followingCount")]
    pub following_count: i64,
    #[serde(rename="ViewerContext")]
    pub viewer_context: ViewerContext
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pfp {
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