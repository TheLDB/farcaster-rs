use crate::constants::merkle::API_ROOT;
use crate::types::casts::casts::{Cast, CastRoot};
use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    // fetches all Casts for a given `username` from newest to oldest
    // pub async fn get_casts_by_username(
    //     &self,
    //     username: &str,
    // ) -> Result<Vec<Cast>, Box<dyn std::error::Error>> {
    //     self.get_casts_by_username_with_limit(username, 0).await
    // }

    // V2 Implementation of the get casts w/ limit function
    pub async fn get_casts_by_fid_with_limit(
        &self,
        fid: u64,
        max_casts: usize,
    ) -> Result<Vec<Cast>, Box<dyn Error>> {
        let mut casts: Vec<Cast> = Vec::new();
        let mut fetch_url = format!("{}/v2/casts?fid={}", API_ROOT, fid);

        let read_limit = match max_casts {
            0 => usize::MAX,
            _ => max_casts,
        };

        while casts.len() < read_limit {
            let response = reqwest::Client::new()
                .get(&fetch_url)
                .header(
                    "Authorization",
                    &self.account.session_token.as_ref().unwrap().secret,
                )
                .send()
                .await?
                .text()
                .await?;

            // Weird edge case, literally only happens once throughout every post. Thanks.
            let response = response.replace(r#"\ud835"#, "\u{fffd}");

            let cast_root: Result<CastRoot, serde_json::Error> = serde_json::from_str(&response);

            match cast_root {
                Ok(cast) => {
                    casts.extend(cast.result.casts);

                    if let Some(next_url) = cast.next {
                        if let Some(cursor_url) = next_url.cursor {
                            fetch_url =
                                format!("{}/v2/casts?fid={}&cursor={}", API_ROOT, fid, cursor_url);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Err(e) => {
                    println!("{}", response);
                    println!("{}", e);
                }
            }
        }

        if max_casts > 0 {
            casts.truncate(max_casts);
        }

        Ok(casts)
    }

    // pub async fn get_casts_by_username_with_limit(
    //     &self,
    //     username: &str,
    //     max_casts: usize,
    // ) -> Result<Vec<Cast>, Box<dyn std::error::Error>> {
    //     let user = self.get_user_by_username(username).await?;
    //     let address = user.result.user.address;
    //
    //     let mut casts = Vec::new();
    //     let mut fetch_url = format!("https://api.farcaster.xyz/v1/profiles/{}/casts", address);
    //
    //     // when max_casts is zero, sky is the limit!
    //     let read_limit = match max_casts {
    //         0 => usize::MAX,
    //         _ => max_casts,
    //     };
    //
    //     while casts.len() < read_limit {
    //         // fetch CastRoot
    //         let response = reqwest::get(&fetch_url).await?.text().await?;
    //         let cast_root: CastRoot = serde_json::from_str(&response)?;
    //
    //         // store received casts
    //         casts.extend(cast_root.result.casts);
    //
    //         if let Some(next_url) = cast_root.meta.next {
    //             // update fetch_url
    //             fetch_url = next_url;
    //         } else {
    //             // no more casts available
    //             break;
    //         }
    //     }
    //
    //     // trim casts to max_casts
    //     if max_casts > 0 {
    //         casts.truncate(max_casts);
    //     }
    //
    //     Ok(casts)
    // }
}
