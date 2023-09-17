use crate::models::traits::{Id, Label, Position, Required, ToSecretString};
use crate::{impl_id, impl_label, impl_position, impl_required, impl_to_secret_string};

use chrono::ParseError;
use secrecy::SecretString;
use std::num::ParseIntError;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct Number {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: i128,
}

impl Number {
    pub fn new(label: String, required: bool, value: i128) -> Self {
        Self {
            id: 0,
            label,
            position: 0,
            required,
            value,
        }
    }
    pub fn from_database(
        mut id: u64,
        mut label: String,
        mut position: u32,
        mut required: bool,
        mut value: String,
    ) -> Result<Number, ParseIntError> {
        let new_value = value.parse::<i128>();
        value.zeroize();
        match new_value {
            Ok(value) => Ok(Number {
                id,
                label,
                position,
                required,
                value,
            }),
            Err(e) => {
                id.zeroize();
                label.zeroize();
                position.zeroize();
                required.zeroize();
                Err(e)
            }
        }
    }
    pub fn value(&self) -> &i128 {
        &self.value
    }
    pub fn set_value(&mut self, value: i128) {
        self.value.zeroize();
        self.value = value;
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub enum TextType {
    Normal,
    Long,
    Sensitive,
}
#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct Text {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
    text_type: TextType,
}

impl Text {
    pub fn new(label: String, required: bool, value: String, text_type: TextType) -> Self {
        Text::from_database(0, label, 0, required, value, text_type)
    }
    pub fn from_database(
        id: u64,
        label: String,
        position: u32,
        required: bool,
        value: String,
        text_type: TextType,
    ) -> Self {
        Self {
            id,
            label,
            position,
            required,
            value,
            text_type,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn text_type(&self) -> &TextType {
        &self.text_type
    }
    pub fn set_value(&mut self, value: String) {
        self.value.zeroize();
        self.value = value;
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct Datetime {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    #[zeroize(skip)]
    value: chrono::DateTime<chrono::Local>,
}

impl Datetime {
    pub fn new(label: String, required: bool, value: chrono::DateTime<chrono::Local>) -> Datetime {
        Datetime {
            id: 0,
            label,
            position: 0,
            required,
            value,
        }
    }
    pub fn from_database(
        mut id: u64,
        mut label: String,
        mut position: u32,
        mut required: bool,
        mut value: String,
    ) -> Result<Datetime, ParseError> {
        let new_value: Result<chrono::DateTime<chrono::Local>, ParseError> =
            std::str::FromStr::from_str(value.as_str());
        value.zeroize();
        match new_value {
            Ok(value) => Ok(Datetime {
                id,
                label,
                position,
                required,
                value,
            }),
            Err(e) => {
                id.zeroize();
                label.zeroize();
                position.zeroize();
                required.zeroize();
                Err(e)
            }
        }
    }

    pub fn value(&self) -> &chrono::DateTime<chrono::Local> {
        &self.value
    }
    pub fn set_value(&mut self, value: chrono::DateTime<chrono::Local>) {
        self.value = value;
    }
}

impl_id!(for Number, Text, Datetime);
impl_label!(for Number, Text, Datetime);
impl_position!(for Number, Text, Datetime);
impl_required!(for Number, Text, Datetime);
impl_to_secret_string!(for Number, Text, Datetime);

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
