# farcaster-rs ABI Documentation

What is the ABI module?

ABI harbors all the functions that are associated with getting the ABI JSON for the Farcaster V2 Goerli contracts

## Functions:
- [Get a Registry ABI]()


# Example Usage

## Get a registry ABI

Code:
```rust
use farcaster_rs::{Farcaster, types::abi::registry::Registry};

#[tokio::main]
async fn main() {
    let abi = Farcaster::get_registry_abi(Registry::NAME).unwrap();

    println!("{}", abi);
}
```

Output: 

The JSON is way too long for this markdown document, however it is stored in [here](../json/NameRegistryV2.json)