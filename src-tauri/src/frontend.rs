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
use crate::database::Database;
use secrecy::{ExposeSecret, SecretString};
use std::fs;
use std::ops::Not;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, State, Window};
use totp_rs::{Rfc6238, TOTP};

/// Name of the database file.
pub const DATABASE_FILE_NAME: &str = "database.password_manager";

/// Database connection for tauri state. Used for accessing the database.
#[derive(Default)]
pub struct DatabaseConnection {
    database: Mutex<Option<Database>>,
}

impl DatabaseConnection {
    /// Opens connection to the database. If the database does not exist, it will be created.
    /// # Error
    /// Returns an error if the database cannot be opened.
    pub fn connect(
        &self,
        password: SecretString,
        app_handle: AppHandle,
    ) -> Result<(), &'static str> {
        let path = Self::database_path(app_handle).ok_or("Failed to get database path")?;

        if path.exists().not() {
            fs::create_dir_all(path.parent().ok_or("Failed to get data directory path")?)
                .map_err(|_| "Failed to create data directory")?;
        }

        let path = path.to_str().ok_or("Path is not valid UTF-8")?;

        let mut guard = self
            .database
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        *guard = Some(Database::open(path, password.expose_secret())?);
        Ok(())
    }

    /// Closes connection to the database.
    /// # Error
    /// Returns an error if the database cannot be closed.
    pub fn disconnect(&self) -> Result<(), &'static str> {
        let mut guard = self
            .database
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        *guard = None;
        Ok(())
    }

    /// Checks if the database is connected. Returns false if the database is not connected or if the database mutex is poisoned.
    pub fn is_connected(&self) -> bool {
        self.database
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

/// TOTP manager for tauri state. Used for managing TOTP secrets and generating codes.
#[derive(Default)]
pub struct TOTPManager {
    hash_map: Mutex<HashMap<u64, TOTP>>,
}

impl TOTPManager {
    /// Adds a new TOTP secret to the manager.
    /// # Error
    /// Returns an error if the secret is invalid or if the manager mutex is poisoned.
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

        let mut guard = self
            .hash_map
            .lock()
            .map_err(|_| "Failed to access manager lock")?;
        guard.insert(id, totp);
        Ok(())
    }

    /// Generates a TOTP code for the given secret.
    /// # Return
    /// Returns the current TOTP code and the time to live in seconds or None if the secret does not exist or if the manager mutex is poisoned.
    pub fn get_code(&self, id: &u64) -> Option<(String, u64)> {
        let mut guard = self.hash_map.lock().ok()?;
        let totp = guard.get_mut(id)?;
        let current = totp.generate_current().ok()?;
        let ttl = totp.ttl().ok()?;
        Some((current, ttl))
    }

    /// Removes a TOTP secret from the manager.
    pub fn remove(&self, id: &u64) {
        self.hash_map.lock().ok().map(|mut guard| {
            guard.remove(id);
        });
    }
}
