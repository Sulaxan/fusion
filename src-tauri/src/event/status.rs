use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use async_trait::async_trait;
use log::info;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

use crate::lcu::api::LcuApi;

use super::Emitter;

pub const STATUS_EVENT_ID: &str = "lcu_status";

pub struct StatusEmitter {
    lcu_api: Arc<RwLock<Option<LcuApi>>>,
}

impl StatusEmitter {
    pub fn new(lcu_api: Arc<RwLock<Option<LcuApi>>>) -> Self {
        Self { lcu_api }
    }
}

#[async_trait]
impl Emitter for StatusEmitter {
    fn interval(&self) -> Duration {
        Duration::from_secs(25)
    }

    async fn emit(&self, handle: &AppHandle) -> Result<()> {
        let available;
        if let Some(api) = &*self.lcu_api.read().await {
            available = api.is_available().await;
        } else {
            available = false;
        }

        info!("Status = {}", available); 
        handle
            .emit_all(STATUS_EVENT_ID, available)
            .with_context(|| format!("in event emitter [{}]", STATUS_EVENT_ID))?;

        Ok(())
    }
}
