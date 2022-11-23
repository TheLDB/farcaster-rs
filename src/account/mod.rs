pub mod auth;

use crate::constants::merkle::AUTH_TOKEN_DEFAULT_DURATION_SECS;
use crate::types::account::Account;
use chrono::Utc;
use ethers::signers::coins_bip39::English;
use ethers::signers::{LocalWallet, MnemonicBuilder};
use std::error::Error;

impl Account {
    /// create Farcaster account using mnemonic phrase
    pub async fn from_mnemonic(mnemonic_phrase: &str, token_duration: Option<i64>) -> Result<Self, Box<dyn Error>> {
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

    /// create Farcaster account using private key
    pub async fn from_private_key(key: &str, token_duration: Option<i64>) -> Result<Self, Box<dyn Error>> {
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

    /// returns authentication token
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

    pub async fn regen_session_token(&mut self) -> Result<(), Box<dyn Error>> {
        self.bearer_token = Some(crate::Farcaster::generate_bearer(&self.wallet, self.token_duration_secs).await?);

        self.session_token = Some(crate::Farcaster::get_session_token(&self.bearer_token.as_ref().unwrap()).await?);

        Ok(())
    }

    /// Revokes a specific session token
    pub async fn revoke_auth_token(&mut self) -> Result<(), Box<dyn Error>> {
        // Return error if no session token is present
        if self.session_token.is_none() {
            return Err(Box::from("No session token present"));
        }

        let _revoke =
            crate::Farcaster::revoke_session_token(&self.session_token.as_ref().unwrap()).await?;

        // Structure not build yet, therefore function not built yet.
        Ok(())
    }
}
