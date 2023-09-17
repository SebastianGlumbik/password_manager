pub mod models;

use crate::models::traits::{Id, Label, Position, Required, ToSecretString};
use crate::models::*;
use rusqlite::{params, Connection, Result, Row};
use secrecy::ExposeSecret;
use std::error::Error;
use zeroize::Zeroize;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open(password: &str) -> Result<Database, &'static str> {
        if password.trim().is_empty() {
            return Err("Password cannot be empty");
        }
        let Ok(connection) = Connection::open("database.db") else {
            return Err("Failed to open database");
        };
        let mut sql = format!("PRAGMA key = '{password}';");
        if connection.execute_batch(&sql).is_err() {
            sql.zeroize();
            return Err("Failed to set a key");
        } else {
            sql.zeroize();
        }
        if connection
            .execute_batch("SELECT count(*) FROM sqlite_master;")
            .is_err()
        {
            return Err("Invalid password");
        }
        let sql = "create table if not exists Record (
                            id_record integer primary key,
                            name text not null,
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
                            type text not null,
                            value text not null,
                            foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                        );";
        if connection.execute_batch(sql).is_err() {
            return Err("Failed to create database");
        }
        Ok(Database { connection })
    }
    pub fn change_key(&mut self, new_password: &str) -> Result<(), &'static str> {
        let mut sql = format!("PRAGMA rekey = '{new_password}';");
        if self.connection.execute_batch(&sql).is_err() {
            sql.zeroize();
            Err("Failed to set a key")
        } else {
            sql.zeroize();
            Ok(())
        }
    }

    fn row_to_record(&self, row: &Row) -> Result<Record> {
        Ok(Record::from_database(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            Category::from_string(row.get(4)?),
        ))
    }

    fn row_to_content(&self, row: &Row) -> Result<Content, Box<dyn Error>> {
        let id: u64 = row.get(0)?;
        let label: String = row.get(1)?;
        let position: u32 = row.get(2)?;
        let required: bool = row.get(3)?;
        let type_: String = row.get(4)?;
        let value: String = row.get(5)?;
        Ok(match type_.as_str() {
            "Number" => Content::Number(basic::Number::from_database(
                id, label, position, required, value,
            )?),
            "NormalText" => Content::Text(basic::Text::from_database(
                id,
                label,
                position,
                required,
                value,
                basic::TextType::Normal,
            )),
            "LongText" => Content::Text(basic::Text::from_database(
                id,
                label,
                position,
                required,
                value,
                basic::TextType::Long,
            )),
            "SensitiveText" => Content::Text(basic::Text::from_database(
                id,
                label,
                position,
                required,
                value,
                basic::TextType::Sensitive,
            )),
            "Datetime" => Content::Datetime(basic::Datetime::from_database(
                id, label, position, required, value,
            )?),
            "Password" => Content::Password(specific::Password::from_database(
                id, label, position, required, value,
            )),
            "TOTP" => Content::TOTP(specific::TOTP::from_database(
                id, label, position, required, value,
            )?),
            "URL" => Content::URL(specific::URL::from_database(
                id, label, position, required, value,
            )?),
            "Email" => Content::Email(specific::Email::from_database(
                id, label, position, required, value,
            )?),
            "PhoneNumber" => Content::PhoneNumber(specific::PhoneNumber::from_database(
                id, label, position, required, value,
            )?),
            "BankCardNumber" => Content::BankCardNumber(specific::BankCardNumber::from_database(
                id, label, position, required, value,
            )?),
            _ => return Err("Invalid type".into()),
        })
    }
    pub fn get_record(&self, id: u32) -> Result<Record> {
        let sql =
            "SELECT id_record, name, created, last_modified, category FROM Record WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        stmt.query_row(params![id], |row| self.row_to_record(row))
    }
    pub fn get_all_records(&self) -> Result<Vec<Record>> {
        let sql = "SELECT id_record, name, created, last_modified, category FROM Record;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([], |row| self.row_to_record(row))?;
        Ok(items_iter.map(|item| item.unwrap()).collect())
    }

    pub fn load_record_content(&self, record: &mut Record) -> Result<(), Box<dyn Error>> {
        let sql = "SELECT id_content, label, position, required, type, value FROM Content WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([record.id()], |row| Ok(self.row_to_content(row)))?;
        items_iter.for_each(|item| {
            if let Ok(item) = item.unwrap() {
                record.add_content(item);
            }
        });
        Ok(())
    }
    pub fn save_record(&mut self, record: &mut Record) -> Result<()> {
        let transaction = self.connection.transaction()?;
        let name = record.name();
        let created = record.created();
        let last_modified = chrono::Local::now();
        let category = record.category().name();
        let id_record = record.id();

        let mut params = params![name, created, last_modified, category, id_record].to_vec();
        let sql = if id_record == 0 {
            params.pop();
            "INSERT INTO Record (name, created, last_modified, category) VALUES (?1, ?2, ?3, ?4);"
        } else {
            "UPDATE Record SET name = ?1, created = ?2, last_modified = ?3, category = ?4 WHERE id_record = ?5;"
        };
        transaction.execute(sql, &*params)?;
        let id_record = if id_record == 0 {
            record.set_id(transaction.last_insert_rowid() as u64);
            record.id()
        } else {
            record.id()
        };

        let mut inserted_ids_position: Vec<(u64, u32)> = Vec::new();

        for content in record.content() {
            let label = content.label();
            let position = content.position();
            let required = content.required();
            let type_ = content.type_();
            let secret_value = match content {
                Content::Number(number) => number.to_secret_string(),
                Content::Text(text) => text.to_secret_string(),
                Content::Datetime(datetime) => datetime.to_secret_string(),
                Content::Password(password) => password.to_secret_string(),
                Content::TOTP(totp) => totp.to_secret_string(),
                Content::URL(url) => url.to_secret_string(),
                Content::Email(email) => email.to_secret_string(),
                Content::PhoneNumber(phone_number) => phone_number.to_secret_string(),
                Content::BankCardNumber(bank_card_number) => bank_card_number.to_secret_string(),
            };
            let value = secret_value.expose_secret();
            let id_content = content.id();
            let mut params = params![label, position, required, type_, value].to_vec();
            let sql = if id_content == 0 {
                params.append(&mut params![id_record].to_vec());
                "INSERT INTO Content (label, position, required, type, value, id_record) VALUES (?1, ?2, ?3, ?4, ?5, ?6);"
            } else {
                params.append(&mut params![id_content].to_vec());
                "UPDATE Content SET label = ?1, position = ?2, required = ?3, type = ?4, value = ?5 WHERE id_content = ?6;"
            };
            transaction.execute(sql, &*params)?;
            if id_content == 0 {
                inserted_ids_position
                    .push((transaction.last_insert_rowid() as u64, content.position()));
            }
        }
        // Update ids for inserted contents
        for (id_content, position) in inserted_ids_position {
            if let Some(content) = record.update_content(position) {
                content.set_id(id_content)
            }
        }
        // Delete deleted from database
        for id_content in record.deleted() {
            let sql = "DELETE FROM Content WHERE id_content = ?1;";
            transaction.execute(sql, params![id_content])?;
        }

        transaction.commit()?;
        Ok(())
    }
    pub fn delete_record(&mut self, record: &Record) -> Result<()> {
        let transaction = self.connection.transaction()?;
        let sql = "DELETE FROM Content WHERE id_record = ?1;";
        transaction.execute(sql, params![record.id()])?;
        let sql = "DELETE FROM Record WHERE id_record = ?1;";
        transaction.execute(sql, params![record.id()])?;
        transaction.commit()?;
        Ok(())
    }
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
