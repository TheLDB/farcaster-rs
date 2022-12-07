use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedPfp {
    pub url: String,
    pub verified: bool,
}
