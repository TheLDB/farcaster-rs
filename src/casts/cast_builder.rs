use std::error::Error;
use crate::{CastBuilder, Farcaster};
use chrono::offset::Utc;
use serde_json::{Value, json};
use crate::constants::merkle::API_ROOT;
use crate::types::casts::published_cast::PublishedCast;

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
            mentions: None
        }
    }

    pub fn content(&self, content: &str) -> Self {
        let embeds = self.embeds.clone();
        let mentions = self.mentions.clone();

        CastBuilder {
            content: Some(content.to_string()),
            embeds,
            mentions,
        }
    }

    pub fn embed(&self, embed: &str) -> Result<Self, Box<dyn Error>> {
        let content = self.content.clone();
        let mentions = self.mentions.clone();

        let mut new_embeds: Vec<String> = Vec::new();

        if let Some(embeds) = self.embeds.clone() {
            println!("{}", embeds.len());
            for i in embeds {
                new_embeds.push(i);
            }
        }


        if new_embeds.len() > 1 {
            Err(Box::from("You cannot have more than two embeds"))
        }
        else {
            new_embeds.push(embed.to_string());

            Ok(CastBuilder {
                content,
                embeds: Some(new_embeds),
                mentions
            })
        }
    }

    pub fn mention(&self, mention: &str) -> Result<Self, Box<dyn Error>> {
        let content = self.content.clone();
        let embeds = self.embeds.clone();

        let mut new_mentions: Vec<String> = Vec::new();

        if let Some(mentions) = self.mentions.clone() {
            for i in mentions {
                new_mentions.push(i)
            }
        }

        if new_mentions.len() > 4 {
            Err(Box::from("You cannot have more than five embeds"))
        }
        else {
            Ok(CastBuilder {
                content,
                mentions: Some(new_mentions),
                embeds
            })
        }

    }

    pub async fn cast(&self, farcaster: &Farcaster) -> Result<PublishedCast, Box<dyn Error>> {
        let text = self.content.clone().unwrap();
        let embeds = self.embeds.clone().unwrap();
        let mentions = self.mentions.clone().unwrap();

        let payload: Value = json!({
            "timestamp": Utc::now().timestamp_millis(),
            "text": text,
            "embeds": embeds,
            "mentions": mentions,
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

        println!("{:#?}", published_cast);
        Ok(published_cast)
    }
}
