//! The Follows module that handles followers and follows and following
//!
//! # Quickstart with the Following module
//!
//! 1. Use the follow_user function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Follow a user
//! let follow = farcaster.follow_user_by_username("dwr").await?;
//! let follow = farcaster.follow_user_by_fid(0).await?;
//! let follow = farcaster.follow_user_by_address("0x0000....").await?;
//! ```
//!
//! 2. Use the unfollow_user function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Unfollow a user
//! let unfollow = farcaster.unfollow_user_by_username("dwr").await?;
//! let unfollow = farcaster.unfollow_user_by_fid(0).await?;
//! let unfollow = farcaster.unfollow_user_by_address("0x0000....").await?;
//! ```
//!
//! //! 3. Use the get_followers function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get followers
//! let followers = farcaster.get_followers_by_address("juan", None, None).await?;
//! let followers = farcaster.get_followers_by_fid(0, None, None).await?;
//! let followers = farcaster.get_followers_by_address("0x0000....", None, None).await?;
//! ```
//!
//! //! //! 4. Use the following function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get following
//! let following = farcaster.get_following_by_username("jayme", None, None).await?;
//! let following = farcaster.get_following_by_fid(0, None, None).await?;
//! let following = farcaster.get_following_by_address("0x000....", None, None).await?;
//! ```
//!

/// Follow a user
pub mod follow_user;
/// Get following
pub mod following;
/// Get followers
pub mod get_followers;
/// Unfollow a user
pub mod unfollow_user;
