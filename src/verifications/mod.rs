//! The Verifications module that handles proof verifications
//!
//! # Quickstart with the Verifications module
//!
//! 1. Use the get_verifications function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get verifications
//! let verif = farcaster.get_verifications_by_username("giu").await?;
//! let verif = farcaster.get_verifications_by_address("0x000...").await?;
//! let verif = farcaster.get_verifications_by_fid(0).await?;
//! ```
//!
//!

/// Get verifications of a user
pub mod get_verifications;
