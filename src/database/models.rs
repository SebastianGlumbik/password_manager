pub mod basic;
pub mod specific;
pub mod traits;

use crate::impl_id;
use crate::models::traits::{Id, Label, Position, Required};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub enum Content {
    Number(basic::Number),
    Text(basic::Text),
    Datetime(basic::Datetime),
    Password(specific::Password),
    Totp(specific::Totp),
    Url(specific::Url),
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
            Content::Totp(_) => "TOTP",
            Content::Url(_) => "URL",
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
            Content::Totp(totp) => totp.id(),
            Content::Url(url) => url.id(),
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
            Content::Totp(totp) => totp.set_id(id),
            Content::Url(url) => url.set_id(id),
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
            Content::Totp(totp) => totp.label(),
            Content::Url(url) => url.label(),
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
            Content::Totp(totp) => totp.set_label(label),
            Content::Url(url) => url.set_label(label),
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
            Content::Totp(totp) => totp.position(),
            Content::Url(url) => url.position(),
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
            Content::Totp(totp) => totp.set_position(position),
            Content::Url(url) => url.set_position(position),
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
            Content::Totp(totp) => totp.required(),
            Content::Url(url) => url.required(),
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
            Content::Totp(totp) => totp.set_required(required),
            Content::Url(url) => url.set_required(required),
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
}

impl Record {
    pub fn new(name: String, category: Category) -> Record {
        Record {
            id: 0,
            name,
            created: chrono::Local::now(),
            last_modified: chrono::Local::now(),
            category,
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
    pub fn set_name(&mut self, name: String) {
        self.name.zeroize();
        self.name = name;
    }
    pub fn set_created(&mut self, created: chrono::DateTime<chrono::Local>) {
        self.created = created;
    }
    pub fn set_last_modified(&mut self, last_modified: chrono::DateTime<chrono::Local>) {
        self.last_modified = last_modified;
    }
    pub fn set_category(&mut self, category: Category) {
        self.category.zeroize();
        self.category = category;
    }
}

impl_id!(for Record);

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
