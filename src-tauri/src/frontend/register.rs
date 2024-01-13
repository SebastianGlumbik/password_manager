use super::*;

/// Register function for the frontend.
/// - Compares the passwords
/// - Checks if the database exists
/// - Creates the database
#[tauri::command]
pub async fn register<'a, 'b, 'c>(
    password: &'a str,
    confirm_password: &'b str,
    connection: State<'c, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    if password != confirm_password {
        return Err("Passwords do not match.");
    }

    if database_exists(app_handle.clone()).is_some() {
        tauri::api::dialog::blocking::message(
            Some(&window),
            "Error",
            "Database already exists, application will now restart",
        );
        app_handle.restart();
    }

    connect_database(password, connection, app_handle.clone())?;

    create_main_window(app_handle, window)?;

    Ok(())
}
