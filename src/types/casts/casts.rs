use std::any::Any;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CastRoot {
    pub result: Result,
    pub next: Option<Next>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub casts: Vec<Cast>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cast {
    pub hash: String,
    pub thread_hash: String,
    pub parent_hash: Option<String>,
    pub parent_author: Option<ParentAuthor>,
    pub author: Author,
    pub text: String,
    pub published_at: i64,
    pub attachments: Option<Attachments>,
    pub replies: Replies,
    pub reactions: Reactions,
    pub recasts: Recasts,
    pub watches: Watches,
    pub viewer_context: ViewerContext,
    pub recast: Option<bool>,
    #[serde(default)]
    pub mentions: Vec<Mention>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAuthor {
    pub fid: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub pfp: Option<Pfp>,
    pub follower_count: i64,
    pub following_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pfp {
    pub url: String,
    pub verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub fid: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub pfp: Option<Pfp2>,
    pub follower_count: i64,
    pub following_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pfp2 {
    pub url: String,
    pub verified: bool,
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
    pub logo: Option<String>,
    pub use_large_image: Option<bool>,
    pub stripped_cast_text: Option<String>,
    pub image: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Replies {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactions {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recasts {
    pub count: i64,
    pub recasters: Vec<Recaster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recaster {
    pub fid: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub recast_hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watches {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerContext {
    pub reacted: bool,
    pub recast: bool,
    pub watched: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub fid: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub pfp: Option<Pfp3>,
    pub follower_count: i64,
    pub following_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pfp3 {
    pub url: String,
    pub verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    pub cursor: Option<String>,
}
