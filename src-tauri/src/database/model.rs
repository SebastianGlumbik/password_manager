pub mod value;

use serde::{Deserialize, Serialize};
pub use value::*;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Record category
/// ## serde
/// Serialize and Deserialize as string
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
    pub fn as_str(&self) -> &str {
        match self {
            Category::Login => "Login",
            Category::BankCard => "BankCard",
            Category::Note => "Note",
            Category::Custom(name) => name,
        }
    }
}

impl Serialize for Category {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'a> Deserialize<'a> for Category {
    fn deserialize<D: serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let category = String::deserialize(deserializer)?;
        Ok(Category::from_string(category))
    }
}

/// Represents a record in the database
/// ## serde
/// If id, created and last_modified are not set, they will be set to their default values. Default value for last_modified is current time.
#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Record {
    #[serde(default)]
    id: u64,
    title: String,
    subtitle: String,
    category: Category,
    #[zeroize(skip)]
    #[serde(default)]
    created: chrono::DateTime<chrono::Local>,
    #[zeroize(skip)]
    #[serde(default = "chrono::Local::now")]
    last_modified: chrono::DateTime<chrono::Local>,
}

impl Record {
    pub fn new(title: String, subtitle: String, category: Category) -> Record {
        Record {
            id: 0,
            title,
            subtitle,
            category,
            created: chrono::Local::now(),
            last_modified: chrono::Local::now(),
        }
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn subtitle(&self) -> &str {
        &self.subtitle
    }
    pub fn category(&self) -> &Category {
        &self.category
    }
    pub fn created(&self) -> chrono::DateTime<chrono::Local> {
        self.created
    }
    pub fn last_modified(&self) -> chrono::DateTime<chrono::Local> {
        self.last_modified
    }
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }
    pub fn set_title(&mut self, title: String) {
        self.title.zeroize();
        self.title = title;
    }
    pub fn set_subtitle(&mut self, subtitle: String) {
        self.subtitle.zeroize();
        self.subtitle = subtitle;
    }
    pub fn set_category(&mut self, category: Category) {
        self.category = category;
    }
    pub fn set_created(&mut self, created: chrono::DateTime<chrono::Local>) {
        self.created = created;
    }
    pub fn set_last_modified(&mut self, last_modified: chrono::DateTime<chrono::Local>) {
        self.last_modified = last_modified;
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Value {
    Number(Number),
    Text(Text),
    SensitiveText(SensitiveText),
    Datetime(Datetime),
    Password(Password),
    TOTPSecret(TOTPSecret),
    Url(Url),
    Email(Email),
    PhoneNumber(PhoneNumber),
    BankCardNumber(BankCardNumber),
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Content {
    #[serde(default)]
    id: u64,
    label: String,
    position: u32,
    required: bool,
    #[serde(flatten)]
    value: Value,
}

impl Content {
    pub fn new(label: String, position: u32, required: bool, value: Value) -> Content {
        Content {
            id: 0,
            label,
            position,
            required,
            value,
        }
    }
    pub fn kind(&self) -> &str {
        match self.value {
            Value::Number(_) => "Number",
            Value::Text(_) => "Text",
            Value::SensitiveText(_) => "SensitiveText",
            Value::Datetime(_) => "Datetime",
            Value::Password(_) => "Password",
            Value::TOTPSecret(_) => "TOTPSecret",
            Value::Url(_) => "URL",
            Value::Email(_) => "Email",
            Value::PhoneNumber(_) => "PhoneNumber",
            Value::BankCardNumber(_) => "BankCardNumber",
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn position(&self) -> u32 {
        self.position
    }
    pub fn required(&self) -> bool {
        self.required
    }
    pub fn value(&self) -> &Value {
        &self.value
    }
    pub fn set_id(&mut self, id: u64) {
        self.id.zeroize();
        self.id = id;
    }
    pub fn set_label(&mut self, label: String) {
        self.label.zeroize();
        self.label = label;
    }
    pub fn set_position(&mut self, position: u32) {
        self.position.zeroize();
        self.position = position;
    }
    pub fn set_required(&mut self, required: bool) {
        self.required.zeroize();
        self.required = required;
    }
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
