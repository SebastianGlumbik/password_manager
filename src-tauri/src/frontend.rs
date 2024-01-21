mod login;
mod main;
mod register;

use super::*;
pub use login::*;
pub use main::*;
pub use register::*;
use secrecy::{ExposeSecret, SecretString};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
#[cfg(target_os = "linux")]
use tauri::Submenu;
use tauri::{AppHandle, CustomMenuItem, Menu, MenuEntry, MenuItem, State, Window, WindowBuilder};

/// Name of the database file.
const DATABASE_FILE_NAME: &str = "database.db";

/// Database connection for tauri state.
#[derive(Default)]
pub struct DatabaseConnection {
    database: Mutex<Option<Database>>,
}

/// Looks for a database file in the app local data directory and returns its path if it exists.
/// Path for data directory:
/// - macOS: ~/Library/Application Support/\<APPLICATION\>
/// - Linux:  ~/.local/share/\<APPLICATION\>
#[tauri::command]
pub async fn database_exists(app_handle: AppHandle) -> Option<PathBuf> {
    if let Some(path) = app_handle
        .path_resolver()
        .app_local_data_dir()
        .map(|path_buf| path_buf.join(DATABASE_FILE_NAME))
    {
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Window types that can be created.
#[derive(Clone, serde::Serialize)]
pub enum WindowType {
    Login,
    Register,
    Main,
}

/// Creates specific window based on the database state and returns the window type.
/// # Errors
/// Returns an error if the window cannot be created.
#[tauri::command]
pub async fn initialize_window<'a>(
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
) -> tauri::Result<WindowType> {
    if let Ok(database) = connection.database.lock() {
        if database.is_some() {
            create_main_window(app_handle)?;
            return Ok(WindowType::Main);
        }
    }

    match database_exists(app_handle.clone()).await {
        Some(_) => {
            create_login_window(app_handle)?;
            Ok(WindowType::Login)
        }
        None => {
            create_register_window(app_handle)?;
            Ok(WindowType::Register)
        }
    }
}

/// Opens connection to the database. If the database does not exist, it will be created.
/// # Errors
/// Returns an error if the database cannot be opened.
pub async fn connect_database<'a, 'b>(
    password: SecretString,
    connection: State<'b, DatabaseConnection>,
    app_handle: AppHandle,
) -> Result<(), &'static str> {
    let path = match database_exists(app_handle.clone()).await {
        Some(path_buf) => path_buf
            .to_str()
            .ok_or("Path is not valid UTF-8")?
            .to_string(),
        None => {
            let path_buf = app_handle
                .path_resolver()
                .app_local_data_dir()
                .ok_or("Failed to get data directory path")?;
            fs::create_dir_all(&path_buf).map_err(|_| "Failed to create data directory")?;
            path_buf
                .join(DATABASE_FILE_NAME)
                .to_str()
                .ok_or("Path is not valid UTF-8")?
                .to_string()
        }
    };

    match connection.database.lock() {
        Ok(mut database) => {
            *database = Some(Database::open(&path, password.expose_secret())?);
            Ok(())
        }
        Err(_) => Err("Failed to access database lock"),
    }
}

/// Deletes the database file and restarts the application.
/// # Dialogs
/// - Has a confirmation dialog before deleting the database file
/// - Has a message dialog if the database file could not be deleted
pub async fn start_over(app_handle: AppHandle, window: Window) {
    if let Some(path_buf) = database_exists(app_handle.clone()).await {
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
