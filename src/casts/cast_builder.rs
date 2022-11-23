use crate::constants::merkle::API_ROOT;
use crate::types::casts::published_cast::PublishedCast;
use crate::{CastBuilder, Farcaster};
use chrono::offset::Utc;
use serde_json::{json, Value};
use std::error::Error;

impl CastBuilder {
    // pub fn new() -> &'static mut Self {
    //     &mut CastBuilder {
    //         content: None,
    //         embeds: None,
    //         mentions: None
    //     }
    // }
    //
    // pub fn content(&mut self, content: &str) -> &mut Self {
    //     self.content = Some(content.to_string());
    //
    //     &mut self
    // }

    pub fn new() -> Self {
        CastBuilder {
            content: None,
            embeds: None,
            mentions: None,
        }
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn content<'a>(&'a mut self, content: &str) -> &'a mut Self {
        self.content = Some(content.to_string());

        self
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn embed<'a>(&'a mut self, embed: &str) -> Result<&'a mut Self, Box<dyn Error>> {
        if let Some(mut embeds) = self.embeds.clone() {
            if embeds.len() > 1 {
                return Err(Box::from("You cannot have more than two embeds in a cast"));
            }

            embeds.push(embed.to_string());
            self.embeds = Some(embeds);
        }
        else {
            self.embeds = Some(vec![embed.to_string()]);
        }

        Ok(self)
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn mention<'a>(&'a mut self, mention: &str) -> Result<&'a mut Self, Box<dyn Error>> {
        if let Some(mut mentions) = self.mentions.clone() {
            if mentions.len() > 4 {
                return Err(Box::from("You cannot have more than four mentions in a cast"));
            }

            mentions.push(mention.to_string());
            self.mentions = Some(mentions);
        }
        else {
            self.mentions = Some(vec![mention.to_string()]);
        }

        Ok(self)
    }

    pub async fn cast(&self, farcaster: &Farcaster) -> Result<PublishedCast, Box<dyn Error>> {
        let mut all_embeds: Vec<&String> = Vec::new();
        let mut all_mentions: Vec<&String> = Vec::new();

        let content = self.content.clone().unwrap_or("".to_string());

        if let Some(embeds) = &self.embeds {
            for i in embeds {
                all_embeds.push(i);
            }
        }

        if let Some(mentions) = &self.mentions {
            for i in mentions {
                all_mentions.push(i)
            }
        }


        let payload: Value = json!({
            "timestamp": Utc::now().timestamp_millis(),
            "text": content,
            "embeds": all_embeds,
            "mentions": all_mentions
        });

        let publish_cast_reqwest: String = reqwest::Client::new()
            .post(format!("{}/v2/casts", API_ROOT))
            .header("Content-Type", "application/json")
            .header("Authorization", &farcaster.account.session_token.as_ref().unwrap().secret)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let published_cast: PublishedCast = serde_json::from_str(&publish_cast_reqwest)?;

        Ok(published_cast)
    }
}
