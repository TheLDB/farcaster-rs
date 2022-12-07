use ethers::{
    abi::{AbiEncode, Token, Tokenizable},
    types::U256,
    utils::parse_bytes32_string,
};

/// # Convert a FC V2 token to an fname (username)
///
/// ## Arguments
///
/// * `token: Token`
///     - Token is a type of ethers::abi::Token
///     - Retrieveable by parsing a log from the logs functions
///
pub fn token_to_fname(token: Token) -> Result<String, Box<dyn std::error::Error>> {
    let u256_token = U256::from_token(token)?;
    let u256_token = u256_token.encode();
    let byte_u256_token: &[u8] = &&u256_token;
    let byte_u256_token: &[u8; 32] = byte_u256_token[0..32].try_into()?;
    let fname = parse_bytes32_string(byte_u256_token)?;

    Ok(fname.to_string())
}
