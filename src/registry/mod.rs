mod token_to_fname;

use crate::constants::registry::{
    FIR_CONTRACT_ADDRESS, FIR_DEPLOYMENT_BLOCK, FNR_CONTRACT_ADDRESS, FNR_DEPLOYMENT_BLOCK,
};
use crate::types::registry::{FIDInfo, Registry};
use ethers::abi::RawLog;
use ethers::providers::Middleware;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, Filter, Log, H256};
use std::collections::HashMap;
use std::error::Error;
use token_to_fname::token_to_fname;

impl Registry {
    /// create new Registry
    /// provider: Ethereum HTTP provider
    pub async fn new(provider: &str) -> Result<Self, Box<dyn Error>> {
        let mut registry = Self {
            fir_abi: serde_json::from_str(include_str!("json/IdRegistryV2.json"))
                .expect("Farcaster ID Registry ABI parse error"),
            fir_block: FIR_DEPLOYMENT_BLOCK,
            fir: HashMap::new(),
            fnr_abi: serde_json::from_str(include_str!("json/NameRegistryV2.json"))
                .expect("Farcaster Name Registry ABI parse error"),
            fnr_block: FNR_DEPLOYMENT_BLOCK,
            fnr: HashMap::new(),
            provider: Provider::<Http>::try_from(provider).expect("Ethereum provider error"),
        };

        // commence initial sync
        registry.sync().await?;

        Ok(registry)
    }

    /// get fid assigned to `address`
    /// return None if not found
    pub fn get_fid_by_address(&self, address: &str) -> Option<u64> {
        for (k, v) in &self.fir {
            if v.address == address {
                return Some(*k);
            }
        }

        None
    }

    /// get fid assigned to `username`
    /// return None if not found
    pub fn get_fid_by_username(&self, username: &str) -> Option<u64> {
        if let Some(address) = self.get_address_by_username(username) {
            return self.get_fid_by_address(address);
        }

        None
    }

    /// get address assigned to `username`
    /// returns None if not found
    pub fn get_address_by_username(&self, username: &str) -> Option<&str> {
        if let Some(address) = self.fnr.get(username) {
            return Some(address);
        }

        None
    }

    /// get address assigned to `fid`
    /// returns None if not found
    pub fn get_address_by_fid(&self, fid: u64) -> Option<&str> {
        if let Some(fid_info) = self.fir.get(&fid) {
            return Some(&fid_info.address);
        }

        None
    }

    /// get username assigned to `fid`
    /// returns None if not found
    pub fn get_username_by_fid(&self, fid: u64) -> Option<&str> {
        if let Some(address) = self.get_address_by_fid(fid) {
            return self.get_username_by_address(address);
        }

        None
    }

    /// get username assigned to `address`
    /// returns None if not found
    pub fn get_username_by_address(&self, address: &str) -> Option<&str> {
        for (k, v) in &self.fnr {
            if v == address {
                return Some(k);
            }
        }

        None
    }

    /// sync Farcaster ID/Name Registry
    pub async fn sync(&mut self) -> Result<(), Box<dyn Error>> {
        // sync FIR
        self.sync_fir_logs().await?;

        // sync FNR
        self.sync_fnr_logs().await?;

        Ok(())
    }

    /// [private] sync Farcaster ID Registry logs
    /// TODO: parse "Transfer" logs
    async fn sync_fir_logs(&mut self) -> Result<(), Box<dyn Error>> {
        // get latest event logs
        let logs = self.get_fir_register_logs(self.fir_block).await?;

        // iterate over logs
        for log in logs {
            // update stored block number
            let block_number = u64::try_from(log.block_number.unwrap()).unwrap();
            if block_number > self.fir_block {
                self.fir_block = block_number;
            }

            // parse log
            let raw_log = RawLog {
                topics: log.topics,
                data: log.data.to_vec(),
            };
            let log_desc = self.fir_abi.event("Register")?.parse_log(raw_log)?;

            // extract address, fid, recovery and url
            let address = log_desc.params.get(0).unwrap().value.to_string();
            let fid = log_desc
                .params
                .get(1)
                .unwrap()
                .value
                .clone()
                .into_uint()
                .unwrap()
                .as_u64();
            let recovery = log_desc.params.get(2).unwrap().value.to_string();
            let url = log_desc.params.get(3).unwrap().value.to_string();

            self.fir.insert(
                fid,
                FIDInfo {
                    address,
                    recovery,
                    url,
                },
            );
        }

        Ok(())
    }

    /// [private] sync Farcaster Name Registry logs
    async fn sync_fnr_logs(&mut self) -> Result<(), Box<dyn Error>> {
        // get latest event logs
        let logs = self.get_fnr_transfer_logs(self.fnr_block).await?;

        // iterate over logs
        for log in logs {
            // update stored block number
            let block_number = u64::try_from(log.block_number.unwrap()).unwrap();
            if block_number > self.fnr_block {
                self.fnr_block = block_number;
            }

            // parse log
            let raw_log = RawLog {
                topics: log.topics,
                data: log.data.to_vec(),
            };
            let log_desc = self.fnr_abi.event("Transfer")?.parse_log(raw_log)?;

            // extract address and username
            let address = log_desc.params.get(1).unwrap().value.to_string();
            let username = token_to_fname(log_desc.params.get(2).unwrap().value.clone())?;

            // insert (or update) FNR HashMap
            self.fnr.insert(username, address);
        }

        Ok(())
    }

    /// [private] get Farcaster ID Registry Register logs starting at block `from_block`
    async fn get_fir_register_logs(&mut self, from_block: u64) -> Result<Vec<Log>, Box<dyn Error>> {
        // prepare contract address and log topic
        let contract_address = FIR_CONTRACT_ADDRESS.parse::<Address>()?;
        let register_topic =
            "0x3cd6a0ffcc37406d9958e09bba79ff19d8237819eb2e1911f9edbce656499c87".parse::<H256>()?;

        // prepare filter
        let register_filter = Filter::new()
            .select(from_block..)
            .address(contract_address)
            .topic0(register_topic);

        // get event logs
        let logs = self.provider.get_logs(&register_filter).await?;

        Ok(logs)
    }

    /// [private] get Farcaster ID Registry Transfer logs starting at block `from_block`
    #[allow(dead_code)]
    async fn get_fir_transfer_logs(&mut self, from_block: u64) -> Result<Vec<Log>, Box<dyn Error>> {
        // prepare contract address and log topic
        let contract_address = FIR_CONTRACT_ADDRESS.parse::<Address>()?;
        let transfer_topic =
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".parse::<H256>()?;

        // prepare filter
        let transfer_filter = Filter::new()
            .select(from_block..)
            .address(contract_address)
            .topic0(transfer_topic);

        // get event logs
        let logs = self.provider.get_logs(&transfer_filter).await?;

        Ok(logs)
    }

    /// [private] get Farcaster Name Registry Transfer logs starting at block `from_block`
    async fn get_fnr_transfer_logs(&mut self, from_block: u64) -> Result<Vec<Log>, Box<dyn Error>> {
        // prepare contract address and log topic
        let contract_address = FNR_CONTRACT_ADDRESS.parse::<Address>()?;
        let transfer_topic =
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".parse::<H256>()?;

        // prepare transfer filter
        let transfer_filter = Filter::new()
            .select(from_block..)
            .address(contract_address)
            .topic0(transfer_topic);

        // get event logs
        let logs = self.provider.get_logs(&transfer_filter).await?;

        Ok(logs)
    }
}
