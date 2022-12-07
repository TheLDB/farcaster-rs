//! The Users module that handles users
//!
//! # Quickstart with the Users module
//!
//! 1. Use the get_user function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get a user
//! let user = farcaster.get_user_by_username("ace").await?;
//! let user = farcaster.get_user_by_address("0X000...").await?;
//! let user = farcaster.get_user_by_fid(0).await?;
//! ```
//!
//! 2. Use the get_custody_address function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get custody address
//! let custody_address = farcaster.get_custody_address_by_username("v").await?;
//! let custody_address = farcaster.get_custody_address_by_fid(0).await?;
//! let custody_address = farcaster.get_custody_address_by_address("0x0000").await?;
//! ```
//!
//! Use the me function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get me
//! let me = farcaster.get_me().await?;
//! ```

/// Get a custody address by FID, Address, or Username
pub mod get_custody_address;
/// Get a user by FID, Address, or Username
pub mod get_user;
/// Get info about the authenticated user
pub mod me;
