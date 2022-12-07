use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedViewerContext {
    pub following: bool,
    #[serde(rename="followedBy")]
    pub followed_by: bool
}