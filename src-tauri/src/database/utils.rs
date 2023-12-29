use crate::database::models::traits::{Id, Position};
use crate::database::models::*;
use chrono::ParseError;
use rusqlite::{Error, Result, Row};
use zeroize::Zeroize;

fn number_from_database(
    mut id: u64,
    mut label: String,
    mut position: u32,
    mut required: bool,
    mut value: String,
) -> Result<basic::Number, Error> {
    let new_value = value.parse::<i128>();
    value.zeroize();
    match new_value {
        Ok(value) => {
            let mut number = basic::Number::new(label, required, value);
            number.set_id(id);
            number.set_position(position);
            Ok(number)
        }
        Err(e) => {
            id.zeroize();
            label.zeroize();
            position.zeroize();
            required.zeroize();
            Err(Error::InvalidColumnType(
                4,
                e.to_string(),
                rusqlite::types::Type::Text,
            ))
        }
    }
}

fn text_from_database(
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
    text_type: basic::TextType,
) -> basic::Text {
    let mut text = basic::Text::new(label, required, value, text_type);
    text.set_id(id);
    text.set_position(position);
    text
}

fn datetime_from_database(
    mut id: u64,
    mut label: String,
    mut position: u32,
    mut required: bool,
    mut value: String,
) -> Result<basic::Datetime, Error> {
    let new_value: Result<chrono::DateTime<chrono::Local>, ParseError> =
        std::str::FromStr::from_str(value.as_str());
    value.zeroize();
    match new_value {
        Ok(value) => {
            let mut datetime = basic::Datetime::new(label, required, value);
            datetime.set_id(id);
            datetime.set_position(position);
            Ok(datetime)
        }
        Err(e) => {
            id.zeroize();
            label.zeroize();
            position.zeroize();
            required.zeroize();
            Err(Error::InvalidColumnType(
                4,
                e.to_string(),
                rusqlite::types::Type::Text,
            ))
        }
    }
}

fn totp_from_database(
    mut id: u64,
    label: String,
    mut position: u32,
    required: bool,
    url: String,
) -> Result<specific::Totp, Error> {
    match specific::Totp::from_url(label, required, url) {
        Ok(mut totp) => {
            totp.set_id(id);
            totp.set_position(position);
            Ok(totp)
        }
        Err(e) => {
            id.zeroize();
            position.zeroize();
            Err(Error::InvalidColumnType(
                4,
                e.to_string(),
                rusqlite::types::Type::Text,
            ))
        }
    }
}

fn url_from_database(
    mut id: u64,
    label: String,
    mut position: u32,
    required: bool,
    value: String,
) -> Result<specific::Url, Error> {
    match specific::Url::new(label, required, value) {
        Ok(mut url) => {
            url.set_id(id);
            url.set_position(position);
            Ok(url)
        }
        Err(e) => {
            id.zeroize();
            position.zeroize();
            Err(Error::InvalidColumnType(
                4,
                e.to_string(),
                rusqlite::types::Type::Text,
            ))
        }
    }
}

fn password_from_database(
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
) -> specific::Password {
    let mut password = specific::Password::new(label, required, value);
    password.set_id(id);
    password.set_position(position);
    password
}

fn email_from_database(
    mut id: u64,
    label: String,
    mut position: u32,
    required: bool,
    value: String,
) -> Result<specific::Email, Error> {
    match specific::Email::new(label, required, value) {
        Ok(mut email) => {
            email.set_id(id);
            email.set_position(position);
            Ok(email)
        }
        Err(e) => {
            id.zeroize();
            position.zeroize();
            Err(Error::InvalidColumnType(
                4,
                e.to_string(),
                rusqlite::types::Type::Text,
            ))
        }
    }
}

fn phone_number_from_database(
    mut id: u64,
    label: String,
    mut position: u32,
    required: bool,
    value: String,
) -> Result<specific::PhoneNumber, Error> {
    match specific::PhoneNumber::new(label, required, value) {
        Ok(mut phone_number) => {
            phone_number.set_id(id);
            phone_number.set_position(position);
            Ok(phone_number)
        }
        Err(e) => {
            id.zeroize();
            position.zeroize();
            Err(Error::InvalidColumnType(
                4,
                e.to_string(),
                rusqlite::types::Type::Text,
            ))
        }
    }
}

fn bank_card_number_from_database(
    mut id: u64,
    label: String,
    mut position: u32,
    required: bool,
    value: String,
) -> Result<specific::BankCardNumber, Error> {
    match specific::BankCardNumber::new(label, required, value) {
        Ok(mut bank_card_number) => {
            bank_card_number.set_id(id);
            bank_card_number.set_position(position);
            Ok(bank_card_number)
        }
        Err(e) => {
            id.zeroize();
            position.zeroize();
            Err(Error::InvalidColumnType(
                4,
                e.to_string(),
                rusqlite::types::Type::Text,
            ))
        }
    }
}

fn record_from_database(
    id: u64,
    name: String,
    created: chrono::DateTime<chrono::Local>,
    last_modified: chrono::DateTime<chrono::Local>,
    category: Category,
) -> Record {
    let mut record = Record::new(name, category);
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
        Category::from_string(row.get(4)?),
    ))
}

pub fn row_to_content(row: &Row) -> Result<Content> {
    let id: u64 = row.get(0)?;
    let label: String = row.get(1)?;
    let position: u32 = row.get(2)?;
    let required: bool = row.get(3)?;
    let type_: String = row.get(4)?;
    let value: String = row.get(5)?;
    Ok(match type_.as_str() {
        "Number" => Content::Number(number_from_database(id, label, position, required, value)?),
        "NormalText" => Content::Text(text_from_database(
            id,
            label,
            position,
            required,
            value,
            basic::TextType::Normal,
        )),
        "LongText" => Content::Text(text_from_database(
            id,
            label,
            position,
            required,
            value,
            basic::TextType::Long,
        )),
        "SensitiveText" => Content::Text(text_from_database(
            id,
            label,
            position,
            required,
            value,
            basic::TextType::Sensitive,
        )),
        "Datetime" => Content::Datetime(datetime_from_database(
            id, label, position, required, value,
        )?),
        "Password" => {
            Content::Password(password_from_database(id, label, position, required, value))
        }
        "TOTP" => Content::Totp(totp_from_database(id, label, position, required, value)?),
        "URL" => Content::Url(url_from_database(id, label, position, required, value)?),
        "Email" => Content::Email(email_from_database(id, label, position, required, value)?),
        "PhoneNumber" => Content::PhoneNumber(phone_number_from_database(
            id, label, position, required, value,
        )?),
        "BankCardNumber" => Content::BankCardNumber(bank_card_number_from_database(
            id, label, position, required, value,
        )?),
        e => {
            return Err(Error::InvalidColumnType(
                4,
                e.to_string(),
                rusqlite::types::Type::Text,
            ))
        }
    })
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
