use crate::database::models::traits::{Id, Label, Position, Required, ToSecretString};
use crate::{
    impl_id, impl_label, impl_position, impl_required, impl_to_secret_string, utils::validate,
};

use regex::Regex;
use secrecy::SecretString;
use serde::Serialize;
use std::ops::Not;
use totp_rs::{Rfc6238, TOTP as TOTP_RS};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Totp {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    #[serde(skip)]
    totp: TOTP_RS,
}

impl Totp {
    pub fn from_url(
        mut label: String,
        mut required: bool,
        mut url: String,
    ) -> Result<Totp, &'static str> {
        let Ok(totp) = TOTP_RS::from_url(&url) else {
            label.zeroize();
            required.zeroize();
            url.zeroize();
            return Err("Invalid OTP Auth URL");
        };
        url.zeroize();
        Ok(Totp {
            id: 0,
            label,
            position: 0,
            required,
            totp,
        })
    }
    pub fn from_secret(
        mut label: String,
        mut required: bool,
        secret: String,
    ) -> Result<Totp, &'static str> {
        let Ok(secret) = totp_rs::Secret::Encoded(secret).to_bytes() else {
            label.zeroize();
            required.zeroize();
            return Err("Invalid OTP Secret");
        };
        let Ok(rfc6238) = Rfc6238::with_defaults(secret) else {
            label.zeroize();
            required.zeroize();
            return Err("Invalid OTP Secret");
        };
        let Ok(totp) = TOTP_RS::from_rfc6238(rfc6238) else {
            label.zeroize();
            required.zeroize();
            return Err("Invalid OTP Secret");
        };
        Ok(Totp {
            id: 0,
            label,
            position: 0,
            required,
            totp,
        })
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Url {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
}

impl Url {
    pub fn new(
        mut label: String,
        mut required: bool,
        mut value: String,
    ) -> Result<Url, &'static str> {
        if Regex::new(r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_+.~#?&/=]*)").unwrap().is_match(value.as_ref()).not() {
            label.zeroize();
            required.zeroize();
            value.zeroize();
            return Err("Invalid URL");
        };
        Ok(Url {
            id: 0,
            label,
            position: 0,
            required,
            value,
        })
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Password {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    #[serde(skip)]
    value: String,
}

impl Password {
    pub fn new(label: String, required: bool, value: String) -> Password {
        Password {
            id: 0,
            label,
            position: 0,
            required,
            value,
        }
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
pub struct Email {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
}

impl Email {
    pub fn new(
        mut label: String,
        mut required: bool,
        mut value: String,
    ) -> Result<Email, &'static str> {
        if Regex::new(r"([a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)")
            .unwrap()
            .is_match(value.as_ref())
            .not()
        {
            // https://emailregex.com/
            label.zeroize();
            required.zeroize();
            value.zeroize();
            return Err("Invalid email");
        };
        Ok(Email {
            id: 0,
            label,
            position: 0,
            required,
            value,
        })
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct PhoneNumber {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
}

impl PhoneNumber {
    pub fn new(
        mut label: String,
        mut required: bool,
        mut value: String,
    ) -> Result<PhoneNumber, &'static str> {
        if Regex::new(r"(\+[1-9]{1,4})?[0-9]([0-9]*)")
            .unwrap()
            .is_match(value.as_ref())
            .not()
        {
            label.zeroize();
            required.zeroize();
            value.zeroize();
            return Err("Invalid phone number");
        };
        Ok(PhoneNumber {
            id: 0,
            label,
            position: 0,
            required,
            value,
        })
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct BankCardNumber {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    #[serde(skip)]
    value: String,
}

impl BankCardNumber {
    pub fn new(
        mut label: String,
        mut required: bool,
        mut value: String,
    ) -> Result<BankCardNumber, &'static str> {
        if validate::card::is_luhn_valid(value.as_str()).not() {
            label.zeroize();
            required.zeroize();
            value.zeroize();
            return Err("Invalid credit card number");
        };
        Ok(BankCardNumber {
            id: 0,
            label,
            position: 0,
            required,
            value,
        })
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

impl_id!(for Totp, Url, Password, Email, PhoneNumber, BankCardNumber);
impl_label!(for Totp, Url, Password, Email, PhoneNumber, BankCardNumber);
impl_position!(for Totp, Url, Password, Email, PhoneNumber, BankCardNumber);
impl_required!(for Totp, Url, Password, Email, PhoneNumber, BankCardNumber);
impl_to_secret_string!(for Url, Password, Email, PhoneNumber, BankCardNumber);

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
