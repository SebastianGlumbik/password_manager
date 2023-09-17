pub mod basic;
pub mod specific;
pub mod traits;

use crate::impl_id;
use crate::models::traits::{Id, Label, Position, Required};
use std::cmp::Ordering;
use std::ops::Not;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub enum Content {
    Number(basic::Number),
    Text(basic::Text),
    Datetime(basic::Datetime),
    Password(specific::Password),
    TOTP(specific::TOTP),
    URL(specific::URL),
    Email(specific::Email),
    PhoneNumber(specific::PhoneNumber),
    BankCardNumber(specific::BankCardNumber),
}

impl Content {
    pub fn type_(&self) -> &str {
        match self {
            Content::Number(_) => "Number",
            Content::Text(text) => match text.text_type() {
                basic::TextType::Normal => "NormalText",
                basic::TextType::Long => "LongText",
                basic::TextType::Sensitive => "SensitiveText",
            },
            Content::Datetime(_) => "Datetime",
            Content::Password(_) => "Password",
            Content::TOTP(_) => "TOTP",
            Content::URL(_) => "URL",
            Content::Email(_) => "Email",
            Content::PhoneNumber(_) => "PhoneNumber",
            Content::BankCardNumber(_) => "BankCardNumber",
        }
    }
}

impl Id for Content {
    fn id(&self) -> u64 {
        match self {
            Content::Number(number) => number.id(),
            Content::Text(text) => text.id(),
            Content::Datetime(datetime) => datetime.id(),
            Content::Password(password) => password.id(),
            Content::TOTP(totp) => totp.id(),
            Content::URL(url) => url.id(),
            Content::Email(email) => email.id(),
            Content::PhoneNumber(phone_number) => phone_number.id(),
            Content::BankCardNumber(bank_card_number) => bank_card_number.id(),
        }
    }

    fn set_id(&mut self, id: u64) {
        match self {
            Content::Number(number) => number.set_id(id),
            Content::Text(text) => text.set_id(id),
            Content::Datetime(datetime) => datetime.set_id(id),
            Content::Password(password) => password.set_id(id),
            Content::TOTP(totp) => totp.set_id(id),
            Content::URL(url) => url.set_id(id),
            Content::Email(email) => email.set_id(id),
            Content::PhoneNumber(phone_number) => phone_number.set_id(id),
            Content::BankCardNumber(bank_card_number) => bank_card_number.set_id(id),
        }
    }
}

impl Label for Content {
    fn label(&self) -> &str {
        match self {
            Content::Number(number) => number.label(),
            Content::Text(text) => text.label(),
            Content::Datetime(datetime) => datetime.label(),
            Content::Password(password) => password.label(),
            Content::TOTP(totp) => totp.label(),
            Content::URL(url) => url.label(),
            Content::Email(email) => email.label(),
            Content::PhoneNumber(phone_number) => phone_number.label(),
            Content::BankCardNumber(bank_card_number) => bank_card_number.label(),
        }
    }

    fn set_label(&mut self, label: String) {
        match self {
            Content::Number(number) => number.set_label(label),
            Content::Text(text) => text.set_label(label),
            Content::Datetime(datetime) => datetime.set_label(label),
            Content::Password(password) => password.set_label(label),
            Content::TOTP(totp) => totp.set_label(label),
            Content::URL(url) => url.set_label(label),
            Content::Email(email) => email.set_label(label),
            Content::PhoneNumber(phone_number) => phone_number.set_label(label),
            Content::BankCardNumber(bank_card_number) => bank_card_number.set_label(label),
        }
    }
}

impl Position for Content {
    fn position(&self) -> u32 {
        match self {
            Content::Number(number) => number.position(),
            Content::Text(text) => text.position(),
            Content::Datetime(datetime) => datetime.position(),
            Content::Password(password) => password.position(),
            Content::TOTP(totp) => totp.position(),
            Content::URL(url) => url.position(),
            Content::Email(email) => email.position(),
            Content::PhoneNumber(phone_number) => phone_number.position(),
            Content::BankCardNumber(bank_card_number) => bank_card_number.position(),
        }
    }

    fn set_position(&mut self, position: u32) {
        match self {
            Content::Number(number) => number.set_position(position),
            Content::Text(text) => text.set_position(position),
            Content::Datetime(datetime) => datetime.set_position(position),
            Content::Password(content) => content.set_position(position),
            Content::TOTP(totp) => totp.set_position(position),
            Content::URL(url) => url.set_position(position),
            Content::Email(email) => email.set_position(position),
            Content::PhoneNumber(phone_number) => phone_number.set_position(position),
            Content::BankCardNumber(bank_card_number) => bank_card_number.set_position(position),
        }
    }
}

impl Required for Content {
    fn required(&self) -> bool {
        match self {
            Content::Number(number) => number.required(),
            Content::Text(text) => text.required(),
            Content::Datetime(datetime) => datetime.required(),
            Content::Password(content) => content.required(),
            Content::TOTP(totp) => totp.required(),
            Content::URL(url) => url.required(),
            Content::Email(email) => email.required(),
            Content::PhoneNumber(phone_number) => phone_number.required(),
            Content::BankCardNumber(bank_card_number) => bank_card_number.required(),
        }
    }

    fn set_required(&mut self, required: bool) {
        match self {
            Content::Number(number) => number.set_required(required),
            Content::Text(text) => text.set_required(required),
            Content::Datetime(datetime) => datetime.set_required(required),
            Content::Password(password) => password.set_required(required),
            Content::TOTP(totp) => totp.set_required(required),
            Content::URL(url) => url.set_required(required),
            Content::Email(email) => email.set_required(required),
            Content::PhoneNumber(phone_number) => phone_number.set_required(required),
            Content::BankCardNumber(bank_card_number) => bank_card_number.set_required(required),
        }
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub enum Category {
    Login,
    BankCard,
    Note,
    Custom(String),
}

impl Category {
    pub fn from_string(category: String) -> Category {
        match category.as_str() {
            "Login" => Category::Login,
            "BankCard" => Category::BankCard,
            "Note" => Category::Note,
            _ => Category::Custom(category),
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Category::Login => "Login",
            Category::BankCard => "BankCard",
            Category::Note => "Note",
            Category::Custom(name) => name,
        }
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct Record {
    id: u64,
    name: String,
    #[zeroize(skip)]
    created: chrono::DateTime<chrono::Local>,
    #[zeroize(skip)]
    last_modified: chrono::DateTime<chrono::Local>,
    category: Category,
    content: Vec<Content>,
    deleted: Vec<u64>,
}

impl Record {
    pub fn new(name: String, category: Category) -> Record {
        Record::from_database(
            0,
            name,
            chrono::Local::now(),
            chrono::Local::now(),
            category,
        )
    }
    pub fn from_database(
        id: u64,
        name: String,
        created: chrono::DateTime<chrono::Local>,
        last_modified: chrono::DateTime<chrono::Local>,
        category: Category,
    ) -> Record {
        Record {
            id,
            name,
            created,
            last_modified,
            category,
            content: Vec::new(),
            deleted: Vec::new(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn created(&self) -> chrono::DateTime<chrono::Local> {
        self.created
    }
    pub fn last_modified(&self) -> chrono::DateTime<chrono::Local> {
        self.last_modified
    }
    pub fn category(&self) -> &Category {
        &self.category
    }
    pub fn content(&self) -> &Vec<Content> {
        &self.content
    }

    pub fn deleted(&self) -> &Vec<u64> {
        &self.deleted
    }
    pub fn set_name(&mut self, name: String) {
        self.name.zeroize();
        self.name = name;
    }
    pub fn set_last_modified(&mut self, last_modified: chrono::DateTime<chrono::Local>) {
        self.last_modified = last_modified;
    }
    pub fn add_content(&mut self, mut content: Content) {
        match content.position() {
            0 => {
                content.set_position((self.content.len() as u32) + 1);
                self.content.push(content);
            }
            _ => {
                let id = content.id();
                self.content.push(content);
                self.content
                    .sort_by(|a, b| match a.position().cmp(&b.position()) {
                        Ordering::Less => Ordering::Less,
                        Ordering::Equal => {
                            if b.id() == id {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            }
                        }
                        Ordering::Greater => Ordering::Greater,
                    });
                self.content
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, content)| content.set_position((i + 1) as u32));
            }
        }
    }
    pub fn update_content(&mut self, position: u32) -> Option<&mut Content> {
        self.content.get_mut((position - 1) as usize)
    }

    pub fn delete_content(&mut self, position: u32) -> bool {
        if let Some(content) = self.content.get((position - 1) as usize) {
            if content.required().not() {
                self.deleted.push(content.id());
                self.content.remove((position - 1) as usize);
                return true;
            }
        }

        false
    }
    pub fn clear_content(&mut self) {
        self.content.clear();
    }
    pub fn content_count(&self) -> usize {
        self.content.len()
    }
}

impl_id!(for Record);

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
