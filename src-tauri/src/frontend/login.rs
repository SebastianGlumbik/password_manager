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

    connect_database(password, connection, app_handle.clone())?;

    window.close().map_err(|_| "Failed to close window")?;

    // TODO change also in register.rs maybe separate function for building window
    WindowBuilder::new(
        &app_handle,
        "main",
        tauri::WindowUrl::App("/src/main.html".into()),
    )
    .resizable(true)
    .title("Password Manager")
    .min_inner_size(640f64, 480f64)
    .inner_size(800f64, 600f64)
    .build()
    .map_err(|_| "Failed to open new window")?;

    Ok(())
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
