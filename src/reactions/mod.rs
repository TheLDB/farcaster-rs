//! The Reactions module that handles likes and recasts
//!
//! # Quickstart with the Reactions module
//!
//! 1. Use the like_cast function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Like Cast
//! let like = farcaster.like_cast_by_cast_hash("cast hash").await?;
//! ```
//!
//! 2. Use the get_likes function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get likes
//! let likes = farcaster.get_likes_by_cast_hash("cast_hash", None, None).await?;
//! ```
//!
//! 3. Use the delete_like function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Delete like on a cast
//! let likes = farcaster.delete_like_by_cast_hash("cast hash").await?;
//! ```
//!
//! 4. Use the get_recasters function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get recasters
//! let recasters = farcaster.get_recasters_by_cast_hash("cast hash", None, None).await?;
//! ```
//!
//! 2. Use the recast function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Recast
//! let recast = farcaster.recast_by_cast_hash("cast hash").await?;
//! ```
//!
//! 5. Use the delete_recast function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Delete recast
//! let delete = farcaster.delete_recast_by_cast_hash("cast hash").await?;
//! ```
//!
pub mod delete_like;
pub mod delete_recast;
pub mod get_likes;
pub mod get_recasters;
pub mod like_cast;
pub mod recast;
