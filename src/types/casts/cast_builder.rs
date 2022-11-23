use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CastBuilder {
    pub(crate) content: Option<String>,
    pub(crate) embeds: Option<Vec<String>>,
    pub(crate) mentions: Option<Vec<String>>,
}
