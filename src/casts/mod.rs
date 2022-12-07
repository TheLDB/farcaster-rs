//! The Casts module that handles protocol casts
//!
//! # Quickstart with the Casts module
//!
//! 1. Use the publish_cast function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Publish a cast.
//! // 1st Param: Content
//! // 2nd & 3rd Param: Parent Cast Hash & Parent User FID
//!     // Used for replying to a cast
//! let cast = farcaster.publish_cast("cast content", Some("optional parent hash to reply to"), Some(0)).await?;
//! ```
//!
//! //! 2. Use the get_casts function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Get the casts of a user.
//! // All three functions return the same thing, just different methods of fetching
//! // 1st Param: FID/username/address
//! // 2nd Param: Optional Limit of how many to fetch (max 100)//!
//! // 3rd Param: Optional cursor for pagination
//! let casts = farcaster.get_casts_by_fid(0, None, None).await?;
//! let casts = farcaster.get_casts_by_username("dwr", None, None).await?;
//! let casts = farcaster.get_casts_by_address("0x000...", None, None).await?;
//! ```
//!
//! //! //! 3. Use the delete_cast function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! let delete_cast = farcaster.delete_cast_by_cast_hash("cast hash").await?;
//! ```

pub mod delete_cast;
pub mod get_casts;
/// Publish a cast to the protocol
/// ```no_run
/// use farcaster_rs::{Account, Farcaster};
///
/// let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
/// let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
///
/// // Publish a cast.
/// // 1st Param: Content
/// // 2nd & 3rd Param: Parent Cast Hash & Parent User FID
///     // Used for replying to a cast
/// let cast = farcaster.publish_cast("cast content", Some("optional parent hash to reply to"), Some(0)).await?;
/// ```
pub mod publish_cast;
