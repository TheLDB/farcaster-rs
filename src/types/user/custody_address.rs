use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CustodyAddressRoot {
    pub result: CustodyAddressResult,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustodyAddressResult {
    #[serde(rename = "custodyAddress")]
    pub custody_address: String,
}
