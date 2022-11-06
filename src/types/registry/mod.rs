use ethers::core::abi::Abi;
use ethers::providers::{Http, Provider};
use std::collections::HashMap;

/// interface to Farcaster ID/Name Registry
#[derive(Debug)]
pub struct Registry {
    #[allow(dead_code)]
    pub(crate) fir_abi: Abi, // FIR JSON ABI
    #[allow(dead_code)]
    pub(crate) fir_block: u64, // FIR block to fetch from
    #[allow(dead_code)]
    pub(crate) fir: HashMap<String, String>, // FIR mapping
    pub(crate) fnr_abi: Abi,                 // FNR JSON ABI
    pub(crate) fnr_block: u64,               // FNR block to fetch from
    pub(crate) fnr: HashMap<String, String>, // FNR name to address mapping
    pub(crate) provider: Provider<Http>,     // Ethereum HTTP provider
}
