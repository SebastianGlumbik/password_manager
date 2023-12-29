use super::*;

/// Register function for the frontend.
/// - Compares the passwords
/// - Checks if the database exists
/// - Creates the database
#[tauri::command]
pub fn register(
    password: &str,
    confirm_password: &str,
    connection: State<DatabaseConnection>,
    app_handle: tauri::AppHandle,
    window: tauri::Window,
) -> Result<(), &'static str> {
    if password != confirm_password {
        return Err("Passwords do not match.");
    }

    if database_exists(app_handle.clone()).is_some() {
        tauri::api::dialog::message(
            Some(&window),
            "Error",
            "Database already exists, please restart the application",
        );
        app_handle.restart();
    }

    connect_database(password, connection, app_handle)?;
    window
        .eval("window.location.href = '/src/html/main.html'")
        .map_err(|_| "Failed to redirect")
}
