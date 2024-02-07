use super::model::*;
use rusqlite::{Error, Result, Row};
use zeroize::Zeroize;

/// Helper function to convert a number from the database to a Number struct.
/// # Error
/// Returns an error if the value cannot be converted to a Number.
fn number_from_database(value: String) -> Result<Number, Error> {
    Number::new(value)
        .map_err(|e| Error::InvalidColumnType(4, e.to_string(), rusqlite::types::Type::Text))
}

/// Helper function to convert a date from the database to a Date struct.
/// # Error
/// Returns an error if the value cannot be converted to a Date.
fn date_from_database(value: String) -> Result<Date, Error> {
    Date::new(value)
        .map_err(|e| Error::InvalidColumnType(4, e.to_string(), rusqlite::types::Type::Text))
}

/// Helper function to convert a TOTP secret from the database to a TOTPSecret struct.
/// # Error
/// Returns an error if the value cannot be converted to a TOTPSecret.
fn totp_from_database(value: String) -> Result<TOTPSecret, Error> {
    TOTPSecret::new(value)
        .map_err(|e| Error::InvalidColumnType(4, e.to_string(), rusqlite::types::Type::Text))
}

/// Helper function to convert an url from the database to an Url struct.
/// # Error
/// Returns an error if the value cannot be converted to an Url.
fn url_from_database(value: String) -> Result<Url, Error> {
    Url::new(value)
        .map_err(|e| Error::InvalidColumnType(4, e.to_string(), rusqlite::types::Type::Text))
}

/// Helper function to convert an email from the database to an Email struct.
/// # Error
/// Returns an error if the value cannot be converted to an Email.
fn email_from_database(value: String) -> Result<Email, Error> {
    Email::new(value)
        .map_err(|e| Error::InvalidColumnType(4, e.to_string(), rusqlite::types::Type::Text))
}

/// Helper function to convert a phone number from the database to a PhoneNumber struct.
/// # Error
/// Returns an error if the value cannot be converted to a PhoneNumber.
fn phone_number_from_database(value: String) -> Result<PhoneNumber, Error> {
    PhoneNumber::new(value)
        .map_err(|e| Error::InvalidColumnType(4, e.to_string(), rusqlite::types::Type::Text))
}

/// Helper function to convert a bank card number from the database to a BankCardNumber struct.
/// # Error
/// Returns an error if the value cannot be converted to a BankCardNumber.
fn bank_card_number_from_database(value: String) -> Result<BankCardNumber, Error> {
    BankCardNumber::new(value)
        .map_err(|e| Error::InvalidColumnType(4, e.to_string(), rusqlite::types::Type::Text))
}

/// Helper function to convert a record from the database to a Record struct.
fn record_from_database(
    id: u64,
    title: String,
    subtitle: String,
    created: chrono::DateTime<chrono::Local>,
    last_modified: chrono::DateTime<chrono::Local>,
    category: Category,
) -> Record {
    let mut record = Record::new(title, subtitle, category);
    record.set_id(id);
    record.set_created(created);
    record.set_last_modified(last_modified);
    record
}

/// Helper function to convert a row from the database to a Record struct.
/// # Error
/// Returns an error if the row cannot be converted to a Record.
pub fn row_to_record(row: &Row) -> Result<Record> {
    Ok(record_from_database(
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        Category::from_string(row.get(5)?),
    ))
}

/// Helper function to convert a row from the database to a Content struct.
/// # Error
/// Returns an error if the row cannot be converted to a Content.
pub fn row_to_content(row: &Row) -> Result<Content> {
    let mut id: u64 = row.get(0)?;
    let mut label: String = row.get(1)?;
    let mut position: u32 = row.get(2)?;
    let mut required: bool = row.get(3)?;
    let mut kind: String = row.get(4)?;
    let mut value: String = row.get(5)?;

    let value = match kind.as_str() {
        "Number" => Value::Number(number_from_database(value)?),
        "Text" => Value::Text(Text::new(value)),
        "LongText" => Value::LongText(LongText::new(value)),
        "SensitiveText" => Value::SensitiveText(SensitiveText::new(value)),
        "Date" => Value::Date(date_from_database(value)?),
        "Password" => Value::Password(Password::new(value)),
        "TOTPSecret" => Value::TOTPSecret(totp_from_database(value)?),
        "Url" => Value::Url(url_from_database(value)?),
        "Email" => Value::Email(email_from_database(value)?),
        "PhoneNumber" => Value::PhoneNumber(phone_number_from_database(value)?),
        "BankCardNumber" => Value::BankCardNumber(bank_card_number_from_database(value)?),
        _ => {
            id.zeroize();
            label.zeroize();
            position.zeroize();
            required.zeroize();
            kind.zeroize();
            value.zeroize();
            return Err(Error::InvalidColumnType(
                4,
                "Unknown kind".to_string(),
                rusqlite::types::Type::Text,
            ));
        }
    };

    let mut content = Content::new(label, position, required, value);
    content.set_id(id);

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_from_database_invalid() {
        let result = number_from_database("invalid".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_number_from_database_valid() {
        let result = number_from_database("123".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), "123");
    }
    #[test]
    fn test_date_from_database_invalid() {
        let result = date_from_database("invalid".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_date_from_database_valid() {
        let result = date_from_database("2020-04-12".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value().to_string(), "2020-04-12");
    }
    #[test]
    fn test_totp_from_database_invalid() {
        let result = totp_from_database("invalid".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_totp_from_database_valid() {
        let result = totp_from_database("rfffmaz4jsjq3qurwhzna2wljastmywv".to_string());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().value(),
            "rfffmaz4jsjq3qurwhzna2wljastmywv".to_uppercase()
        );
    }
    #[test]
    fn test_url_from_database_invalid() {
        let result = url_from_database("invalid".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_url_from_database_valid() {
        let result = url_from_database("https://example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), "https://example.com");
    }
    #[test]
    fn test_email_from_database_invalid() {
        let result = email_from_database("invalid".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_email_from_database_valid() {
        let result = email_from_database("example@email.com".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), "example@email.com");
    }
    #[test]
    fn test_phone_number_from_database_invalid() {
        let result = phone_number_from_database("invalid".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_phone_number_from_database_valid() {
        let result = phone_number_from_database("+14152370800".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), "+14152370800");
    }
    #[test]
    fn test_bank_card_number_from_database_invalid() {
        let result = bank_card_number_from_database("invalid".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_bank_card_number_from_database_valid() {
        let result = bank_card_number_from_database("4702932172193242".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), "4702932172193242");
    }
    #[test]
    fn test_record_from_database() {
        let record = record_from_database(
            1,
            "Title".to_string(),
            "Subtitle".to_string(),
            chrono::Local::now(),
            chrono::Local::now(),
            Category::Login,
        );
        assert_eq!(record.id(), 1);
        assert_eq!(record.title(), "Title");
        assert_eq!(record.subtitle(), "Subtitle");
        assert_eq!(record.category(), &Category::Login);
    }
}
