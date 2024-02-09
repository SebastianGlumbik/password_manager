use super::*;
use crate::database::model::{Category, Record};
use std::fmt::format;
use tauri::MenuEvent;

/// Shows a critical error message and restarts the application.
///
/// **NOTE:** Has blocking dialog [`tauri::api::dialog::blocking`].
pub fn critical_error(message: &str, app_handle: AppHandle, window: Window) {
    tauri::api::dialog::blocking::message(
        Some(&window),
        "Critical Error",
        format!("{}\nApplication will now restart", message),
    );
    app_handle.restart();
}

/// Handles all menu events.
pub fn menu_event(event: MenuEvent, app_handle: AppHandle, window: Window) {
    let thread_builder = std::thread::Builder::new();
    let window_clone = window.clone();
    let thread = thread_builder.spawn(move || match event.menu_item_id() {
        "Start Over" => block_on(start_over(app_handle, window_clone)),
        "Choose database" => block_on(choose_database(app_handle, window_clone)),
        "Settings" => window_clone.emit("settings", ()).unwrap_or_default(),
        "New Login" => window_clone
            .emit_all(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::Login),
            )
            .unwrap_or_default(),
        "New Bank Card" => window_clone
            .emit(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::BankCard),
            )
            .unwrap_or_default(),
        "New Note" => window_clone
            .emit(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::Note),
            )
            .unwrap_or_default(),
        "New Other" => window_clone
            .emit(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::Other),
            )
            .unwrap_or_default(),
        "Export Database" => block_on(export_database(app_handle, window_clone)),
        _ => tauri::api::dialog::message(
            Some(&window_clone),
            "Error",
            format!("Unknown menu item: {}", event.menu_item_id()),
        ),
    });

    if thread.is_err() {
        tauri::api::dialog::message(Some(&window), "Error", "Unexpected error");
    }
}

/// Deletes the database file and restarts the application. Has blocking dialogs, so it cannot be run on the main thread.
/// # Dialogs
/// - Has a confirmation dialog before deleting the database file
/// - Has a message dialog if the database file could not be deleted
pub async fn start_over(app_handle: AppHandle, window: Window) {
    if let Some(path_buf) = database_path(app_handle.clone()) {
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
    if let Some(old_database) = database_path(app_handle.clone()) {
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

/// Exports the database file. Has blocking dialogs, so it cannot be run on the main thread.
pub async fn export_database(app_handle: AppHandle, window: Window) {
    if let Some(source) = database_path(app_handle.clone()) {
        if let Some(destination) = tauri::api::dialog::blocking::FileDialogBuilder::new()
            .set_parent(&window)
            .set_title("Export database")
            .set_file_name(DATABASE_FILE_NAME)
            .save_file()
        {
            if let Err(error) = fs::copy(source, destination) {
                tauri::api::dialog::blocking::message(
                    Some(&window),
                    "Error",
                    format!("Failed to copy database file: {}", error),
                );
            }
        }
    }
}
