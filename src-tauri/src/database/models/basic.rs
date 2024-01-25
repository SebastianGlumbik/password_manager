use crate::database::models::traits::{Id, Label, Position, Required, ToSecretString};
use crate::{impl_id, impl_label, impl_position, impl_required, impl_to_secret_string};

use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
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
    pub fn value(&self) -> &i128 {
        &self.value
    }
    pub fn set_value(&mut self, value: i128) {
        self.value.zeroize();
        self.value = value;
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub enum TextType {
    Normal,
    Long,
    Sensitive,
}

//TODO Skip sensitive text
#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Text {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
    text_type: TextType,
}

impl Text {
    pub fn new(label: String, required: bool, value: String, text_type: TextType) -> Text {
        Text {
            id: 0,
            label,
            position: 0,
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
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
