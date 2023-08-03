// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Result};

mod event;
mod lcu;

fn register_event_dispatchers(handle: tauri::AppHandle) {}

#[tokio::main]
async fn main() -> Result<()> {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .with_context(|| "Could not build tauri app")?;

    app.run(|_handle, _event| {});

    Ok(())
}
