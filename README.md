<div>
    <img alt="farcaster-rs" align="center" src="https://raw.githubusercontent.com/TheLDB/farcaster-rs/main/assets/banner.png" />
    <h1 align="center">farcaster-rs</h1>
     <h3 align="center">🚀 A simple & easy way to interface with <a href="https://farcaster.xyz">Farcaster</a> via <span style="color: #dea584;">Rust 🦀</span></h3>
     <p align="Center">Author: Landon Boles</p>
     <div align="center" style="display: flex; justify-content: center;">
        <a href="https://github.com/TheLDB" style="padding-right: 5px;">GitHub</a>
        <p> | </p>
        <a href="https://perl.xyz/profile/lndnnft" style="padding-right: 5px; padding-left: 5px;">Farcaster</a>
        <p> | </p>
        <a href="https://twitter.com/landon_xyz" style="padding-left: 5px; padding-right: 5px;">Bird App</a>
     </div>
</div>

<br />
<br />
<br />

# Credits

- [MistApproach](https://github.com/MistApproach)
  - Tons of various improvements & suggestions to build on top of the crate & prepare it for the hubs launch.

# 📜 Documentation

## For extensive documentation, visit our [docs.rs page](https://docs.rs/farcaster-rs/1.0.0/farcaster_rs/)

<br />

# 🚀 Getting Started

## Installation

To get started, add the farcaster_rs crate to your `Cargo.toml` file

```toml
farcaster_rs = "1.0.0"
```

Once you have the crate installed, you can start using the crate!

## Usage

To connect to and use Farcaster API you need Ethereum provider HTTP endpoint along with mnemonic phrase
or private key of an existing Farcaster account.

```rust
use farcaster_rs::{
  Farcaster,
  Account
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Initialize a new Account with a phrase/private key and an optional key duration (defaults to 1 hour)
  let account = Account::from_mnemonic("mnemonic phrase", None).await?;
  
  // Create a Farcaster connection
  let farcaster = Farcaster::new("eth node", account).await?;
  
  let casts = farcaster.get_casts_by_username("lndnnft", None, None).await?;
  
  println!("{:#?}", casts);
  
  Ok(())
}
```

<br />

# 🙏 Contributing

To start, I appreciate any and all contributions to the <span style="color: #dea584">farcaster-rs</span> repository!

There are 3 prefered things I'd like if you decide to contribute, however.

## 1. Ensure the issue/contribution is needed

If you spend your time building something, please ensure it's actually wanted/needed, this is best done by using the [Issues](https://github.com/TheLDB/farcaster-rs/issues) tab, and either viewing other discussions, or opening a new issue/discussion

## 2. Create a new branch for your contribution

Once you have validated the contribution, and forked the repo to your own GitHub account, please create a new branch to commit your code onto.

This can be done via the git CLI pretty easily:

```sh
$ git switch -c my_cool_feature
```

## 3. Create a detailed pull request, with documentation

I'd like to keep everything documented to make it as easy as possible for people looking to use the crate.

When opening a pull request, please ensure your function/contribution has been properly documented, and include good information about it in the PR. (use common sense)

Thanks so much!
