use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCollectionsRoot {
    pub result: UserCollectionsResult,
    pub next: Option<UserCollectionsNext>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCollectionsResult {
    pub collections: Vec<Collection>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename="itemCount")]
    pub item_count: i64,
    #[serde(rename="ownerCount")]
    pub owner_count: i64,
    #[serde(rename="farcasterOwnerCount")]
    pub fc_owner_count: i64,
    #[serde(rename="imageUrl")]
    pub image_url: String,
    #[serde(rename="volumeTraded")]
    pub volume_traded: String,
    #[serde(rename="externalUrl")]
    pub external_url: Option<String>,
    #[serde(rename="openSeaUrl")]
    pub opensea_url: Option<String>,
    #[serde(rename="twitterUsername")]
    pub twitter_username: Option<String>,
    #[serde(rename="schemaName")]
    pub schema_name: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCollectionsNext {
    pub cursor: Option<String>
}