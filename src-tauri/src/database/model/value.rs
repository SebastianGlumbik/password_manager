use crate::utils::validate;
use regex::Regex;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use std::ops::Not;
use totp_rs::{Rfc6238, TOTP as TOTP_RS};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Number {
    value: i128,
}

impl Number {
    pub fn new(value: i128) -> Number {
        Number { value }
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Totp {
    #[serde(skip)]
    totp: TOTP_RS,
}

impl Totp {
    pub fn from_url(mut url: String) -> Result<Totp, &'static str> {
        let Ok(totp) = TOTP_RS::from_url(&url) else {
            url.zeroize();
            return Err("Invalid OTP Auth URL");
        };
        url.zeroize();
        Ok(Totp { totp })
    }
    pub fn from_secret(secret: String) -> Result<Totp, &'static str> {
        let Ok(secret) = totp_rs::Secret::Encoded(secret).to_bytes() else {
            return Err("Invalid OTP Secret");
        };
        let Ok(rfc6238) = Rfc6238::with_defaults(secret) else {
            return Err("Invalid OTP Secret");
        };
        let Ok(totp) = TOTP_RS::from_rfc6238(rfc6238) else {
            return Err("Invalid OTP Secret");
        };
        Ok(Totp { totp })
    }
    pub fn totp(&self) -> &TOTP_RS {
        &self.totp
    }
}

impl ToSecretString for Totp {
    fn to_secret_string(&self) -> SecretString {
        SecretString::new(self.totp.get_url())
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct BankCardNumber {
    #[serde(default)]
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

impl_to_secret_string!(for Number, Text, SensitiveText, Datetime, Password, Url, Email, PhoneNumber, BankCardNumber);

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
