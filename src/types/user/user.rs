use serde::{Deserialize, Serialize};
use serde_json::Value;

/// # User Root Struct
///
/// When getting a user, we need structs to Deserialize the JSON into
///
/// This struct holds one value
///
/// * `result: Result`
///     - Result holds a User type, which itself holds a lot of the user data
///     - To view more about the data types, explore the many structs in this file.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub address: String,
    pub username: String,
    pub display_name: String,
    pub avatar: Avatar,
    pub follower_count: i64,
    pub following_count: i64,
    pub is_viewer_following: bool,
    pub is_following_viewer: bool,
    pub profile: Profile,
    pub referrer_username: Option<String>,
    pub viewer_can_send_direct_casts: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub url: String,
    pub is_verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub bio: Bio,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bio {
    pub text: String,
    pub mentions: Vec<Value>,
}
