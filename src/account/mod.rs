//! The Account module that handles authentication and tokens
//!
//! # Quickstart with the Account module
//!
//! 1. Use the from_mnemonic function as defined in the [Account struct](../types/account/struct.Account.html)
//! ```no_run
//! use farcaster_rs::Account;
//! // Takes a mnemonic phrase and a token duration (defaults to 1 hour)
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! ```
//!
//! 2. Use the from_private_key function as defined in the [Account struct](../types/account/struct.Account.html)
//! ```no_run
//! use farcaster_rs::Account;
//! // Takes a mnemonic phrase and a token duration (defaults to 1 hour)
//! let account: Account = Account::from_private_key("private key", None).await?;
//! ```
//!
//! 3. Regenerate your session token as defined in the [Account struct](../types/account/struct.Account.html)
//! ```no_run
//! use farcaster_rs::{Account, Farcaster};
//! // Takes a mnemonic phrase and a token duration (defaults to 1 hour)
//! let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
//! let mut farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
//!
//! // Regenerate your session token with an optional new token duration (defaults to either your previously set token duration, or the default 1 hour)
//! farcaster.account.regen_session_token(None);
//! ```

pub mod auth;

use crate::constants::merkle::AUTH_TOKEN_DEFAULT_DURATION_SECS;
use crate::types::account::Account;
use chrono::Utc;
use ethers::signers::coins_bip39::English;
use ethers::signers::{LocalWallet, MnemonicBuilder};
use std::error::Error;
use crate::types::account::auth::revoke::RevokedKeyRoot;

impl Account {
    /// Initialize an account with a mnemonic phrase
    ///
    /// # Params
    /// mnemonic_phrase: &str,
    /// token_duration: Option<i64> - Defaults to 1hr
    ///
    /// # Example
    /// ```no_run
    /// let account = farcaster_rs::Account::from_mnemonic("phrase", None).await?;
    /// ```
    pub async fn from_mnemonic(
        mnemonic_phrase: &str,
        token_duration: Option<i64>,
    ) -> Result<Self, Box<dyn Error>> {
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic_phrase)
            .build()
            .expect("Wallet creation using mnemonic phrase failed");

        let token_duration_secs = token_duration.unwrap_or(AUTH_TOKEN_DEFAULT_DURATION_SECS);

        let mut account = Self {
            wallet,
            bearer_token: None,
            session_token: None,
            token_duration_secs: Some(token_duration_secs),
        };

        // generate Bearer Token
        account.generate_bearer().await?;

        // generate Auth Token
        account.get_session_token().await?;

        Ok(account)
    }

    /// Initialize an account with a private key
    ///
    /// # Params:
    /// key: &str,
    /// token_duration: Option<i64> - Defaults to 1hr
    ///
    /// # Example
    /// ```no_run
    /// let account = farcaster_rs::Account::from_private_key("private key", None).await?;
    /// ```
    pub async fn from_private_key(
        key: &str,
        token_duration: Option<i64>,
    ) -> Result<Self, Box<dyn Error>> {
        let wallet = key
            .parse::<LocalWallet>()
            .expect("Wallet creation using private key failed");

        let token_duration_secs = token_duration.unwrap_or(AUTH_TOKEN_DEFAULT_DURATION_SECS);

        let mut account = Self {
            wallet,
            bearer_token: None,
            session_token: None,
            token_duration_secs: Some(token_duration_secs),
        };

        // generate Bearer Token
        account.generate_bearer().await?;

        // generate Auth Token
        account.get_session_token().await?;

        Ok(account)
    }

    /// Get your authentication token
    pub fn get_auth_token(&self) -> Result<&str, Box<dyn Error>> {
        // prepare session token, if not ready
        if self.session_token.is_none() {
            return Err(Box::from("Auth Token not ready"));
        }

        // check if token has expired
        let timestamp = Utc::now().timestamp_millis();
        if timestamp > self.session_token.as_ref().unwrap().expires_at {
            return Err(Box::from("Auth Token expired"));
        }

        Ok(&self.session_token.as_ref().unwrap().secret)
    }

    /// generate Merkle v2 API bearer token
    async fn generate_bearer(&mut self) -> Result<(), Box<dyn Error>> {
        self.bearer_token =
            Some(crate::Farcaster::generate_bearer(&self.wallet, self.token_duration_secs).await?);

        Ok(())
    }

    /// get Merkle v2 API session token
    async fn get_session_token(&mut self) -> Result<(), Box<dyn Error>> {
        // generate bearer token, if not ready
        if self.bearer_token.is_none() {
            self.generate_bearer().await?;
        }

        self.session_token =
            Some(crate::Farcaster::get_session_token(&self.bearer_token.as_ref().unwrap()).await?);

        Ok(())
    }

    /// Function to regenete your session token.
    /// Ideal for bots. Regenerate every hour to secure your token.
    pub async fn regen_session_token(
        &mut self,
        token_duration: Option<i64>,
    ) -> Result<(), Box<dyn Error>> {
        let token_duration_secs = token_duration.unwrap_or(
            self.token_duration_secs
                .unwrap_or(AUTH_TOKEN_DEFAULT_DURATION_SECS),
        );

        self.bearer_token =
            Some(crate::Farcaster::generate_bearer(&self.wallet, Some(token_duration_secs)).await?);

        self.session_token =
            Some(crate::Farcaster::get_session_token(&self.bearer_token.as_ref().unwrap()).await?);

        Ok(())
    }

    /// Revokes a specific session token
    pub async fn revoke_auth_token(&mut self) -> Result<RevokedKeyRoot, Box<dyn Error>> {
        // Return error if no session token is present
        if self.session_token.is_none() {
            return Err(Box::from("No session token present"));
        }

        let revoke =
            crate::Farcaster::revoke_session_token(&self.session_token.as_ref().unwrap()).await?;

        Ok(revoke)
    }
}
