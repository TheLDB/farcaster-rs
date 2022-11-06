use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bearer {
    pub bearer: String,
    pub payload: Payload,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub method: String,
    pub params: Params,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    pub timestamp: i64,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
}
