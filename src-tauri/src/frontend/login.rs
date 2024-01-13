use super::*;

/// Login function for the frontend.
/// - Checks if the database exist
/// - Opens the database
#[tauri::command]
pub async fn login<'a, 'b>(
    password: &'a str,
    connection: State<'b, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    if database_exists(app_handle.clone()).is_none() {
        tauri::api::dialog::blocking::message(
            Some(&window),
            "Error",
            "Database does not exist, application will now restart",
        );
        app_handle.restart();
    }

    connect_database(password, connection, app_handle.clone())?;

    create_main_window(app_handle, window)?;

    Ok(())
}

/// Deletes the database file and restarts the application.
/// - Has a confirmation dialog before deleting the database file
/// - Has a message dialog if the database file could not be deleted
pub fn start_over(app_handle: AppHandle, window: Window) {
    if let Some(path_buf) = database_exists(app_handle.clone()) {
        let window_close = window.clone();
        tauri::api::dialog::ask(
            Some(&window),
            "Starting over",
            "Are you sure you want to continue? This action will permanently delete all passwords.",
            move |answer| {
                if answer {
                    if let Err(error) = fs::remove_file(path_buf) {
                        tauri::api::dialog::message(
                            Some(&window_close),
                            "Error",
                            format!("Failed to delete database file: {}", error),
                        );
                    } else {
                        app_handle.restart();
                    }
                }
            },
        );
    } else {
        app_handle.restart();
    }
}
