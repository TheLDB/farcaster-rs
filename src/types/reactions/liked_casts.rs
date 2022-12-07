use serde::{Deserialize, Serialize};
use crate::types::shared::pfp::SharedPfp;
use crate::types::shared::shared_profile::SharedProfile;
use crate::types::shared::viewer_context::SharedViewerContext;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManyLikedCastsRoot {
    pub result: LikedCastResult
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LikedCastResult {
    pub likes: Vec<Like>
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
    pub pfp: SharedPfp,
    pub profile: SharedProfile,
    #[serde(rename="followerCount")]
    pub follower_count: i64,
    #[serde(rename="followingCount")]
    pub following_count: i64,
    #[serde(rename="referrerUsername")]
    pub referrer_username: Option<String>,
    #[serde(rename="viewerContext")]
    pub viewer_context: SharedViewerContext
}