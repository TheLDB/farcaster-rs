use crate::constants::merkle::API_ROOT;
use crate::types::assets::collection_owners::CollectionOwnersRoot;
use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    pub async fn get_collection_owners(
        &self,
        collection_id: &str,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<CollectionOwnersRoot, Box<dyn Error>> {
        let mut url = format!(
            "{}/v2/collection-owners?collectionid={}",
            API_ROOT, collection_id
        );

        if limit.is_some() {
            url.push_str(format!("&limit={}", limit.unwrap()).as_str())
        }

        if cursor.is_some() {
            url.push_str(format!("&cursor={}", cursor.unwrap()).as_str())
        }

        let collection_reqwest = &self.reqwest_get(&url).await?;

        let collections: CollectionOwnersRoot = serde_json::from_str(&collection_reqwest)?;

        Ok(collections)
    }
}
