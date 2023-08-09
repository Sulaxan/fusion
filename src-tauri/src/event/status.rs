use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tauri::AppHandle;
use tokio::sync::RwLock;

use crate::lcu::api::LcuApi;

use super::Emitter;

pub struct StatusEmitter {
    lcu_api: Arc<RwLock<LcuApi>>,
}

impl StatusEmitter {
    pub fn new(lcu_api: Arc<RwLock<LcuApi>>) -> Self {
        Self { lcu_api }
    }
}

#[async_trait]
impl Emitter for StatusEmitter {
    fn interval(&self) -> Duration {
        // Duration::from_secs(5 * 60)
        Duration::from_secs(5)
    }

    async fn emit(&self, handle: &AppHandle) {
        let api = self.lcu_api.read().await;
        let available = api.is_available().await;

        println!("available = {}", available);
    }
}
