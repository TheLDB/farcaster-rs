use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CastBuilder {
    pub(crate) content: String,
    pub(crate) embeds: Vec<String>,
    pub(crate) mentions: Vec<String>,
}
