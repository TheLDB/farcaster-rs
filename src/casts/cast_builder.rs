use crate::constants::merkle::API_ROOT;
use crate::types::casts::published_cast::PublishedCast;
use crate::{CastBuilder, Farcaster};
use chrono::offset::Utc;
use serde_json::{json, Value};
use std::error::Error;

impl CastBuilder {
    pub fn new() -> Self {
        CastBuilder {
            content: "".to_string(),
            embeds: Vec::new(),
            mentions: Vec::new(),
        }
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn content<'a>(&'a mut self, content: &str) -> &'a mut Self {
        self.content = content.to_string();

        self
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn embed<'a>(&'a mut self, embed: &str) -> Result<&'a mut Self, Box<dyn Error>> {
        if self.embeds.len() > 1 {
            return Err(Box::from("You cannot have more than two embeds in a cast"));
        }

        self.embeds.push(embed.to_string());

        Ok(self)
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn mention<'a>(&'a mut self, mention: &str) -> Result<&'a mut Self, Box<dyn Error>> {
        if self.mentions.len() > 4 {
            return Err(Box::from(
                "You cannot have more than four mentions in a cast",
            ));
        }

        self.mentions.push(mention.to_string());

        Ok(self)
    }

    pub async fn cast(&self, farcaster: &Farcaster) -> Result<PublishedCast, Box<dyn Error>> {
        let payload: Value = json!({
            "timestamp": Utc::now().timestamp_millis(),
            "text": self.content,
            "embeds": self.embeds,
            "mentions": self.mentions
        });

        let publish_cast_reqwest: String = reqwest::Client::new()
            .post(format!("{}/v2/casts", API_ROOT))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                &farcaster.account.session_token.as_ref().unwrap().secret,
            )
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let published_cast: PublishedCast = serde_json::from_str(&publish_cast_reqwest)?;

        Ok(published_cast)
    }
}
