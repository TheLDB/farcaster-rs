//! The Notifications module that handles notifications
//!
//! # Quickstart with the Notification module
//!
//! 1. Use the get_notifications function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get Notifications
//! let notifications = farcaster.get_notifications(None, None).await?;
//! ```

/// Fetch your notifications
pub mod get_notifications;