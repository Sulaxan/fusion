use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tauri::AppHandle;
use tokio::{
    sync::Mutex,
    time::{self},
};

pub mod status;

#[async_trait]
pub trait Emitter: Sync + Send {
    /// The interval for how often the event should be emitted.
    fn interval(&self) -> Duration;
    /// Called when an event should emit.
    async fn emit(&self, handle: &AppHandle);
}

/// Starts an emitter by assigning it a new thread.
pub fn start<'a>(emitter: Arc<Mutex<dyn Emitter>>, app_handle: Arc<AppHandle>) {
    tokio::task::spawn(async move {
        let e = emitter.lock().await;
        let mut interval = time::interval(e.interval());
        drop(e);

        loop {
            interval.tick().await;
            let e = emitter.lock().await;
            e.emit(&app_handle).await;
            drop(e);
        }
    });
}
