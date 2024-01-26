pub mod model;
mod utils;

use crate::database::model::value::ToSecretString;
use model::*;
use rusqlite::{params, Connection, Result};
use secrecy::{ExposeSecret, SecretString};

pub struct Database {
    connection: Connection,
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

        Ok(Database { connection })
    }

    pub fn change_key(&mut self, new_password: &str) -> Result<(), &'static str> {
        let sql = SecretString::new(format!("PRAGMA rekey = '{new_password}';"));
        self.connection
            .execute_batch(sql.expose_secret())
            .map_err(|_| "Failed to set a new key")
    }

    pub fn get_record(&self, id_record: u64) -> Result<Record> {
        let sql = SecretString::new(
            "SELECT id_record, title, subtitle, created, last_modified, category FROM Record WHERE id_record = ?1;".to_string());
        let mut stmt = self.connection.prepare(sql.expose_secret())?;
        stmt.query_row(params![id_record], |row| utils::row_to_record(row))
    }

    pub fn get_content(&self, id_content: u64) -> Result<Content> {
        let sql = SecretString::new(
            "SELECT id_content, label, position, required, kind, value FROM Content WHERE id_content = ?1;".to_string());
        let mut stmt = self.connection.prepare(sql.expose_secret())?;
        stmt.query_row(params![id_content], |row| utils::row_to_content(row))
    }
    pub fn get_all_records(&self) -> Result<Vec<Record>> {
        let sql = SecretString::new(
            "SELECT id_record, title, subtitle, created, last_modified, category FROM Record;"
                .to_string(),
        );
        let mut stmt = self.connection.prepare(sql.expose_secret())?;
        let items_iter = stmt.query_map([], |row| utils::row_to_record(row))?;
        items_iter.collect()
    }

    pub fn get_all_content_for_record(&self, id_record: u64) -> Result<Vec<Content>> {
        let sql = SecretString::new("SELECT id_content, label, position, required, kind, value FROM Content WHERE id_record = ?1;".to_string());
        let mut stmt = self.connection.prepare(sql.expose_secret())?;
        let items_iter = stmt.query_map([id_record], |row| utils::row_to_content(row))?;
        items_iter.collect()
    }
    pub fn save_record(&self, record: &mut Record) -> Result<()> {
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
        self.connection.execute(sql, &*params)?;
        if id_record == 0 {
            record.set_id(self.connection.last_insert_rowid() as u64);
        }
        Ok(())
    }
    pub fn save_content(&self, id_record: u64, content: &mut Content) -> Result<()> {
        let label = content.label();
        let position = content.position();
        let required = content.required();
        let kind = content.kind();
        let secret_value = match &content.value() {
            Value::Number(number) => number.to_secret_string(),
            Value::Text(text) => text.to_secret_string(),
            Value::SensitiveText(sensitive_text) => sensitive_text.to_secret_string(),
            Value::Datetime(datetime) => datetime.to_secret_string(),
            Value::Password(password) => password.to_secret_string(),
            Value::Totp(totp) => totp.to_secret_string(),
            Value::Url(url) => url.to_secret_string(),
            Value::Email(email) => email.to_secret_string(),
            Value::PhoneNumber(phone_number) => phone_number.to_secret_string(),
            Value::BankCardNumber(bank_card_number) => bank_card_number.to_secret_string(),
        };
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
        self.connection.execute(sql, &*params)?;
        if id_content == 0 {
            content.set_id(self.connection.last_insert_rowid() as u64);
        }
        Ok(())
    }
    pub fn delete_record(&mut self, record: Record) -> Result<()> {
        let transaction = self.connection.transaction()?;
        let sql = SecretString::new("DELETE FROM Content WHERE id_record = ?1;".to_string());
        transaction.execute(sql.expose_secret(), params![record.id()])?;
        let sql = SecretString::new("DELETE FROM Record WHERE id_record = ?1;".to_string());
        transaction.execute(sql.expose_secret(), params![record.id()])?;
        transaction.commit()
    }

    pub fn delete_content(&mut self, content: Content) -> Result<()> {
        let transaction = self.connection.transaction()?;
        let sql = SecretString::new("DELETE FROM Content WHERE id_content = ?1;".to_string());
        transaction.execute(sql.expose_secret(), params![content.id()])?;
        transaction.commit()
    }
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
