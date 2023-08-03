pub mod api;

#[tauri::command]
pub async fn is_online() -> bool {
    //todo: determine from lcu api
    true
}
