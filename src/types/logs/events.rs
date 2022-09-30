use std::fmt;

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub enum Events {
    Transfer,
    Register,
}

impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Events::Register => write!(f, "Register"),
            Events::Transfer => write!(f, "Transfer")
        }
    }
}