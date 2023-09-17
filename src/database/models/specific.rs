use crate::models::traits::{Id, Label, Position, Required, ToSecretString};
use crate::{
    impl_id, impl_label, impl_position, impl_required, impl_to_secret_string, utils::validate,
};

use secrecy::SecretString;
use std::ops::Not;
use totp_rs::{Rfc6238, TOTP as TOTP_RS};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct TOTP {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    totp: TOTP_RS,
}

impl TOTP {
    pub fn from_url(label: String, required: bool, url: String) -> Result<TOTP, &'static str> {
        TOTP::from_database(0, label, 0, required, url)
    }
    pub fn from_database(
        mut id: u64,
        mut label: String,
        mut position: u32,
        mut required: bool,
        mut url: String,
    ) -> Result<TOTP, &'static str> {
        let Ok(totp) = TOTP_RS::from_url(&url) else {
            id.zeroize();
            label.zeroize();
            required.zeroize();
            position.zeroize();
            url.zeroize();
            return Err("Invalid OTP Auth URL");
        };
        url.zeroize();
        Ok(TOTP {
            id,
            label,
            position,
            required,
            totp,
        })
    }
    pub fn from_secret(
        mut label: String,
        mut required: bool,
        secret: String,
    ) -> Result<TOTP, &'static str> {
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
        Ok(TOTP {
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

impl ToSecretString for TOTP {
    fn to_secret_string(&self) -> SecretString {
        SecretString::new(self.totp.get_url())
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct URL {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
}

impl URL {
    pub fn new(label: String, required: bool, value: String) -> Result<URL, &'static str> {
        URL::from_database(0, label, 0, required, value)
    }
    pub fn from_database(
        mut id: u64,
        mut label: String,
        mut position: u32,
        mut required: bool,
        mut value: String,
    ) -> Result<URL, &'static str> {
        if validate::is_url(value.as_str()).not() {
            id.zeroize();
            label.zeroize();
            position.zeroize();
            required.zeroize();
            value.zeroize();
            return Err("Invalid URL");
        };
        Ok(URL {
            id,
            label,
            position,
            required,
            value,
        })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, mut value: String) -> Result<(), &'static str> {
        if validate::is_url(value.as_str()).not() {
            value.zeroize();
            return Err("Invalid URL");
        }
        self.value.zeroize();
        self.value = value;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct Password {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
}

impl Password {
    pub fn new(label: String, required: bool, value: String) -> Password {
        Password::from_database(0, label, 0, required, value)
    }
    pub fn from_database(
        id: u64,
        label: String,
        position: u32,
        required: bool,
        value: String,
    ) -> Password {
        Password {
            id,
            label,
            position,
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

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct Email {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
}

impl Email {
    pub fn new(label: String, required: bool, value: String) -> Result<Email, &'static str> {
        Email::from_database(0, label, 0, required, value)
    }
    pub fn from_database(
        mut id: u64,
        mut label: String,
        mut position: u32,
        mut required: bool,
        mut value: String,
    ) -> Result<Email, &'static str> {
        if validate::is_email(value.as_str()).not() {
            id.zeroize();
            label.zeroize();
            position.zeroize();
            required.zeroize();
            value.zeroize();
            return Err("Invalid email");
        };
        Ok(Email {
            id,
            label,
            position,
            required,
            value,
        })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, mut value: String) -> Result<(), &'static str> {
        if validate::is_email(value.as_str()).not() {
            value.zeroize();
            return Err("Invalid email");
        }
        self.value.zeroize();
        self.value = value;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct PhoneNumber {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
}

impl PhoneNumber {
    pub fn new(label: String, required: bool, value: String) -> Result<PhoneNumber, &'static str> {
        PhoneNumber::from_database(0, label, 0, required, value)
    }
    pub fn from_database(
        mut id: u64,
        mut label: String,
        mut position: u32,
        mut required: bool,
        mut value: String,
    ) -> Result<PhoneNumber, &'static str> {
        if validate::is_phone_number(value.as_str()).not() {
            id.zeroize();
            label.zeroize();
            position.zeroize();
            required.zeroize();
            value.zeroize();
            return Err("Invalid phone number");
        };
        Ok(PhoneNumber {
            id,
            label,
            position,
            required,
            value,
        })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, mut value: String) -> Result<(), &'static str> {
        if validate::is_phone_number(value.as_str()).not() {
            value.zeroize();
            return Err("Invalid phone number");
        }
        self.value.zeroize();
        self.value = value;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct BankCardNumber {
    id: u64,
    label: String,
    position: u32,
    required: bool,
    value: String,
}

impl BankCardNumber {
    pub fn new(
        label: String,
        required: bool,
        value: String,
    ) -> Result<BankCardNumber, &'static str> {
        BankCardNumber::from_database(0, label, 0, required, value)
    }
    pub fn from_database(
        mut id: u64,
        mut label: String,
        mut position: u32,
        mut required: bool,
        mut value: String,
    ) -> Result<BankCardNumber, &'static str> {
        if validate::card::is_luhn_valid(value.as_str()).not() {
            id.zeroize();
            label.zeroize();
            position.zeroize();
            required.zeroize();
            value.zeroize();
            return Err("Invalid credit card number");
        };
        Ok(BankCardNumber {
            id,
            label,
            position,
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

impl_id!(for TOTP, URL, Password, Email, PhoneNumber, BankCardNumber);
impl_label!(for TOTP, URL, Password, Email, PhoneNumber, BankCardNumber);
impl_position!(for TOTP, URL, Password, Email, PhoneNumber, BankCardNumber);
impl_required!(for TOTP, URL, Password, Email, PhoneNumber, BankCardNumber);
impl_to_secret_string!(for URL, Password, Email, PhoneNumber, BankCardNumber);

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
