// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use anyhow::{Context, Result};
use event::status::StatusEmitter;
use lcu::api::LcuApi;
use tokio::sync::{Mutex, RwLock};

mod event;
mod lcu;

fn register_event_dispatchers(handle: tauri::AppHandle) {}

#[tokio::main]
async fn main() -> Result<()> {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .with_context(|| "Could not build tauri app")?;

    // test
    // let lcu_api = Arc::new(RwLock::new(LcuApi::new()?));
    // let app_handle = Arc::new(app.handle());
    // event::start(
    //     Arc::new(Mutex::new(StatusEmitter::new(lcu_api.clone()))),
    //     app_handle,
    // );

    // consumes app
    app.run(|_handle, _event| {});

    Ok(())
}
