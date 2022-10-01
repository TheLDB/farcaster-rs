//! Contains all of the ABI/Registry ABI related functions

/// ## Get 1 of the two Registry ABIs
    /// 
    /// Get either the ID or Name registry ABI's (for the FC V2 smart contracts on Goerli)
    /// 
    /// ## Arguments
    /// 
    /// * `registry`: types::abi::registry::Registry
    ///     - Registry is an enum which has two types:
    ///         - ID (Represents the ID registry ABI)
    ///         - NAME (Represents the Name registry ABI)
    /// 
    /// ## Return Type
    /// ``-> Option<&'static str>``
    ///     - Returns an Optional String 
    /// 
    /// ## Usage
    /// ```
    /// use farcaster_rs::{Farcaster, types::abi::registry::Registry};
    /// 
    /// let registry = Farcaster::get_registry_abi(Registry::ID).unwrap();
    /// ```
pub mod get_registry_abi;