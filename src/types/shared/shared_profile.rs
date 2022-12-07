use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedProfile {
    pub bio: SharedBio
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedBio {
    pub text: String,
    pub mentions: Option<Vec<String>>,
}