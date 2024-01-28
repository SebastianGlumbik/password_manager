use crate::database::model::*;
use chrono::ParseError;
use rusqlite::{Error, Result, Row};
use zeroize::Zeroize;

fn number_from_database(mut value: String) -> Result<Number, Error> {
    let new_value = value.parse::<i64>();
    value.zeroize();
    match new_value {
        Ok(value) => Ok(Number::new(value)),
        Err(e) => Err(Error::InvalidColumnType(
            4,
            e.to_string(),
            rusqlite::types::Type::Text,
        )),
    }
}
fn datetime_from_database(mut value: String) -> Result<Datetime, Error> {
    let new_value: Result<chrono::DateTime<chrono::Local>, ParseError> =
        std::str::FromStr::from_str(value.as_str());
    value.zeroize();
    match new_value {
        Ok(value) => Ok(Datetime::new(value)),
        Err(e) => Err(Error::InvalidColumnType(
            4,
            e.to_string(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn totp_from_database(value: String) -> Result<TOTPSecret, Error> {
    match TOTPSecret::new(value) {
        Ok(totp) => Ok(totp),
        Err(e) => Err(Error::InvalidColumnType(
            4,
            e.to_string(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn url_from_database(value: String) -> Result<Url, Error> {
    match Url::new(value) {
        Ok(url) => Ok(url),
        Err(e) => Err(Error::InvalidColumnType(
            4,
            e.to_string(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn email_from_database(value: String) -> Result<Email, Error> {
    match Email::new(value) {
        Ok(email) => Ok(email),
        Err(e) => Err(Error::InvalidColumnType(
            4,
            e.to_string(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn phone_number_from_database(value: String) -> Result<PhoneNumber, Error> {
    match PhoneNumber::new(value) {
        Ok(phone_number) => Ok(phone_number),
        Err(e) => Err(Error::InvalidColumnType(
            4,
            e.to_string(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn bank_card_number_from_database(value: String) -> Result<BankCardNumber, Error> {
    match BankCardNumber::new(value) {
        Ok(bank_card_number) => Ok(bank_card_number),
        Err(e) => Err(Error::InvalidColumnType(
            4,
            e.to_string(),
            rusqlite::types::Type::Text,
        )),
    }
}

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

pub fn row_to_content(row: &Row) -> Result<Content> {
    let id: u64 = row.get(0)?;
    let label: String = row.get(1)?;
    let position: u32 = row.get(2)?;
    let required: bool = row.get(3)?;
    let kind: String = row.get(4)?;
    let value: String = row.get(5)?;

    let value = match kind.as_str() {
        "Number" => Value::Number(number_from_database(value)?),
        "Text" => Value::Text(Text::new(value)),
        "SensitiveText" => Value::SensitiveText(SensitiveText::new(value)),
        "Datetime" => Value::Datetime(datetime_from_database(value)?),
        "Password" => Value::Password(Password::new(value)),
        "TOTPSecret" => Value::TOTPSecret(totp_from_database(value)?),
        "URL" => Value::Url(url_from_database(value)?),
        "Email" => Value::Email(email_from_database(value)?),
        "PhoneNumber" => Value::PhoneNumber(phone_number_from_database(value)?),
        "BankCardNumber" => Value::BankCardNumber(bank_card_number_from_database(value)?),
        e => {
            return Err(Error::InvalidColumnType(
                4,
                e.to_string(),
                rusqlite::types::Type::Text,
            ))
        }
    };

    let mut content = Content::new(label, position, required, value);
    content.set_id(id);

    Ok(content)
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
