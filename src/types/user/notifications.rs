use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct NotifRoot {
    pub result: NotifResult,
    pub meta: Meta
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct NotifResult {
    pub notifications: HashMap<String, CastReaction>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CastReaction {
    pub id: String,
    #[serde(rename="type")]
    pub reaction_type: String,
    pub timestamp: i64,
    pub user: User,
    #[serde(rename="aggregatedItems")]
    pub aggregated_items: Option<Vec<AggregatedItem>>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub address: String,
    #[serde(rename="displayName")]
    pub display_name: String,
    pub avatar: Avatar,
    #[serde(rename="isViewerFollowing")]
    pub is_viewer_following: Option<bool>,
    #[serde(rename="isFollowingViewer")]
    pub is_following_viewer: Option<bool>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Avatar {
    pub url: String,
    #[serde(rename="isVerified")]
    pub is_verified: bool
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]

pub struct AggregatedItem {
    pub id: String,
    pub timestamp: i64,
    pub user: User,
    pub reaction: Option<Reaction>,
    pub cast: Option<Cast>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    pub merkle_root: String,
    pub cast_merkle_root: String,
    pub fc_address: String,
    pub address: String,
    pub timestamp: i64
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cast {
    pub text: String,
    pub merkle_root: String,
    pub thread_merkle_root: String,
    pub published_at: i64
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Meta {
    pub next: String
}