use super::*;

/// Login function for the frontend.
/// - Checks if the database exist
/// - Opens the database
#[tauri::command]
pub fn login(
    password: &str,
    connection: State<DatabaseConnection>,
    app_handle: tauri::AppHandle,
    window: tauri::Window,
) -> Result<(), &'static str> {
    if database_exists(app_handle.clone()).is_none() {
        return Err("Database does not exist, please start over");
    }

    connect_database(password, connection, app_handle)?;
    window
        .eval("window.location.href = '/src/html/main.html'")
        .map_err(|_| "Failed to redirect")
}

/// Deletes the database file and restarts the application.
#[tauri::command]
pub fn start_over(app_handle: tauri::AppHandle) -> Result<(), &'static str> {
    if let Some(path_buf) = database_exists(app_handle.clone()) {
        fs::remove_file(path_buf).map_err(|_| "Failed to delete database")?;
    }
    app_handle.restart();
    Ok(())
}
