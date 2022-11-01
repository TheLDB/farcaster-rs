use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// # Notification Root Struct
///
/// When getting all of the Notifications, we need structs to Deserialize all of the JSON into
///
/// This holds two different values:
///
/// * `result`: NotifResult
///     - NotifResult is also defined in this struct, feel free to explore it, there is no documentation for it
///
/// * `meta`: Meta
///     - Meta holds a next value which is the link to the next page of notifications for Pagination reasons
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct NotifRoot {
    pub result: NotifResult,
    pub meta: Meta,
}

/// # Notification Result Struct
///
/// This is an inner struct for the Notifications, and only getting documentation because the way it's parsed is special.
///
/// * When getting the results for the notifications, no object key is defined/set, its always a prefix, then a random address
///     - Ex: ``cast-reaction-v2:0x00000....`` but with an actual address
///
/// * Because of the above, we parse the JSON into a ``HashMap<String, CastReaction``
///     - The String will be the object name, and the CastReaction is the underlying value
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct NotifResult {
    pub notifications: HashMap<String, CastReaction>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CastReaction {
    pub id: String,
    #[serde(rename = "type")]
    pub reaction_type: String,
    pub timestamp: i64,
    pub user: User,
    #[serde(rename = "aggregatedItems")]
    pub aggregated_items: Option<Vec<AggregatedItem>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub address: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub avatar: Avatar,
    #[serde(rename = "isViewerFollowing")]
    pub is_viewer_following: Option<bool>,
    #[serde(rename = "isFollowingViewer")]
    pub is_following_viewer: Option<bool>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Avatar {
    pub url: String,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]

pub struct AggregatedItem {
    pub id: String,
    pub timestamp: i64,
    pub user: User,
    pub reaction: Option<Reaction>,
    pub cast: Option<Cast>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub merkle_root: String,
    pub cast_merkle_root: String,
    pub fc_address: String,
    pub address: String,
    pub timestamp: i64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cast {
    pub text: String,
    pub merkle_root: String,
    pub thread_merkle_root: String,
    pub published_at: i64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Meta {
    pub next: String,
}
