mod command;
mod event;
mod menu;
mod window;

pub use command::*;
pub use event::*;
pub use menu::*;
use std::collections::HashMap;
pub use window::*;

use super::*;
use secrecy::{ExposeSecret, SecretString};
use std::fs;
use std::ops::Not;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, State, Window};
use totp_rs::{Rfc6238, TOTP};

/// Name of the database file.
pub const DATABASE_FILE_NAME: &str = "database.password_manager";

/// Database connection for tauri state.
#[derive(Default)]
pub struct DatabaseConnection {
    database_mutex: Mutex<Option<Database>>,
}

impl DatabaseConnection {
    /// Opens connection to the database. If the database does not exist, it will be created.
    /// # Errors
    /// Returns an error if the database cannot be opened.
    pub fn connect(
        &self,
        password: SecretString,
        app_handle: AppHandle,
    ) -> Result<(), &'static str> {
        let path = Self::database_path(app_handle.clone()).ok_or("Failed to get database path")?;

        if path.exists().not() {
            fs::create_dir_all(path.parent().ok_or("Failed to get data directory path")?)
                .map_err(|_| "Failed to create data directory")?;
        }

        let path = path.to_str().ok_or("Path is not valid UTF-8")?;

        match self.database_mutex.lock() {
            Ok(mut guard) => {
                *guard = Some(Database::open(path, password.expose_secret())?);
                Ok(())
            }
            Err(_) => Err("Failed to access database lock"),
        }
    }

    /// Closes connection to the database.
    /// # Errors
    /// Returns an error if the database cannot be closed.
    pub fn disconnect(&self) -> Result<(), &'static str> {
        match self.database_mutex.lock() {
            Ok(mut guard) => {
                *guard = None;
                Ok(())
            }
            Err(_) => Err("Failed to access database lock"),
        }
    }

    /// Checks if the database is connected. Returns false if the database is not connected or if the database mutex is poisoned.
    pub fn is_connected(&self) -> bool {
        self.database_mutex
            .lock()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
    }

    /// Returns full path to the database file based on the app local data directory.
    /// Paths:
    /// - macOS: ~/Library/Application Support/\<APPLICATION\>/[`DATABASE_FILE_NAME`]
    /// - Linux:  ~/.local/share/\<APPLICATION\>/[`DATABASE_FILE_NAME`]
    pub fn database_path(app_handle: AppHandle) -> Option<PathBuf> {
        app_handle
            .path_resolver()
            .app_local_data_dir()
            .map(|path_buf| path_buf.join(DATABASE_FILE_NAME))
    }

    /// Checks if the database file exists.
    pub fn database_exists(app_handle: AppHandle) -> bool {
        if let Some(path) = Self::database_path(app_handle) {
            return path.exists();
        }

        false
    }
}

#[derive(Default)]
pub struct TOTPManager {
    totp_map: Mutex<HashMap<u64, TOTP>>,
}

impl TOTPManager {
    pub fn add_url(&self, id: u64, url: SecretString) -> Result<(), &'static str> {
        let Ok(totp) = TOTP::from_url(url.expose_secret()) else {
            return Err("Invalid OTP Auth URL");
        };

        match self.totp_map.lock() {
            Ok(mut guard) => {
                guard.insert(id, totp);
                Ok(())
            }
            Err(_) => Err("Failed to access manager lock"),
        }
    }

    pub fn add_secret(&self, id: u64, secret: String) -> Result<(), &'static str> {
        let Ok(secret) = totp_rs::Secret::Encoded(secret).to_bytes() else {
            return Err("Invalid OTP Secret");
        };
        let Ok(rfc6238) = Rfc6238::with_defaults(secret) else {
            return Err("Invalid OTP Secret");
        };
        let Ok(totp) = TOTP::from_rfc6238(rfc6238) else {
            return Err("Invalid OTP Secret");
        };

        match self.totp_map.lock() {
            Ok(mut guard) => {
                guard.insert(id, totp);
                Ok(())
            }
            Err(_) => Err("Failed to access manager lock"),
        }
    }

    pub fn get_code(&self, id: &u64) -> Option<(String, u64)> {
        if let Ok(mut guard) = self.totp_map.lock() {
            if let Some(totp) = guard.get_mut(id) {
                let current = totp.generate_current().ok()?;
                let ttl = totp.ttl().ok()?;
                return Some((current, ttl));
            }
        }

        None
    }

    pub fn remove(&self, id: &u64) {
        self.totp_map.lock().ok().map(|mut guard| {
            guard.remove(id);
        });
    }
}
