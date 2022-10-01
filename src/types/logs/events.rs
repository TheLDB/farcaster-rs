use std::fmt;


use serde::{Deserialize, Serialize};

/// ## Events Enum
/// 
/// Registry holds two possibilities for the enum
/// 
/// * `Transfer`
///     - Represents the Transfer event on either an ID or an fname
/// 
/// * `Register`
///     - Represents the Register event on either an ID or an fname
#[derive(Debug, Deserialize, Serialize)]
pub enum Events {
    Transfer,
    Register,
}


impl fmt::Display for Events {
    /// ## Implementing Display for the Events enum
/// 
/// For a lot of functions, we need these Transfer and Register events to be strings
/// 
/// Here, this just turns them into their respective strings when you run something like:
/// ```
/// let transfer_string = Events::Transfer.to_string();
/// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Events::Register => write!(f, "Register"),
            Events::Transfer => write!(f, "Transfer")
        }
    }
}