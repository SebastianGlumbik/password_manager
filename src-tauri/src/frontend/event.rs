use super::*;
use tauri::MenuEvent;

/// Handles all menu events.
pub fn menu_event(event: MenuEvent, app_handle: AppHandle, window: Window) {
    let thread_builder = std::thread::Builder::new();
    let window_clone = window.clone();
    let thread = thread_builder.spawn(move || match event.menu_item_id() {
        "Start Over" => block_on(start_over(app_handle, window_clone)),
        "Choose database" => block_on(choose_database(app_handle, window_clone)),
        "Log out" => block_on(logout(app_handle.clone().state(), app_handle, window_clone))
            .unwrap_or_default(),
        "New Login" => window_clone
            .emit_all(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::Login),
            )
            .unwrap(),
        "New Bank Card" => window_clone
            .emit(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::BankCard),
            )
            .unwrap(),
        "New Note" => window_clone
            .emit(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::Note),
            )
            .unwrap(),
        "New Other" => window_clone
            .emit(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::Other),
            )
            .unwrap(),
        _ => println!("Unknown menu item: {}", event.menu_item_id()),
    });

    if thread.is_err() {
        tauri::api::dialog::blocking::message(Some(&window), "Error", "Failed to create thread");
    }
}

/// Deletes the database file and restarts the application. Has blocking dialogs, so it cannot be run on the main thread.
/// # Dialogs
/// - Has a confirmation dialog before deleting the database file
/// - Has a message dialog if the database file could not be deleted
pub async fn start_over(app_handle: AppHandle, window: Window) {
    if let Some(path_buf) = DatabaseConnection::database_path(app_handle.clone()) {
        if tauri::api::dialog::blocking::ask(
            Some(&window),
            "Starting over",
            "Are you sure you want to continue? This action will permanently delete all passwords.",
        ) {
            if let Err(error) = fs::remove_file(path_buf) {
                tauri::api::dialog::blocking::message(
                    Some(&window),
                    "Error",
                    format!("Failed to delete database file: {}", error),
                );
            }

            app_handle.restart()
        }
    } else {
        tauri::api::dialog::blocking::message(
            Some(&window),
            "Error",
            "Failed to get database path",
        );
    }
}

/// Sets the database file and restarts the application. Has blocking dialogs, so it cannot be run on the main thread.
/// # Dialogs
/// - Has a file dialog to select the database file
/// - If the database file already exists, has a confirmation dialog before overwriting the database file
/// - Has a message dialog if the database file could not be copied
pub async fn choose_database(app_handle: AppHandle, window: Window) {
    if let Some(old_database) = DatabaseConnection::database_path(app_handle.clone()) {
        if !old_database.exists() || tauri::api::dialog::blocking::ask(
            Some(&window),
            "Set database",
            "Database already exists. Are you sure you want to continue? This action will permanently delete all passwords in the current database.",
        ) {
            if let Some(new_database) = tauri::api::dialog::blocking::FileDialogBuilder::new()
                .set_parent(&window)
                .set_title("Set database").add_filter("Password Manager", &["password_manager"])
                .pick_file(){
                if let Err(error) = fs::copy(new_database, old_database) {
                    tauri::api::dialog::blocking::message(
                        Some(&window),
                        "Error",
                        format!("Failed to copy database file: {}", error),
                    );
                }
                app_handle.restart();
            }
        }
    }
}

/// Disconnects from the database, creates login window and closes main window.
pub async fn logout<'a>(
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    connection.disconnect()?;

    #[cfg(target_os = "macos")]
    app_handle
        .save_window_state(StateFlags::all())
        .unwrap_or_default();

    initialize_window(connection, app_handle)
        .await
        .map_err(|_| "Failed to create login window")?;

    window.close().map_err(|_| "Failed to close window")?;

    Ok(())
}
