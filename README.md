# farcaster-rs
By Landon Boles [Github](https://github.com/TheLDB) | [Twitter](https://twitter.com/landon_xyz) | Farcaster: @lndnNFT

docs.rs link when finished

## Description
farcaster-rs is a rust crate built to interface & interact with the [Farcaster](https://farcaster.xyz) V2 Smart Contracts on Goerli, and their API

This project was built to:
- Get more familiar w/ Rust
- Build a fun project
- Have a nice way to interact w/ Farcaster for future projects

# Example Usage
There are extensive docs on the docs.rs page (link once finished)

# Get a user's info by their @
```rust
use farcaster_rs::Farcaster;

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{api-key-here}".to_string());
        // Root is a struct found in the types/user folder, the info from the API gets parsed into the Root struct, which holds a few other structs in the same file
        let user_info: Root = farcaster.get_user_by_username("dwr".to_string()).await.unwrap();
        println!("{:?}", user_info);
}
```

## Output:
```sh
Root {
    result: Result {
        user: User {
            address: "0x8773442740C17C9d0F0B87022c722F9a136206eD",
            username: "farcaster",
            display_name: "Farcaster",
            avatar: Avatar {
                url: "https://danromero.org/assets/fc_logo.jpg",
                is_verified: false,
            },
            follower_count: 203,
            following_count: 49,
            is_viewer_following: false,
            is_following_viewer: false,
            profile: Profile {
                bio: Bio {
                    text: "A sufficiently decentralized social network.",
                    mentions: [],
                },
            },
            referrer_username: "dwr",
            viewer_can_send_direct_casts: false,
        },
    },
}
```

# Get all Name Registry logs
```rust
#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{api-key-here}".to_string());
    
    let name_registry_logs: Vec<Log> = farcaster.get_name_registry_logs().await.unwrap();
    // Just grabbing first to show example of output, in real scenarios iterate over them
    let first: &Log = name_registry_logs.get(0).unwrap();
    println!("{:?}", first);
}
```

## Output:
```rust
Log { 
    address: 0xe3be01d99baa8db9905b33a3ca391238234b79d1, 
    topics: [
        0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef, 0x0000000000000000000000000000000000000000000000000000000000000000, 0x0000000000000000000000008773442740c17c9d0f0b87022c722f9a136206ed, 0x6661726361737465720000000000000000000000000000000000000000000000
    ],
    data: Bytes(0x),
    block_hash: Some(0x318c1013788674810b4cc15763afa9bbe5a2cd184a6a932a46bd64ba71b55684),
    block_number: Some(7648814),
    transaction_hash: Some(0x975f9e1c3cd38406df8e2c2458be8236d512d9784323bfbff077d2061e88f147), transaction_index: Some(52),
    log_index: Some(81),
    transaction_log_index: None,
    log_type: None,
    removed: Some(false)
}
```

# Parse a Name Registry Log to get the args
```rust
use farcaster_rs::abi::get_registry_abi::Registry; // Registry is an enum with NAME and ID for the different registry types
use farcaster_rs::Farcaster;
// * Enum, has Transfer & Register
use farcaster_rs::types::events::Events;

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{api-key-here}".to_string());
    let name_registry_logs = farcaster.get_name_registry_logs().await.unwrap();

    for i in name_registry_logs {
        let parsed_log = farcaster
            .parse_log(i, Registry::NAME, Events::Transfer)
            .await
            .unwrap();

        println!("{:?}", parsed_log);
    }
}
```

## Output:
```rust
Log { 
    params: [
        LogParam { name: "from", value: Address(0x0000000000000000000000000000000000000000) },
        LogParam { name: "to", value: Address(0xd64a3c00a7b25a4bce06a596fbdd2b9facdfda4e) },
        LogParam { name: "tokenId", value: Uint(48608429786950814485630709443268649182543529910706935975149071627325697687552)}
    ] 
}
```