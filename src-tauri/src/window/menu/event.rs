use super::*;
use crate::database::model::*;
use crate::database::DATABASE_FILE_NAME;
use std::fs;
use tauri::{MenuEvent, Window};

/// Handles all menu events.
pub fn menu_event(event: MenuEvent, app_handle: AppHandle, window: Window) {
    match event.menu_item_id() {
        "Start Over" => start_over(app_handle, window),
        "Choose database" => choose_database(app_handle, window),
        "Settings" => window.emit("settings", ()).unwrap_or_default(),
        "New Login" => window
            .emit_all(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::Login),
            )
            .unwrap_or_default(),
        "New Bank Card" => window
            .emit(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::BankCard),
            )
            .unwrap_or_default(),
        "New Note" => window
            .emit(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::Note),
            )
            .unwrap_or_default(),
        "New Other" => window
            .emit(
                "new_record",
                Record::new("".to_string(), "".to_string(), Category::Other),
            )
            .unwrap_or_default(),
        "Export Database" => export_database(app_handle, window),
        _ => tauri::api::dialog::message(
            Some(&window),
            "Error",
            format!("Unknown menu item: {}", event.menu_item_id()),
        ),
    }
}

/// Deletes the database file and restarts the application. Has dialogs.
pub fn start_over(app_handle: AppHandle, window: Window) {
    tauri::async_runtime::spawn_blocking(move || {
        if let Some(path_buf) = Database::path(&app_handle) {
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
            tauri::api::dialog::message(Some(&window), "Error", "Failed to get database path");
        }
    });
}

/// Sets the database file and restarts the application. Has dialogs.
pub fn choose_database(app_handle: AppHandle, window: Window) {
    tauri::async_runtime::spawn_blocking(move || {
        if let Some(old_database) = Database::path(&app_handle) {
            if !old_database.exists() || tauri::api::dialog::blocking::ask(
                Some(&window),
                "Set database",
                "Database already exists. Are you sure you want to continue? This action will permanently delete all passwords in the current database.",
            ) {
                if let Some(new_database) = tauri::api::dialog::blocking::FileDialogBuilder::new()
                    .set_parent(&window)
                    .set_title("Set database").add_filter("Password Manager", &["password_manager"])
                    .pick_file() {
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
    });
}

/// Exports the database file. Has dialog.
pub fn export_database(app_handle: AppHandle, window: Window) {
    tauri::async_runtime::spawn_blocking(move || {
        if let Some(source) = Database::path(&app_handle) {
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
    });
}
