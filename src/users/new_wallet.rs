use ethers::{signers::{LocalWallet, MnemonicBuilder, coins_bip39::English, Wallet}, prelude::k256::ecdsa::SigningKey};

use crate::{Farcaster, types::wallet::key_type::KeyType};

impl Farcaster {
    pub async fn new_wallet(key_type: KeyType, key: String, mnemonic_word_count: Option<i64>) -> Result<Wallet<SigningKey>, Box<dyn std::error::Error>> {
        match key_type {
            KeyType::PrivateKey => {
                let wallet = key.parse::<LocalWallet>();
                match wallet {
                    Ok(success) => {
                        Ok(success)
                    }
                    Err(e) => {
                        Err(Box::from(format!("Unable to parse private key into wallet. Error: {:?}", e)))
                    }
                }

            }
            KeyType::MnemonicPhrase => {
                let word_count = mnemonic_word_count.unwrap_or(12);
                let word_count: usize = word_count.try_into().unwrap();
                let wallet = MnemonicBuilder::<English>::default()
                    .word_count(word_count)
                    .phrase(key.as_str())
                    .build();

                match wallet {
                    Ok(success) => {
                        Ok(success)
                    }
                    Err(e) => {
                        Err(Box::from(format!("Unable to build Mnemonic phrase into new wallet. Error: {:?}", e)))
                    }
                }
            }
        }
    }
}