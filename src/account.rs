use crate::constants::merkle::AUTH_TOKEN_DEFAULT_DURATION_SECS;
use crate::types::account::FarcasterAccount;
use chrono::Utc;
use ethers::signers::coins_bip39::English;
use ethers::signers::{LocalWallet, MnemonicBuilder};
use std::error::Error;

impl FarcasterAccount {
    /// create Farcaster account using mnemonic phrase
    pub fn from_mnemonic(mnemonic_phrase: &str) -> Self {
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic_phrase)
            .build()
            // TODO: avoid panics
            .expect("Wallet creation using mnemonic phrase failed");

        Self {
            wallet,
            bearer_token: None,
            session_token: None,
            token_duration_secs: Some(AUTH_TOKEN_DEFAULT_DURATION_SECS),
        }
    }

    /// create Farcaster account using private key
    pub fn from_private_key(key: &str) -> Self {
        let wallet = key
            .parse::<LocalWallet>()
            // TODO: avoid panics
            .expect("Wallet creation using private key failed");

        Self {
            wallet,
            bearer_token: None,
            session_token: None,
            token_duration_secs: Some(AUTH_TOKEN_DEFAULT_DURATION_SECS),
        }
    }

    /// returns authentication token
    pub async fn get_auth_token(&mut self) -> Result<&str, Box<dyn Error>> {
        // prepare session token, if not ready
        if self.session_token.is_none() {
            self.get_session_token().await?;
        }

        // check if token has expired
        let timestamp = Utc::now().timestamp_millis();
        if timestamp > self.session_token.as_ref().unwrap().expires_at {
            // refresh token
            self.bearer_token = None;
            self.get_session_token().await?;
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

    /// Revokes a specific session token
    pub async fn revoke_session_token(&mut self) -> Result<(), Box<dyn Error>> {
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
