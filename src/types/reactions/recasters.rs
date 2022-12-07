use crate::types::shared::pfp::SharedPfp;
use crate::types::shared::shared_profile::SharedProfile;
use crate::types::shared::viewer_context::SharedViewerContext;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecastersRoot {
    pub result: RecastersResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecastersResult {
    pub users: Vec<Recaster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recaster {
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
    #[serde(rename = "ViewerContext")]
    pub viewer_context: SharedViewerContext,
}
