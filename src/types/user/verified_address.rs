use serde::{Deserialize, Serialize};

/// # Verified Address Root Struct
/// 
/// When getting a verified address, we need structs to Deserialize the JSON into
/// 
/// This struct holds on type
/// 
/// * `result: Result`
///     - Result is a struct that holds a vector of VerifiedAddress'
///     - To view the other structs that VerifiedAddress' gets Deserialized into, explore this file
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub verified_addresses: Vec<VerifiedAddress>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedAddress {
    pub signed_message: String,
    pub signer_address: String,
    pub farcaster_address: String,
    pub original_message: String,
}
