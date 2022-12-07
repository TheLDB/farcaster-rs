//! Contains all of the types used throughout farcaster-rs

/// ## Casts
/// Holds structs for cast info, casts, etc...
pub mod casts;
/// ## User
/// Holds structs for user info, notifications, etc...
pub mod user;
/// ## Wallet
/// Holds structs for wallets
pub mod wallet;

/// ## Account
/// Holds Farcaster Account
pub mod account;
/// ## Registry
/// Holds Farcaster ID/Name Registry
pub mod registry;

/// ## Reactions
/// Holds types for reaction events
pub mod reactions;

/// ## Follows
/// Holds types for followers/following
pub mod follows;

/// ## Shared
/// Contains shared types i.e. Profile, Pfp, etc....
pub mod shared;