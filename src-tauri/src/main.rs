// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use anyhow::{Context, Result};
use event::status::StatusEmitter;
use lcu::api::LcuApi;
use log::warn;
use tauri::AppHandle;
use tokio::sync::{Mutex, RwLock};

mod event;
mod lcu;

fn init_logger() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .init();
}

fn init_event_emitters(app_handle: Arc<AppHandle>, lcu_api: Arc<RwLock<Option<LcuApi>>>) {
    event::start(
        Arc::new(Mutex::new(StatusEmitter::new(lcu_api.clone()))),
        app_handle,
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .with_context(|| "Could not build tauri app")?;

    
    let lcu_api = match LcuApi::new() {
        Ok(api) => Arc::new(RwLock::new(Some(api))),
        Err(e) => {
            warn!("Could not create connection to LCU API (client not open?): {}", e);
            Arc::new(RwLock::new(None))
        }
    };
    let app_handle = Arc::new(app.handle());

    init_event_emitters(app_handle, lcu_api);

    // consumes app
    app.run(|_handle, _event| {});

    Ok(())
}
