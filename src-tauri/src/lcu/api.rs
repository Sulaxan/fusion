use anyhow::Result;
use shaco::rest::RESTClient;

pub struct LcuApi {
    client: RESTClient,
}

impl LcuApi {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(LcuApi { client: RESTClient::new()? })
    }

    pub async fn is_available(&self) -> bool {
        // we assume any error means the api isn't available
        self.client.post("/Help".to_string(), {}).await.is_ok()
    }
}
