mod convert;
pub mod model;

use super::*;
use crate::database::model::value::ToSecretString;
use model::*;
use rusqlite::{params, Connection, OptionalExtension, Result};
use secrecy::{ExposeSecret, SecretString};
use std::fs;
use std::ops::Not;
use std::path::PathBuf;
use std::sync::Mutex;

/// Name of the database file.
pub const DATABASE_FILE_NAME: &str = "database.password_manager";

/// Database for the application. It uses SQLite with SQLCipher.
pub struct Database {
    connection: Mutex<Connection>,
}

impl Database {
    /// Returns full path to the database file based on the app local data directory.
    /// Paths:
    /// - macOS: ~/Library/Application Support/\<APPLICATION\>/[`DATABASE_FILE_NAME`]
    /// - Linux:  ~/.local/share/\<APPLICATION\>/[`DATABASE_FILE_NAME`]
    pub fn path(app_handle: &AppHandle) -> Option<PathBuf> {
        app_handle
            .path_resolver()
            .app_local_data_dir()
            .map(|path_buf| path_buf.join(DATABASE_FILE_NAME))
    }

    /// Checks if the database file exists based on the app local data directory.
    pub fn exists(app_handle: &AppHandle) -> bool {
        Database::path(app_handle)
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    /// Opens database file. If the file does not exist, it will be created. Location of the file is based on the app local data directory.
    /// # Errors
    /// If database cannot be opened
    pub fn open(password: &str, app_handle: &AppHandle) -> Result<Database, &'static str> {
        if password.trim().is_empty() {
            return Err("Password can not be empty");
        }

        let path = Database::path(app_handle).ok_or("Failed to get database path")?;
        if path.exists().not() {
            fs::create_dir_all(path.parent().ok_or("Failed to get data directory path")?)
                .map_err(|_| "Failed to create data directory")?;
        }
        let path = path.to_str().ok_or("Path is not valid UTF-8")?;

        let Ok(connection) = Connection::open(path) else {
            return Err("Failed to open database");
        };

        let sql = SecretString::new(format!("PRAGMA key = '{password}';").into());
        connection
            .execute_batch(sql.expose_secret())
            .map_err(|_| "Failed to unlock database")?;

        connection
            .execute_batch("PRAGMA cache_size = 0;")
            .unwrap_or_default();

        connection
            .execute_batch("SELECT count(*) FROM sqlite_master;")
            .map_err(|_| "Invalid password")?;

        connection
            .execute_batch("PRAGMA cipher_memory_security = ON;")
            .map_err(|_| "Failed to enable memory security")?;

        connection
            .execute_batch("
                        create table if not exists Settings (
                            name text primary key,
                            value text not null
                        );
                        create table if not exists Record (
                            id_record integer primary key,
                            title text not null,
                            subtitle text not null,
                            created datetime not null,
                            last_modified datetime not null,
                            category text not null
                        );
                        create table if not exists Content (
                            id_content integer primary key,
                            id_record integer not null,
                            label text not null,
                            position integer not null,
                            required integer not null,
                            kind text not null,
                            value text not null,
                            foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                        );
                        create table if not exists DataBreachCache (
                            hash text primary key,
                            exposed integer not null,
                            checked datetime not null
                        );"
            ).map_err(|_| "Failed to create database")?;

        Ok(Database {
            connection: Mutex::new(connection),
        })
    }

    /// Changes the password for the database. It will re-encrypt the database with the new password.
    /// # Errors
    /// If the new password is empty or if the key cannot be changed.
    pub fn change_key(&self, new_password: &str) -> Result<(), &'static str> {
        if new_password.trim().is_empty() {
            return Err("Password can not be empty");
        }
        let sql = SecretString::new(format!("PRAGMA rekey = '{new_password}';").into());
        self.connection
            .lock()
            .map_err(|_| "Failed to access database lock")?
            .execute_batch(sql.expose_secret())
            .map_err(|_| "Failed to set a new key")
    }

    pub fn get_setting(&self, name: &str) -> Result<SecretValue, &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare("SELECT value FROM Settings WHERE name = ?1;")
            .map_err(|_| "Failed to prepare statement")?;
        stmt.query_row(params![name], |row| row.get(0))
            .map_err(|_| "Failed to get setting")
    }

    pub fn get_content(&self, id_content: u64) -> Result<Content, &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare("SELECT id_content, label, position, required, kind, value FROM Content WHERE id_content = ?1;")
            .map_err(|_| "Failed to prepare statement")?;
        stmt.query_row(params![id_content], convert::row_to_content)
            .map_err(|_| "Failed to get content")
    }

    pub fn get_all_records(&self) -> Result<Vec<Record>, &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare(
                "SELECT id_record, title, subtitle, created, last_modified, category FROM Record;",
            )
            .map_err(|_| "Failed to prepare statement")?;
        let result: Result<Vec<Record>> = stmt
            .query_map([], convert::row_to_record)
            .map_err(|_| "Failed to map records")?
            .collect();
        result.map_err(|_| "Failed to get records")
    }

    pub fn get_all_content_for_record(&self, id_record: u64) -> Result<Vec<Content>, &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare("SELECT id_content, label, position, required, kind, value FROM Content WHERE id_record = ?1;")
            .map_err(|_| "Failed to prepare statement")?;
        let result: Result<Vec<Content>> = stmt
            .query_map([id_record], convert::row_to_content)
            .map_err(|_| "Failed to map content")?
            .collect();
        result.map_err(|_| "Failed to get content")
    }

    pub fn get_all_passwords_for_record(
        &self,
        id_record: u64,
    ) -> Result<Vec<SecretValue>, &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare("SELECT value FROM Content WHERE id_record = ?1 AND kind = 'Password';")
            .map_err(|_| "Failed to prepare statement")?;
        let result: Result<Vec<SecretValue>> = stmt
            .query_map([id_record], |row| row.get(0))
            .map_err(|_| "Failed to map password")?
            .collect();
        result.map_err(|_| "Failed to get passwords")
    }

    /// Based on the hash, it returns the breach status from the cache.
    pub fn get_data_breach_status(&self, hash: &str) -> Result<Option<bool>, &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare("SELECT exposed FROM DataBreachCache WHERE hash = ?1;")
            .map_err(|_| "Failed to prepare statement")?;
        stmt.query_row(params![hash], |row| row.get(0))
            .optional()
            .map_err(|_| "Failed to get breach status")
    }

    pub fn save_setting(&self, name: &str, value: &str) -> Result<(), &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        connection
            .execute(
                "REPLACE INTO Settings (name, value) VALUES (?1, ?2);",
                params![name, value],
            )
            .map_err(|_| "Failed to save setting")?;
        Ok(())
    }

    /// Saves a record to the database. Based on the id, it will insert or update the record. If the record is new, it will get an id.
    pub fn save_record(&self, record: &mut Record) -> Result<(), &'static str> {
        record.set_last_modified(chrono::Local::now());
        let title = record.title();
        let subtitle = record.subtitle();
        let created = record.created();
        let last_modified = record.last_modified();
        let category = record.category().as_str();
        let id_record = record.id();

        let mut params =
            params![title, subtitle, created, last_modified, category, id_record].to_vec();
        let sql = if id_record == 0 {
            params.pop();
            "INSERT INTO Record (title, subtitle, created, last_modified, category) VALUES (?1, ?2, ?3, ?4, ?5);"
        } else {
            "UPDATE Record SET title = ?1, subtitle = ?2, created = ?3, last_modified = ?4, category = ?5 WHERE id_record = ?6;"
        };
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        connection
            .execute(sql, &*params)
            .map_err(|_| "Failed to save record")?;
        if id_record == 0 {
            record.set_id(connection.last_insert_rowid() as u64);
        }
        Ok(())
    }

    /// Saves content to the database. Based on the id, it will insert or update the content. If the content is new, it will get an id.
    pub fn save_content(&self, id_record: u64, content: &mut Content) -> Result<(), &'static str> {
        let label = content.label();
        let position = content.position();
        let required = content.required();
        let kind = content.kind();
        let secret_value = content.value().to_secret_string();
        let value = secret_value.expose_secret();
        let id_content = content.id();
        let mut params = params![label, position, required, kind, value].to_vec();
        let sql = if id_content == 0 {
            params.append(&mut params![id_record].to_vec());
            "INSERT INTO Content (label, position, required, kind, value, id_record) VALUES (?1, ?2, ?3, ?4, ?5, ?6);"
        } else {
            params.append(&mut params![id_content].to_vec());
            "UPDATE Content SET label = ?1, position = ?2, required = ?3, kind = ?4, value = ?5 WHERE id_content = ?6;"
        };
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        connection
            .execute(sql, &*params)
            .map_err(|_| "Failed to save content")?;
        if id_content == 0 {
            content.set_id(connection.last_insert_rowid() as u64);
        }
        Ok(())
    }

    /// To add password hash breach status to the cache.
    pub fn add_data_breach_cache(&self, hash: &str, exposed: bool) -> Result<(), &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        connection
            .execute("REPLACE INTO DataBreachCache (hash, exposed, checked) VALUES (?1, ?2, datetime('now'));", params![hash, exposed])
            .map_err(|_| "Failed to save content")?;
        Ok(())
    }

    pub fn delete_setting(&self, name: &str) -> Result<(), &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        connection
            .execute("DELETE FROM Settings WHERE name = ?1;", params![name])
            .map_err(|_| "Failed to delete setting")?;
        Ok(())
    }

    /// Deletes a record from the database. It will also delete all content for the record.
    pub fn delete_record(&self, record: Record) -> Result<(), &'static str> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let transaction = connection
            .transaction()
            .map_err(|_| "Failed to start transaction")?;
        transaction
            .execute(
                "DELETE FROM Content WHERE id_record = ?1;",
                params![record.id()],
            )
            .map_err(|_| "Failed to delete records content")?;
        transaction
            .execute(
                "DELETE FROM Record WHERE id_record = ?1;",
                params![record.id()],
            )
            .map_err(|_| "Failed to delete record")?;
        transaction
            .commit()
            .map_err(|_| "Failed to commit transaction")
    }

    pub fn delete_content(&self, content: Content) -> Result<(), &'static str> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let transaction = connection
            .transaction()
            .map_err(|_| "Failed to start transaction")?;
        transaction
            .execute(
                "DELETE FROM Content WHERE id_content = ?1;",
                params![content.id()],
            )
            .map_err(|_| "Failed to delete content")?;
        transaction
            .commit()
            .map_err(|_| "Failed to commit transaction")
    }

    /// Deletes all password hash breach status older than 24 hours.
    pub fn delete_data_breach_cache_older_24h(&self) -> Result<(), &'static str> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        connection
            .execute(
                "DELETE FROM DataBreachCache WHERE checked < datetime('now', '-1 day');",
                [],
            )
            .map_err(|_| "Failed to delete old breach status")?;
        Ok(())
    }
}
