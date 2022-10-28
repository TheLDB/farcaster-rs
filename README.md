<div align="center">
    <img src="https://raw.githubusercontent.com/TheLDB/farcaster-rs/main/docs/banner.png" >
    <h1 align="center">farcaster-rs</h1>
     <h3 align="center">🚀 A simple & easy way to interface with <a href="https://farcaster.xyz">Farcaster</a> via <span style="color: #dea584;">Rust 🦀</span></h1>
     <p>Author: Landon Boles</p>
     <div align="center" style="display: flex; justify-content: center;">
        <a href="https://github.com/TheLDB" style="padding-right: 5px;">GitHub</a>
        <p> | </p>
        <a href="farcaster://profiles/0x23D994eb22A23F57088D72a421B46Fb5Fe5Ae1fa/posts" style="padding-left: 5px; padding-right: 5px;">Farcaster</a>
        <p> | </p>
        <a href="https://twitter.com/landon_xyz" style="padding-left: 5px; padding-right: 5px;">Bird App</a>
     </div>
</div>

<br />
<br />
<br />

# 📜 Documentation

## For extensive documentation, visit our [docs.rs page](https://docs.rs/farcaster-rs/0.1.0-pre.3/farcaster_rs/)

Otherwise, check out the [docs](https://github.com/TheLDB/farcaster-rs/tree/main/docs) folder for examples on how to use functions from each method.

<br />

# 🚀 Getting Started

## Installation

To get started, add the farcaster_rs crate to your `Cargo.toml` file

```toml
farcaster_rs = "0.1.0-pre.3"
```

Once you have the crate installed, you can start using the crate!

## Usage

In your `main.rs` file, set up a new Farcaster struct using the `::new(client: String)` method.

```rust
use farcaster_rs::Farcaster;

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/key".to_string());

    let landon = farcaster.get_user_by_username("lndnnft".to_string()).await.unwrap();

    println!("{:#?}", landon);
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
