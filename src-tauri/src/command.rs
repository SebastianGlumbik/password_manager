pub mod authentication;
pub mod cloud;
pub mod database;
pub mod password;
pub mod totp;
pub mod validation;
pub mod window;

use super::*;
use crate::database::model::value::ToSecretString;
use crate::database::model::{value, Category, Content, Record, Value};
use crate::database::Database;
use secrecy::{ExposeSecret, SecretString};
use std::ops::Not;
use tauri::State;

/// Takes value from database and copies it to the clipboard.
/// # Error
/// If value cannot be copied to the clipboard
#[tauri::command]
pub async fn copy_value_to_clipboard<'a>(
    id: u64,
    database: State<'a, Database>,
    totp_manager: State<'a, TOTPManager>,
) -> Result<(), &'static str> {
    let content = database
        .get_content(id)
        .map_err(|_| "Failed to load content")?;

    let value = if let Value::TOTPSecret(_) = content.value() {
        let (code, _) = totp_manager
            .get_code(&id)
            .ok_or("Failed to get TOTP code")?;
        SecretString::new(code)
    } else {
        content.value().to_secret_string()
    };

    arboard::Clipboard::new()
        .map_err(|_| "Clipboard is not available")?
        .set_text(value.expose_secret())
        .map_err(|_| "Failed to copy value to clipboard")
}
