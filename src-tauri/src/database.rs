mod convert;
pub mod model;
use model::*;
use rusqlite::{params, Connection, Result};
use secrecy::{ExposeSecret, SecretString};
use std::sync::Mutex;

pub struct Database {
    connection: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &str, password: &str) -> Result<Database, &'static str> {
        if password.trim().is_empty() {
            return Err("Password can not be empty");
        }
        let Ok(connection) = Connection::open(path) else {
            return Err("Failed to open database");
        };

        let sql = SecretString::new(format!("PRAGMA key = '{password}';"));
        connection
            .execute_batch(sql.expose_secret())
            .map_err(|_| "Failed to unlock database")?;

        let sql = SecretString::new("SELECT count(*) FROM sqlite_master;".to_string());
        connection
            .execute_batch(sql.expose_secret())
            .map_err(|_| "Invalid password")?;

        let sql = SecretString::new("PRAGMA cipher_memory_security = ON;".to_string());
        connection
            .execute_batch(sql.expose_secret())
            .map_err(|_| "Failed to enable memory security")?;

        let sql = SecretString::new(
                        "create table if not exists Record (
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
                        );".to_string());
        connection
            .execute_batch(sql.expose_secret())
            .map_err(|_| "Failed to create database")?;

        Ok(Database {
            connection: Mutex::new(connection),
        })
    }

    pub fn change_key(&self, new_password: &str) -> Result<(), &'static str> {
        if new_password.trim().is_empty() {
            return Err("Password can not be empty");
        }
        let sql = SecretString::new(format!("PRAGMA rekey = '{new_password}';"));
        self.connection
            .lock()
            .map_err(|_| "Failed to access database lock")?
            .execute_batch(sql.expose_secret())
            .map_err(|_| "Failed to set a new key")
    }

    pub fn get_record(&self, id_record: u64) -> Result<Record, &'static str> {
        let sql = SecretString::new(
            "SELECT id_record, title, subtitle, created, last_modified, category FROM Record WHERE id_record = ?1;".to_string());
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare(sql.expose_secret())
            .map_err(|_| "Failed to prepare statement")?;
        stmt.query_row(params![id_record], convert::row_to_record)
            .map_err(|_| "Failed to get record")
    }

    pub fn get_content(&self, id_content: u64) -> Result<Content, &'static str> {
        let sql = SecretString::new(
            "SELECT id_content, label, position, required, kind, value FROM Content WHERE id_content = ?1;".to_string());
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare(sql.expose_secret())
            .map_err(|_| "Failed to prepare statement")?;
        stmt.query_row(params![id_content], convert::row_to_content)
            .map_err(|_| "Failed to get content")
    }
    pub fn get_all_records(&self) -> Result<Vec<Record>, &'static str> {
        let sql = SecretString::new(
            "SELECT id_record, title, subtitle, created, last_modified, category FROM Record;"
                .to_string(),
        );
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare(sql.expose_secret())
            .map_err(|_| "Failed to prepare statement")?;
        let result: Result<Vec<Record>> = stmt
            .query_map([], convert::row_to_record)
            .map_err(|_| "Failed to map records")?
            .collect();
        result.map_err(|_| "Failed to get records")
    }

    pub fn get_all_content_for_record(&self, id_record: u64) -> Result<Vec<Content>, &'static str> {
        let sql = SecretString::new("SELECT id_content, label, position, required, kind, value FROM Content WHERE id_record = ?1;".to_string());
        let connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let mut stmt = connection
            .prepare(sql.expose_secret())
            .map_err(|_| "Failed to prepare statement")?;
        let result: Result<Vec<Content>> = stmt
            .query_map([id_record], convert::row_to_content)
            .map_err(|_| "Failed to map content")?
            .collect();
        result.map_err(|_| "Failed to get content")
    }
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
    pub fn delete_record(&self, record: Record) -> Result<(), &'static str> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| "Failed to access database lock")?;
        let transaction = connection
            .transaction()
            .map_err(|_| "Failed to start transaction")?;
        let sql = SecretString::new("DELETE FROM Content WHERE id_record = ?1;".to_string());
        transaction
            .execute(sql.expose_secret(), params![record.id()])
            .map_err(|_| "Failed to delete records content")?;
        let sql = SecretString::new("DELETE FROM Record WHERE id_record = ?1;".to_string());
        transaction
            .execute(sql.expose_secret(), params![record.id()])
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
        let sql = SecretString::new("DELETE FROM Content WHERE id_content = ?1;".to_string());
        transaction
            .execute(sql.expose_secret(), params![content.id()])
            .map_err(|_| "Failed to delete content")?;
        transaction
            .commit()
            .map_err(|_| "Failed to commit transaction")
    }
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
