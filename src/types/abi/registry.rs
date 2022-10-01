use serde::{Deserialize, Serialize};

/// ## Registry Enum
/// 
/// Registry holds two possibilities for the enum
/// 
/// * `ID`
///     - Represents logs from the ID registry/contract
/// 
/// * `NAME`
///     - Represents logs from the Name registry/contract
#[derive(Debug, Deserialize, Serialize)]
pub enum Registry {
    ID,
    NAME
}