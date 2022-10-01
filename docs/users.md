# farcaster-rs Users Documentation

What is the users module?

Users harbors all the functions that are associated with user function, from getting casts to getting a users info, etc...

## Functions:
- [Get all casts of a user w/ Pagination](#get-all-casts-of-a-user)
- [Get a users info](#get-a-users-info)
- [Get the verified address of a user](#get-the-verified-address-of-a-user)
- [Get the followers of a user](#get-the-followers-of-a-user)

# Example Usage

## Get all casts of a user
With pagination!

Code:
```rust
use farcaster_rs::Farcaster;

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{key}".to_string());

    // get_casts takes 3 parameters. Username (String), the # of casts per page, and the page number.
    // In this example, we are fetching all casts from @ace (Alex Kwon, co-founder of https://perl.xyz!), and getting one cast per page, and we're getting the first page.
    let ace_casts = farcaster.get_casts("ace".to_string(), 1, 0).await.unwrap();

    println!("{:#?}", ace_casts);
}
```

Output:
```rust
Root {
    result: Result {
        casts: [
            Cast {
                body: Body {
                    type_field: "text-short",
                    published_at: 1664549609335,
                    sequence: 4944,
                    address: "0x71503b1309D6797bC4C4FbfDE973aDc2C191834b",
                    username: "ace",
                    data: Data {
                        text: "your turn now 🔥 you can @perl any posts you’d like to save - got you off the waitlist",
                        reply_parent_merkle_root: Some(
                            "0x6e34fa6387565b69f85dad7032ede6124197b9895905252b9ab1c5b787f8b77b",
                        ),
                    },
                    prev_merkle_root: "0xf3246370c6b43fe8c061fc62e5f38773fb821de5b32277bc566680c0930a7842",
                },
                attachments: Attachments {
                    open_graph: Some(
                        [],
                    ),
                },
                signature: "0xc48660fa98243261d0630d006b137d5d1ce1dcf6f18c069d67ed5083fd5cb12d2390c76974b02724894976b39c24f7d634edc6b02ef02cf205a67e5fb9d3983f1b",
                merkle_root: "0x3d3108d846dd3e383227d12c425f2ed50d0963c1dc0d8af966f9d489d3289467",
                thread_merkle_root: "0x4535bc3c182da0bae736c2bd75dc524d23556515aa3851e7d6d33ffe73e25097",
                meta: Meta {
                    display_name: "Alex Kwon",
                    avatar: "https://lh3.googleusercontent.com/UfvVA_GoDMbI2o9Op7m5XcoFEkGY4pWSJepN5iC0z-Bq2xIzzRv6aeBDZizUtMoDAsloXE71TTiwV_4uQ4UTrjeo3-f1WLWdmbdKmQ",
                    is_verified_avatar: true,
                    mentions: Some(
                        [
                            Mention {
                                address: Some(
                                    "0xC4c4dB549D13EBd6A9805a42Ef6A3e591aF6AA2f",
                                ),
                                username: Some(
                                    "perl",
                                ),
                            },
                        ],
                    ),
                    num_reply_children: 0,
                    reactions: Reactions {
                        count: 0,
                        type_field: "Like",
                        self_field: false,
                    },
                    recasters: [],
                    recasts: Recasts {
                        count: 0,
                        self_field: false,
                    },
                    watches: Watches {
                        count: 0,
                        self_field: false,
                    },
                    reply_parent_username: Some(
                        ReplyParentUsername {
                            address: "0x3829fb26E00b4acB38221dE932C0D844F851b2A3",
                            username: "missjenny",
                        },
                    ),
                    recast: None,
                },
            },
        ],
    },
    meta: Meta2 {
        next: "https://api.farcaster.xyz/v1/profiles/0x71503b1309D6797bC4C4FbfDE973aDc2C191834b/casts?cursor=eyJwYWdlIjoxLCJwZXJQYWdlIjoxLCJmYXJjYXN0ZXJBZGRyZXNzIjoiMHg3MTUwM2IxMzA5RDY3OTdiQzRDNEZiZkRFOTczYURjMkMxOTE4MzRiIiwidmlld2VyQWRkcmVzcyI6IiIsImluY2x1ZGVEZWxldGVkQ2FzdHMiOmZhbHNlfQ",
    },
}
```

## Get a users info

Code:
```rust
use farcaster_rs::Farcaster;

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{key}".to_string());

    // get_user_by_username takes 1 parameter, a username in the form of a String
    // In this example, we are fetching the profile of @dwr (Dan Romero, he's part of the Farcaster team!)
    let dwr = farcaster.get_user_by_username("dwr".to_string()).await.unwrap();

    println!("{:#?}", dwr);
}
```

Output:
```rust
Root {
    result: Result {
        user: User {
            address: "0x74232BF61e994655592747E20bDF6fa9B9476f79",
            username: "dwr",
            display_name: "Dan Romero",
            avatar: Avatar {
                url: "https://res.cloudinary.com/merkle-manufactory/image/fetch/c_fill,f_png,w_256/https://lh3.googleusercontent.com/MyUBL0xHzMeBu7DXQAqv0bM9y6s4i4qjnhcXz5fxZKS3gwWgtamxxmxzCJX7m2cuYeGalyseCA2Y6OBKDMR06TWg2uwknnhdkDA1AA",
                is_verified: true,
            },
            follower_count: 2486,
            following_count: 2560,
            is_viewer_following: false,
            is_following_viewer: false,
            profile: Profile {
                bio: Bio {
                    text: "Working on Farcaster nf.td/dwr",
                    mentions: [],
                },
            },
            referrer_username: Some(
                "farcaster",
            ),
            viewer_can_send_direct_casts: false,
        },
    },
}
```

## Get the verified address of a user

Code:
```rust
use farcaster_rs::Farcaster;

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{key}".to_string());

    // get_verified_address takes 1 parameter, a username in the form of a String
    // In this example, we are fetching the connected address of @jayme (https://maindrop.xyz, https://nf.td/jayme)
    let jayme_address = farcaster.get_verified_address("jayme".to_string()).await.unwrap();

    println!("{:#?}", jayme_address);
}
```

Output:
```rust
Root {
    result: Result {
        verified_addresses: [
            VerifiedAddress {
                signed_message: "0x594c77f67ea6bee881aedac641ba1ce2a7c77db72d2a3510322babe9febec5454609c38065148be1789170a7ff5a10f6964dbe3ead8c9e19db8ebc5dcb59dec01b",
                signer_address: "0x9EAB9D856a3a667dc4CD10001D59c679C64756E7",
                farcaster_address: "0x1Ca66c990E86B750eA6B2180d17fFf89273A5c0d",
                original_message: "Connected Address: 0x9EAB9D856a3a667dc4CD10001D59c679C64756E7\n\nFarcaster Address: 0x6E97105278801721f7F3e0d995e5F47AA4309023",
            },
        ],
    },
}
```

## Get the followers of a user

Code:
```rust
use farcaster_rs::Farcaster;

#[tokio::main]
async fn main() {
    let farcaster = Farcaster::new("https://goerli.infura.io/v3/{key}".to_string());

    // get_followers takes 2 Optional parameters, either a username (known as an fname on FC), or an Ethereum address
    // In this example, we are fetching the followers of @dwr (https://danromero.org, https://farcaster.xyz)
    let followers = farcaster.get_followers(Some("dwr".to_string()), None).await.unwrap();

    for i in followers {
        println!("{:#?}", i);
    }
}
```

Output:
```rust
Follower {
    address: "0x6cd21149041F81d640C2A292402b2318Cbf5aD04",
    username: "shilpa",
    display_name: Some(
        "shilpa",
    ),
    avatar: Avatar {
        url: Some(
            "https://lh3.googleusercontent.com/DEUlOy0Z3GKRWUoRSfAk2w9-nnysYmyZa6-7-m07r-O9LePeTh0SVwGM-uuPzHFh14vtsnxQ5iptbiPoyZmbpTFaBrWACyOAd26XWQ",
        ),
        is_verified: Some(
            true,
        ),
    },
    is_viewer_following: false,
    verifications: [],
}
```