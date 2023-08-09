use anyhow::Result;
use shaco::rest::RESTClient;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("LCU Rest Client Error: {msg}")]
pub struct RestClientError {
    msg: String,
}

pub struct LcuApi {
    client: RESTClient,
}

impl LcuApi {
    pub fn new() -> Result<Self> {
        Ok(LcuApi {
            client: RESTClient::new().map_err(|e| RestClientError { msg: e.to_string() })?,
        })
    }

    pub async fn is_available(&self) -> bool {
        // we assume any error means the api isn't available
        self.client.post("/Help".to_string(), {}).await.is_ok()
    }
}
