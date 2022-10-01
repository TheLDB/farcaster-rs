use serde::{Deserialize, Serialize};

/// # KeyType Enum
/// 
/// When initializing a wallet to do multiple authorized actions such as publishing a cast (WIP), you need to create a wallet with a private key
/// 
/// When creating a new wallet, you use a KeyType value, which has two options:
/// 
/// * `PrivateKey`
///     - PrivateKey is going to be less common, but still supported. Just pass this in with your private key
/// 
/// * `MnemonicPhrase`
///     - Mnemonic Phrases are the 12-24 word keys most commonly used to retrieve your ethereum wallet/account.
#[derive(Debug, Deserialize, Serialize)]
pub enum KeyType {
    PrivateKey,
    MnemonicPhrase,
}