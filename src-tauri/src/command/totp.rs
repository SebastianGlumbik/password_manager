use super::*;

/// Returns a TOTP code based on content id.
/// # Error
/// Returns error when TOTP is not loaded into the TOTP manager or TOTP code cannot be generated
#[tauri::command]
pub async fn get_totp_code<'a>(
    id: u64,
    totp_manager: State<'a, TOTPManager>,
) -> Result<(String, u64), &'static str> {
    totp_manager.get_code(&id).ok_or("Failed to get TOTP code")
}
