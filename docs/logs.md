# farcaster-rs Logs

What is the logs module?

Logs harbors all the functions that are associated with ID & Name registry events/logs on the new Farcaster V2 Goerli contracts

## Functions:
- [Get ID Registry Logs](/)
- [Get Name Registry Logs](/)
- [Parse a log to get the underlying args](/)


# Example Usage

## Get ID Registry Logs

Code:
```rust
use farcaster_rs::Farcaster;

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{key}".to_string());

    // In production/practice, don't use .unwrap(), check if the result is Ok first.
    let id_logs = farcaster.get_id_registry_logs().await.unwrap();

    for log in id_logs {
        println!("{:#?}", log);
    }
}
```

Output: 
```rust
Log {
    address: 0xda107a1caf36d198b12c16c7b6a1d1c795978c42,
    topics: [
        0x3cd6a0ffcc37406d9958e09bba79ff19d8237819eb2e1911f9edbce656499c87,
        0x0000000000000000000000008918a95630f16e28f0a8132b61beab0f160385f2,
        0x0000000000000000000000000000000000000000000000000000000000000a12,
    ],
    data: Bytes(0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000001a68747470733a2f2f7777772e6661726361737465722e78797a2f000000000000),
    block_hash: Some(
        0x98f45998fe07a98f05a766714ad66e192981ada6b8ae0e3278857c31c3c9aeaf,
    ),
    block_number: Some(
        7686868,
    ),
    transaction_hash: Some(
        0x1333521c70a7c43b4712c6f095115da81501673b34a3a243c6a2309819a14adb,
    ),
    transaction_index: Some(
        1,
    ),
    log_index: Some(
        1,
    ),
    transaction_log_index: None,
    log_type: None,
    removed: Some(
        false,
    ),
}
```

## Get Name Registry Logs

Code:
```rust
use farcaster_rs::Farcaster;

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{key}".to_string());

    // In production/practice, don't use .unwrap(), check if the result is Ok first.
    let name_logs = farcaster.get_name_registry_logs().await.unwrap();

    for log in name_logs {
        println!("{:#?}", log);
    }
}
```

Output: 
```rust
Log {
    address: 0xe3be01d99baa8db9905b33a3ca391238234b79d1,
    topics: [
        0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef,
        0x0000000000000000000000000000000000000000000000000000000000000000,
        0x0000000000000000000000008918a95630f16e28f0a8132b61beab0f160385f2,
        0x6365726573000000000000000000000000000000000000000000000000000000,
    ],
    data: Bytes(0x),
    block_hash: Some(
        0x98f45998fe07a98f05a766714ad66e192981ada6b8ae0e3278857c31c3c9aeaf,
    ),
    block_number: Some(
        7686868,
    ),
    transaction_hash: Some(
        0x1333521c70a7c43b4712c6f095115da81501673b34a3a243c6a2309819a14adb,
    ),
    transaction_index: Some(
        1,
    ),
    log_index: Some(
        2,
    ),
    transaction_log_index: None,
    log_type: None,
    removed: Some(
        false,
    ),
}
```

## Parse a log

Code:
```rust
use farcaster_rs::{Farcaster, types::{logs::events::Events, abi::registry::Registry}};

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{key}".to_string());

    // In production/practice, don't use .unwrap(), check if the result is Ok first.
    let name_logs = farcaster.get_name_registry_logs().await.unwrap();
    let log_one = name_logs.get(1).unwrap().to_owned();

    // Registry is an enum of either NAME or ID, depending on the event type, it's needed to parse the log
    // Events is also an enum of either Register or Transfer, and resembles which kind of transaction this is. In this case, it's a Transfer
    let parsed_log = farcaster.parse_log(log_one, Registry::NAME, Events::Transfer).await.unwrap();

    println!("{:#?}", parsed_log);
}
```

Output: 
```rust
Log {
    params: [
        LogParam {
            name: "from",
            value: Address(
                0x0000000000000000000000000000000000000000,
            ),
        },
        LogParam {
            name: "to",
            value: Address(
                0x4114e33eb831858649ea3702e1c9a2db3f626446,
            ),
        },
        LogParam {
            name: "tokenId",
            value: Uint(
                53372916132825433828052250902442082526116633556818697486937480128647458193408,
            ),
        },
    ],
}
```

