use super::*;

//TODO Cloud
#[tauri::command]
pub async fn cloud() -> Result<String, &'static str> {
    Ok("Not implemented".to_string())
}
