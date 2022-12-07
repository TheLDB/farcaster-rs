//! The Assets module that handles getting related NFT assets
//!
//! # Quickstart with the Assets module
//!
//! 1. Use the get_user_collections function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // All three functions return the same info, just conveninent ways of fetching them
//! // Fetch the collections a user owns
//! let collections_username = farcaster.get_collections_by_username("lndnnft", None, None).await?;
//! let collections_fid = farcaster.get_collections_by_fid(2, None, None).await?;
//! let collections_address = farcaster.get_collections_by_address("0xef......", None, None).await?;
//! ```
//!
//! 2. Use the get_collection_owners function
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//!
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Returns all Farcaster collection owners of the ID specified
//! let collections_owners = farcaster.get_collection_owners("collectionID", None, None).await?;
//! ```

pub mod get_user_collections;
pub mod get_collection_owners;