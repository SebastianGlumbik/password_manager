pub mod database;
pub mod utils;

pub use crate::database::*;
use crate::models::Record;

//TODO Comments for whole project

//TODO csv export/import, cloud backup
pub struct PasswordManager {
    database: Database,
    records: Vec<Record>,
}

impl PasswordManager {
    pub fn new(password: &str) -> Result<PasswordManager, &'static str> {
        let database = Database::open(password)?;
        let records = Vec::new();
        Ok(PasswordManager { database, records })
    }
    pub fn change_key(&mut self, new_password: &str) -> Result<(), &'static str> {
        self.database.change_key(new_password)
    }
    pub fn load_from_drive(&mut self) -> Result<(), &'static str> {
        let Ok(records) = self.database.get_all_records() else {
            return Err("Failed to load records from database");
        };
        self.records = records;
        Ok(())
    }
    pub fn save_to_drive(&mut self) -> Result<(), &'static str> {
        for record in self.records.iter_mut() {
            if self.database.save_record(record).is_err() {
                return Err("Failed to save records to drive");
            }
        }
        Ok(())
    }

    pub fn load_content(&mut self, index: usize) -> Result<(), &'static str> {
        if let Some(record) = self.records.get_mut(index) {
            if self.database.load_record_content(record).is_err() {
                return Err("Failed to load contents from database");
            };
        }
        Ok(())
    }
    pub fn records(&self) -> &Vec<Record> {
        &self.records
    }
    pub fn add_record(&mut self, record: Record) {
        self.records.push(record);
    }
    pub fn update_record(&mut self, index: usize) -> Option<&mut Record> {
        self.records.get_mut(index)
    }
    pub fn delete_record(&mut self, index: usize) -> bool {
        self.database
            .delete_record(&self.records.remove(index))
            .is_ok()
    }
    pub fn clear_records(&mut self) {
        self.records.clear();
    }
    pub fn records_count(&self) -> usize {
        self.records.len()
    }
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
