use crate::utils::validate;
use regex::Regex;
use secrecy::SecretString;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Not;
use totp_rs::{Rfc6238, TOTP as TOTP_RS};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Number {
    value: i64,
}

impl Number {
    pub fn new(value: i64) -> Number {
        Number { value }
    }
    pub fn value(&self) -> &i64 {
        &self.value
    }
    pub fn set_value(&mut self, value: i64) {
        self.value.zeroize();
        self.value = value;
    }
}

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Text {
    value: String,
}

impl Text {
    pub fn new(value: String) -> Text {
        Text { value }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, value: String) {
        self.value.zeroize();
        self.value = value;
    }
}

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct SensitiveText {
    #[serde(default)]
    #[serde(skip_serializing)]
    value: String,
}

impl SensitiveText {
    pub fn new(value: String) -> SensitiveText {
        SensitiveText { value }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, value: String) {
        self.value.zeroize();
        self.value = value;
    }
}

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Datetime {
    #[zeroize(skip)]
    value: chrono::DateTime<chrono::Local>,
}

impl Datetime {
    pub fn new(value: chrono::DateTime<chrono::Local>) -> Datetime {
        Datetime { value }
    }

    pub fn value(&self) -> &chrono::DateTime<chrono::Local> {
        &self.value
    }
    pub fn set_value(&mut self, value: chrono::DateTime<chrono::Local>) {
        self.value = value;
    }
}

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Password {
    #[serde(default)]
    #[serde(skip_serializing)]
    value: String,
}

impl Password {
    pub fn new(value: String) -> Password {
        Password { value }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, value: String) {
        self.value.zeroize();
        self.value = value;
    }
}

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct TOTPSecret {
    #[serde(skip_serializing)]
    value: String,
}

impl TOTPSecret {
    pub fn new(value: String) -> Result<TOTPSecret, &'static str> {
        let Ok(secret) = totp_rs::Secret::Encoded(value).to_bytes() else {
            return Err("Invalid OTP Secret");
        };
        let Ok(rfc6238) = Rfc6238::with_defaults(secret) else {
            return Err("Invalid OTP Secret");
        };
        let Ok(totp) = TOTP_RS::from_rfc6238(rfc6238) else {
            return Err("Invalid OTP Secret");
        };
        Ok(TOTPSecret {
            value: totp.get_secret_base32(),
        })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Url {
    value: String,
}

impl Url {
    pub fn new(mut value: String) -> Result<Url, &'static str> {
        if Regex::new(r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_+.~#?&/=]*)").unwrap().is_match(value.as_ref()).not() {
            value.zeroize();
            return Err("Invalid URL");
        };
        Ok(Url { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, mut value: String) -> Result<(), &'static str> {
        if Regex::new(r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_+.~#?&/=]*)").unwrap().is_match(value.as_ref()).not() {
            value.zeroize();
            return Err("Invalid URL");
        }
        self.value.zeroize();
        self.value = value;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(mut value: String) -> Result<Email, &'static str> {
        if Regex::new(r"([a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)")
            .unwrap()
            .is_match(value.as_ref())
            .not()
        {
            // https://emailregex.com/
            value.zeroize();
            return Err("Invalid email");
        };
        Ok(Email { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, mut value: String) -> Result<(), &'static str> {
        if Regex::new(r"([a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)")
            .unwrap()
            .is_match(value.as_ref())
            .not()
        {
            value.zeroize();
            return Err("Invalid email");
        }
        self.value.zeroize();
        self.value = value;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct PhoneNumber {
    value: String,
}

impl PhoneNumber {
    pub fn new(mut value: String) -> Result<PhoneNumber, &'static str> {
        if Regex::new(r"(\+[1-9]{1,4})?[0-9]([0-9]*)")
            .unwrap()
            .is_match(value.as_ref())
            .not()
        {
            value.zeroize();
            return Err("Invalid phone number");
        };
        Ok(PhoneNumber { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, mut value: String) -> Result<(), &'static str> {
        if Regex::new(r"(\+[1-9]{1,4})?[0-9]([0-9]*)")
            .unwrap()
            .is_match(value.as_ref())
            .not()
        {
            value.zeroize();
            return Err("Invalid phone number");
        }
        self.value.zeroize();
        self.value = value;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct BankCardNumber {
    #[serde(skip_serializing)]
    value: String,
}

impl BankCardNumber {
    pub fn new(mut value: String) -> Result<BankCardNumber, &'static str> {
        if validate::card::is_luhn_valid(value.as_str()).not() {
            value.zeroize();
            return Err("Invalid credit card number");
        };
        Ok(BankCardNumber { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, mut value: String) -> Result<(), &'static str> {
        if validate::card::is_luhn_valid(value.as_str()).not() {
            value.zeroize();
            return Err("Invalid credit card number");
        }
        self.value.zeroize();
        self.value = value;
        Ok(())
    }
}

pub trait ToSecretString {
    fn to_secret_string(&self) -> SecretString;
}
macro_rules! impl_to_secret_string {
    (for $($t:ty),+) => {
        $(impl ToSecretString for $t {
            fn to_secret_string(&self) -> SecretString {
                SecretString::new(self.value.to_string())
            }
        })*
    }
}

impl_to_secret_string!(for Number, Text, SensitiveText, Datetime, Password, TOTPSecret, Url, Email, PhoneNumber, BankCardNumber);

/// https://serde.rs/deserialize-struct.html
macro_rules! impl_deserialize {
    (for $($t:ty),+) => {
        $(impl<'de> Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                enum Field { Value }

                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        struct FieldVisitor;

                        impl<'de> Visitor<'de> for FieldVisitor {
                            type Value = Field;

                            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                formatter.write_str("value")
                            }

                            fn visit_str<E>(self, value: &str) -> Result<Field, E>
                            where
                                E: de::Error,
                            {
                                match value {
                                    "value" => Ok(Field::Value),
                                    _ => Err(de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                struct MyVisitor;

                impl<'de> Visitor<'de> for MyVisitor {
                    type Value = $t;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(format!("struct {}", stringify!($t)).as_str())
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        match seq.next_element::<String>()? {
                            Some(value) => {
                                Self::Value::new(value).map_err(|error| de::Error::custom(error))
                            }
                            None => Ok(Self::Value::default())
                        }
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                    where
                        V: MapAccess<'de>,
                    {
                        let mut value = None;
                        while let Some(key) = map.next_key()? {
                            match key {
                                Field::Value => {
                                    if value.is_some() {
                                        return Err(de::Error::duplicate_field("value"));
                                    }
                                    value = Some(map.next_value()?);
                                }
                            }
                        }
                        match value {
                            Some(value) => {
                                Self::Value::new(value).map_err(|error| de::Error::custom(error))
                            }
                            None => Ok(Self::Value::default())
                        }
                    }
                }

                const FIELDS: &'static [&'static str] = &["value"];
                deserializer.deserialize_struct(stringify!($t), FIELDS, MyVisitor)
            }
        })*
    }
}

impl_deserialize!(for TOTPSecret,Url, Email, PhoneNumber, BankCardNumber);

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
