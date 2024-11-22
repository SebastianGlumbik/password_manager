pub mod value;

use super::*;
use rusqlite::types::FromSql;
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
use value::*;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Secret string with Serialize and Sqlite support
#[derive(Debug, Clone, Deserialize)]
pub struct SecretValue(SecretString);

impl SecretValue {
    pub fn new(secret: SecretString) -> SecretValue {
        SecretValue(secret)
    }

    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl FromStr for SecretValue {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SecretValue(SecretString::from(s)))
    }
}

impl Serialize for SecretValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.expose_secret())
    }
}

impl FromSql for SecretValue {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        Ok(SecretValue(SecretString::from(value.as_str()?)))
    }
}

/// Record category
#[derive(Debug, PartialEq, Clone, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub enum Category {
    Login,
    #[serde(rename(serialize = "Bank Card"))]
    #[serde(alias = "Bank Card")]
    BankCard,
    Note,
    #[serde(other)]
    Other,
}

impl Category {
    /// Converts a string to a category
    pub fn from_string(category: String) -> Category {
        let category = SecretString::new(category.into());
        match category.expose_secret() {
            "Login" => Category::Login,
            "BankCard" => Category::BankCard,
            "Note" => Category::Note,
            _ => Category::Other,
        }
    }
    /// Converts a category to a string
    pub fn as_str(&self) -> &str {
        match self {
            Category::Login => "Login",
            Category::BankCard => "BankCard",
            Category::Note => "Note",
            Category::Other => "Other",
        }
    }
}

/// Represents a record in the database
#[derive(Debug, PartialEq, Clone, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Record {
    #[serde(default)]
    id: u64,
    title: String,
    subtitle: String,
    category: Category,
    #[zeroize(skip)]
    #[serde(default = "chrono::Local::now")]
    created: chrono::DateTime<chrono::Local>,
    #[zeroize(skip)]
    #[serde(default = "chrono::Local::now")]
    last_modified: chrono::DateTime<chrono::Local>,
}

impl Record {
    /// Creates a new record
    /// - id is set to 0 to indicate that it is not in the database
    /// - created and last_modified are set to the current time
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
    pub fn set_created(&mut self, created: chrono::DateTime<chrono::Local>) {
        self.created = created;
    }
    pub fn set_last_modified(&mut self, last_modified: chrono::DateTime<chrono::Local>) {
        self.last_modified = last_modified;
    }
}

/// Represents value of a content
#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Value {
    Number(Number),
    Text(Text),
    LongText(LongText),
    SensitiveText(SensitiveText),
    Date(Date),
    Password(Password),
    TOTPSecret(TOTPSecret),
    Url(Url),
    Email(Email),
    PhoneNumber(PhoneNumber),
    BankCardNumber(BankCardNumber),
}

impl ToSecretString for Value {
    fn to_secret_string(&self) -> SecretString {
        match &self {
            Value::Number(number) => number.to_secret_string(),
            Value::Text(text) => text.to_secret_string(),
            Value::LongText(long_text) => long_text.to_secret_string(),
            Value::SensitiveText(sensitive_text) => sensitive_text.to_secret_string(),
            Value::Date(date) => date.to_secret_string(),
            Value::Password(password) => password.to_secret_string(),
            Value::TOTPSecret(totp_secret) => totp_secret.to_secret_string(),
            Value::Url(url) => url.to_secret_string(),
            Value::Email(email) => email.to_secret_string(),
            Value::PhoneNumber(phone_number) => phone_number.to_secret_string(),
            Value::BankCardNumber(bank_card_number) => bank_card_number.to_secret_string(),
        }
    }
}

/// Represents a content in the database
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
    /// Creates a new content
    /// - id is set to 0 to indicate that it is not in the database
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
            Value::LongText(_) => "LongText",
            Value::SensitiveText(_) => "SensitiveText",
            Value::Date(_) => "Date",
            Value::Password(_) => "Password",
            Value::TOTPSecret(_) => "TOTPSecret",
            Value::Url(_) => "Url",
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_category_from_string() {
        assert_eq!(Category::from_string("Login".to_string()), Category::Login);
        assert_eq!(
            Category::from_string("BankCard".to_string()),
            Category::BankCard
        );
        assert_eq!(Category::from_string("Note".to_string()), Category::Note);
        assert_eq!(Category::from_string("Other".to_string()), Category::Other);
        assert_eq!(
            Category::from_string("Unknown".to_string()),
            Category::Other
        );
    }
    #[test]
    fn test_category_as_str() {
        assert_eq!(Category::Login.as_str(), "Login");
        assert_eq!(Category::BankCard.as_str(), "BankCard");
        assert_eq!(Category::Note.as_str(), "Note");
        assert_eq!(Category::Other.as_str(), "Other");
    }
    #[test]
    fn test_category_serialize() {
        assert_eq!(
            serde_json::to_string(&Category::Login).unwrap(),
            "\"Login\""
        );
        assert_eq!(
            serde_json::to_string(&Category::BankCard).unwrap(),
            "\"Bank Card\""
        );
        assert_eq!(serde_json::to_string(&Category::Note).unwrap(), "\"Note\"");
        assert_eq!(
            serde_json::to_string(&Category::Other).unwrap(),
            "\"Other\""
        );
    }
    #[test]
    fn test_category_deserialize() {
        assert_eq!(
            serde_json::from_str::<Category>("\"Login\"").unwrap(),
            Category::Login
        );
        assert_eq!(
            serde_json::from_str::<Category>("\"BankCard\"").unwrap(),
            Category::BankCard
        );
        assert_eq!(
            serde_json::from_str::<Category>("\"Bank Card\"").unwrap(),
            Category::BankCard
        );
        assert_eq!(
            serde_json::from_str::<Category>("\"Note\"").unwrap(),
            Category::Note
        );
        assert_eq!(
            serde_json::from_str::<Category>("\"Unknown\"").unwrap(),
            Category::Other
        );
    }
    #[test]
    fn test_new_record() {
        let mut record = Record::new("Title".to_string(), "Subtitle".to_string(), Category::Login);
        assert_eq!(record.id(), 0);
        assert_eq!(record.title(), "Title");
        assert_eq!(record.subtitle(), "Subtitle");
        assert_eq!(record.category(), &Category::Login);
        record.set_id(1);
        let now = chrono::Local::now();
        record.set_created(now);
        record.set_last_modified(now);
        assert_eq!(record.id(), 1);
        assert_eq!(record.created(), now);
        assert_eq!(record.last_modified(), now);
    }
    #[test]
    fn test_record_serialize() {
        let record = Record::new("Title".to_string(), "Subtitle".to_string(), Category::Login);
        let created = serde_json::to_string(&record.created()).unwrap();
        let last_modified = serde_json::to_string(&record.last_modified()).unwrap();
        assert_eq!(
            serde_json::to_string(&record).unwrap(),
            format!("{{\"id\":0,\"title\":\"Title\",\"subtitle\":\"Subtitle\",\"category\":\"Login\",\"created\":{},\"last_modified\":{}}}",created,last_modified)
        );
    }
    #[test]
    fn test_record_deserialize() {
        let now = chrono::Local::now();
        let record = serde_json::from_str::<Record>(
            "{\"title\":\"Title\",\"subtitle\":\"Subtitle\",\"category\":\"Login\"}",
        );
        assert!(record.is_ok());
        let record = record.unwrap();
        assert_eq!(record.id(), 0);
        assert_eq!(record.title(), "Title");
        assert_eq!(record.subtitle(), "Subtitle");
        assert_eq!(record.category(), &Category::Login);
        assert!(record.created() >= now);
        assert!(record.last_modified() >= now);
    }
    #[test]
    fn test_new_content() {
        let mut content = Content::new(
            "Label".to_string(),
            1,
            true,
            Value::Text(Text::new("Text".to_string())),
        );
        assert_eq!(content.id(), 0);
        assert_eq!(content.label(), "Label");
        assert_eq!(content.position(), 1);
        assert!(content.required());
        assert_eq!(content.value(), &Value::Text(Text::new("Text".to_string())));
        content.set_id(1);
        assert_eq!(content.id(), 1);
    }
    #[test]
    fn test_content_serialize() {
        let content = Content::new(
            "Label".to_string(),
            1,
            true,
            Value::Text(Text::new("Text".to_string())),
        );
        assert_eq!(
            serde_json::to_string(&content).unwrap(),
            "{\"id\":0,\"label\":\"Label\",\"position\":1,\"required\":true,\"kind\":\"Text\",\"value\":\"Text\"}"
        );
    }
    #[test]
    fn test_content_deserialize() {
        let content = serde_json::from_str::<Content>(
            "{\"label\":\"Label\",\"position\":1,\"required\":true,\"kind\":\"Text\",\"value\":\"Text\"}",
        );
        assert!(content.is_ok());
        let content = content.unwrap();
        assert_eq!(content.id(), 0);
        assert_eq!(content.label(), "Label");
        assert_eq!(content.position(), 1);
        assert!(content.required());
        assert_eq!(content.value(), &Value::Text(Text::new("Text".to_string())));
    }
}
