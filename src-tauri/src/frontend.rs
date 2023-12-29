mod login;
mod register;

use super::*;
pub use login::*;
pub use register::*;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

const DATABASE_FILE_NAME: &str = "database.db";

/// Database connection for the frontend.
pub struct DatabaseConnection {
    database: Mutex<Option<Database>>,
}

impl DatabaseConnection {
    pub fn default() -> Self {
        Self {
            database: Default::default(),
        }
    }
}

/// Looks for a database file in the app local data directory and returns its path if it exists.
/// For macOS, the data directory is ~/Library/Application Support/\<APPLICATION\>
#[tauri::command]
pub fn database_exists(app_handle: tauri::AppHandle) -> Option<PathBuf> {
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

/// Opens connection to the database. If the database does not exist, it will be created.
pub fn connect_database(
    password: &str,
    connection: State<DatabaseConnection>,
    app_handle: tauri::AppHandle,
) -> Result<(), &'static str> {
    let path = match database_exists(app_handle.clone()) {
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
            *database = Some(Database::open(&path, password)?);
            Ok(())
        }
        Err(_) => Err("Failed to access database lock"),
    }
}
