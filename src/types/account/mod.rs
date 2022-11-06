/// ## Auth
/// Holds authentication structs
pub mod auth;

use auth::bearer::Bearer;
use auth::secret::SecretToken;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::signers::Wallet;

/// Farcaster account
/// holds account keypair and Merkle v2 API token
#[derive(Debug)]
pub struct FarcasterAccount {
    pub(crate) wallet: Wallet<SigningKey>,
    pub(crate) bearer_token: Option<Bearer>,
    pub(crate) session_token: Option<SecretToken>,
    pub(crate) token_duration_secs: Option<i64>,
}
