use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type FollowerRoot = Vec<Follower>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Follower {
    pub address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Avatar,
    pub is_viewer_following: bool,
    pub verifications: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub url: Option<String>,
    pub is_verified: Option<bool>,
}
