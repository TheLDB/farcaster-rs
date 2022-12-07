use crate::types::shared::pfp::SharedPfp;
use crate::types::shared::shared_profile::SharedProfile;
use crate::types::shared::viewer_context::SharedViewerContext;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowersRoot {
    pub result: FollowersUsersVec,
    pub next: Option<FollowersNext>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowersUsersVec {
    pub users: Vec<Follower>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Follower {
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
    #[serde(rename = "viewerContext")]
    pub viewer_context: SharedViewerContext,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowersNext {
    pub cursor: Option<String>,
}
