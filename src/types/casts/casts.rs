use serde::{Deserialize, Serialize};

/// Casts - Root struct
/// 
/// * ## !IMPORTANT
///     - There are 21 different structs in this file, and only this root one has documentation.
///     - Please explore the file on your own to see the different data types that the JSON gets Deserialized into
/// This struct holds two data types that are two different structs as well
///  - - -
/// * `result`
///     - Is a type of Result, which is a struct in this same file that holds 99% of the stuff
/// 
/// * `meta`
///     - Holds the Next struct, which provides a link for the next page, to allow for pagination.
///     - To use pagination on your own, I just looped through it x times until I reached the correct number.
///         - There might be a better/faster way to do this though, so please play around with it!
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CastRoot {
    pub result: Result,
    pub meta: Meta2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub casts: Vec<Cast>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cast {
    pub body: Body,
    pub attachments: Attachments,
    pub signature: String,
    pub merkle_root: String,
    pub thread_merkle_root: String,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    #[serde(rename = "type")]
    pub type_field: String,
    pub published_at: i64,
    pub sequence: i64,
    pub address: String,
    pub username: String,
    pub data: Data,
    pub prev_merkle_root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub text: String,
    pub reply_parent_merkle_root: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachments {
    pub open_graph: Option<Vec<OpenGraph>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenGraph {
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub domain: Option<String>,
    pub image: Option<String>,
    pub image_id: Option<i64>,
    pub logo: Option<String>,
    pub use_large_image: Option<bool>,
    pub stripped_cast_text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub display_name: String,
    pub avatar: String,
    pub is_verified_avatar: bool,
    #[serde(default)]
    pub mentions: Option<Vec<Mention>>,
    pub num_reply_children: i64,
    pub reactions: Reactions,
    pub recasters: Vec<Recaster>,
    pub recasts: Recasts,
    pub watches: Watches,
    pub reply_parent_username: Option<ReplyParentUsername>,
    pub recast: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub address: Option<String>,
    pub username: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactions {
    pub count: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "self")]
    pub self_field: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recaster {
    pub address: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub avatar: Option<Avatar>,
    pub follower_count: Option<i64>,
    pub following_count: Option<i64>,
    pub is_viewer_following: Option<bool>,
    pub is_following_viewer: Option<bool>,
    pub profile: Option<Profile>,
    pub referrer_username: Option<String>,
    pub avatar_url: Option<String>,
    pub is_verified_avatar: Option<bool>,
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
    pub direct_message_targets: Option<DirectMessageTargets>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bio {
    pub text: String,
    pub mentions: Vec<Mention2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention2 {
    pub avatar: Avatar2,
    pub address: String,
    pub username: String,
    pub display_name: String,
    pub registered_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar2 {
    pub url: String,
    pub is_verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessageTargets {
    pub telegram: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recasts {
    pub count: i64,
    #[serde(rename = "self")]
    pub self_field: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watches {
    pub count: i64,
    #[serde(rename = "self")]
    pub self_field: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyParentUsername {
    pub address: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta2 {
    pub next: String,
}
