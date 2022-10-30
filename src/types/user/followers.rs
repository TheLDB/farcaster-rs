use serde::{Deserialize, Serialize};
use serde_json::Value;

/// When getting all the followers of a caster, we need structs to Deserialize the JSON into
///     - Follower is defined in this file, along with the Avatar struct, feel free to explore these to learn more about the data types
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
