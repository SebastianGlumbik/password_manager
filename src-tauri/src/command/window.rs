use super::*;
use crate::window::*;

/// Window types that can be created.
#[derive(Clone, serde::Serialize)]
pub enum WindowType {
    Login,
    Register,
    Main,
}

/// Creates specific window based on the database state and returns the window type.
#[tauri::command]
pub async fn initialize_window<'a>(app_handle: AppHandle) -> tauri::Result<WindowType> {
    if app_handle.try_state::<Database>().is_some() {
        create_main_window(app_handle)?;
        Ok(WindowType::Main)
    } else if Database::exists(app_handle.clone()) {
        create_login_window(app_handle)?;
        Ok(WindowType::Login)
    } else {
        create_register_window(app_handle)?;
        Ok(WindowType::Register)
    }
}
