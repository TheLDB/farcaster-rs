use crate::Farcaster;

async fn auth_header() -> String {
    "aaaa".to_string()
}
impl Farcaster {
    pub async fn publish_cast(&self) -> Result<(), Box<dyn std::error::Error>> {
        let test = auth_header().await;
        println!("{}", test);
        Ok(())
    }
}