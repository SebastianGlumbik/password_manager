#![allow(dead_code)]
use secrecy::SecretString;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Not;
use totp_rs::{Rfc6238, TOTP};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Number value
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Number {
    value: String,
}

impl Number {
    /// Create a new Number
    /// # Errors
    /// Returns an error if the value is not valid i64 number
    pub fn new(value: String) -> Result<Number, &'static str> {
        value
            .parse::<i64>()
            .map(|mut value| value.zeroize())
            .map_err(|_| "Invalid number")?;

        Ok(Number { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Text value
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Text {
    value: String,
}

impl Text {
    pub fn new(value: String) -> Text {
        Text { value }
    }
    #[allow(dead_code)]
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Long text value
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct LongText {
    value: String,
}

impl LongText {
    pub fn new(value: String) -> LongText {
        LongText { value }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Sensitive text value
/// This value is not serialized
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct SensitiveText {
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
}

/// Date value
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Date {
    value: String,
}

impl Date {
    /// Create a new Date
    /// # Errors
    /// Returns an error if the value is not valid [`chrono::NaiveDate`] date
    pub fn new(value: String) -> Result<Date, &'static str> {
        value
            .parse::<chrono::NaiveDate>()
            .map(|mut _value| _value = chrono::NaiveDate::default())
            .map_err(|_| "Invalid date")?;

        Ok(Date { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Password value
/// This value is not serialized
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize, Deserialize)]
pub struct Password {
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
}

/// TOTP Secret value
/// This value is not serialized
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct TOTPSecret {
    #[serde(skip_serializing)]
    value: String,
}

impl TOTPSecret {
    /// Create a new TOTPSecret
    /// # Errors
    /// Returns an error if the value is not valid OTP Secret
    pub fn new(value: String) -> Result<TOTPSecret, &'static str> {
        let secret = totp_rs::Secret::Encoded(value)
            .to_bytes()
            .map_err(|_| "Invalid OTP Secret")?;
        let rfc6238 = Rfc6238::with_defaults(secret).map_err(|_| "Invalid OTP Secret")?;
        let totp = TOTP::from_rfc6238(rfc6238).map_err(|_| "Invalid OTP Secret")?;
        Ok(TOTPSecret {
            value: totp.get_secret_base32(),
        })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// URL value
/// Can be URL, IPv4 or IPv6
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Url {
    value: String,
}

impl Url {
    /// Create a new Url
    /// # Errors
    /// Returns an error if the value is not valid URL, IPv4 or IPv6
    pub fn new(mut value: String) -> Result<Url, &'static str> {
        if validator::validate_url(value.as_str()).not()
            && validator::validate_ip_v4(value.as_str()).not()
            && validator::validate_ip_v6(value.as_str()).not()
        {
            value.zeroize();
            return Err("Invalid URL");
        };

        Ok(Url { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Email value
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct Email {
    value: String,
}

impl Email {
    /// Create a new Email
    /// # Errors
    /// Returns an error if the value is not valid email
    pub fn new(mut value: String) -> Result<Email, &'static str> {
        if validator::validate_email(value.as_str()).not() {
            value.zeroize();
            return Err("Invalid email");
        };
        Ok(Email { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Phone number value
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct PhoneNumber {
    value: String,
}

impl PhoneNumber {
    /// Create a new PhoneNumber
    /// # Errors
    /// Returns an error if the value is not valid international phone number
    pub fn new(mut value: String) -> Result<PhoneNumber, &'static str> {
        if validator::validate_phone(value.as_str()).not() {
            value.zeroize();
            return Err("Invalid phone number");
        };

        Ok(PhoneNumber { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Bank card number value
#[derive(Debug, PartialEq, Default, Zeroize, ZeroizeOnDrop, Serialize)]
pub struct BankCardNumber {
    #[serde(skip_serializing)]
    value: String,
}

impl BankCardNumber {
    /// Create a new BankCardNumber
    /// # Errors
    /// Returns an error if the value is not valid bank card number
    pub fn new(mut value: String) -> Result<BankCardNumber, &'static str> {
        if let Err(error) = card_validate::Validate::from(value.as_str()) {
            value.zeroize();
            return Err(match error {
                card_validate::ValidateError::InvalidFormat => "Invalid Format",
                card_validate::ValidateError::InvalidLength => "Invalid Length",
                card_validate::ValidateError::InvalidLuhn => "Invalid Luhn",
                card_validate::ValidateError::UnknownType => "Unknown Type",
                _ => "Unknown Error",
            });
        }

        Ok(BankCardNumber { value })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

pub trait ToSecretString {
    fn to_secret_string(&self) -> SecretString;
}
macro_rules! impl_to_secret_string {
    (for $($t:ty),+) => {
        $(impl ToSecretString for $t {
            /// Convert value to SecretString
            fn to_secret_string(&self) -> SecretString {
                SecretString::new(self.value.to_string())
            }
        })*
    }
}

impl_to_secret_string!(for Number, Text, LongText, SensitiveText, Date, Password, TOTPSecret, Url, Email, PhoneNumber, BankCardNumber);

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
                            None => Err(de::Error::invalid_length(0, &self))
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
                            None => Err(de::Error::missing_field("value"))
                        }
                    }
                }

                const FIELDS: &'static [&'static str] = &["value"];
                deserializer.deserialize_struct(stringify!($t), FIELDS, MyVisitor)
            }
        })*
    }
}

impl_deserialize!(for Number, Date, TOTPSecret, Url, Email, PhoneNumber, BankCardNumber);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_empty() {
        let number = Number::new("".to_string());
        assert!(number.is_err());
    }
    #[test]
    fn test_number_invalid_string() {
        let number = Number::new("invalid".to_string());
        assert!(number.is_err());
    }

    #[test]
    fn test_number_invalid_float() {
        let number = Number::new("1.1".to_string());
        assert!(number.is_err());
    }
    #[test]
    fn test_number_wrong_format_comma() {
        let number = Number::new("1,000".to_string());
        assert!(number.is_err());
    }
    #[test]
    fn test_number_wrong_format_space() {
        let number = Number::new("1 000".to_string());
        assert!(number.is_err());
    }
    #[test]
    fn test_number_valid() {
        let number = Number::new("1000".to_string());
        assert!(number.is_ok());
        assert_eq!(number.unwrap().value(), "1000");
    }
    #[test]
    fn test_number_deserialize_empty() {
        let number = serde_json::from_str::<Number>(r#"{}"#);
        assert!(number.is_err());
    }
    #[test]
    fn test_number_deserialize_invalid() {
        let number = serde_json::from_str::<Number>(r#"{"value":"invalid"}"#);
        assert!(number.is_err());
    }
    #[test]
    fn test_number_deserialize_valid() {
        let number = serde_json::from_str::<Number>(r#"{"value":"1000"}"#);
        assert!(number.is_ok());
        assert_eq!(number.unwrap().value(), "1000");
    }
    #[test]
    fn test_text() {
        let text = Text::new("text".to_string());
        assert_eq!(text.value(), "text");
    }
    #[test]
    fn test_long_text() {
        let long_text = LongText::new("long text".to_string());
        assert_eq!(long_text.value(), "long text");
    }
    #[test]
    fn test_sensitive_text() {
        let sensitive_text = SensitiveText::new("sensitive text".to_string());
        assert_eq!(sensitive_text.value(), "sensitive text");
    }
    #[test]
    fn test_sensitive_text_serialize() {
        let sensitive_text = SensitiveText::new("sensitive text".to_string());
        let serialized = serde_json::to_string(&sensitive_text).unwrap();
        assert_eq!(serialized, r#"{}"#);
    }
    #[test]
    fn test_date_empty() {
        let date = Date::new("".to_string());
        assert!(date.is_err());
    }
    #[test]
    fn test_date_invalid() {
        let date = Date::new("invalid".to_string());
        assert!(date.is_err());
    }
    #[test]
    fn test_date_valid() {
        let date = Date::new("2021-01-01".to_string());
        assert!(date.is_ok());
        assert_eq!(date.unwrap().value(), "2021-01-01");
    }
    #[test]
    fn test_date_deserialize_empty() {
        let date = serde_json::from_str::<Date>(r#"{}"#);
        assert!(date.is_err());
    }
    #[test]
    fn test_date_deserialize_invalid() {
        let date = serde_json::from_str::<Date>(r#"{"value":"invalid"}"#);
        assert!(date.is_err());
    }
    #[test]
    fn test_date_deserialize_valid() {
        let date = serde_json::from_str::<Date>(r#"{"value":"2021-01-01"}"#);
        assert!(date.is_ok());
        assert_eq!(date.unwrap().value(), "2021-01-01");
    }
    #[test]
    fn test_password() {
        let password = Password::new("password".to_string());
        assert_eq!(password.value(), "password");
    }
    #[test]
    fn test_password_serialize() {
        let password = Password::new("password".to_string());
        let serialized = serde_json::to_string(&password).unwrap();
        assert_eq!(serialized, r#"{}"#);
    }

    #[test]
    fn test_totp_secret_empty() {
        let totp_secret = TOTPSecret::new("".to_string());
        assert!(totp_secret.is_err());
    }
    #[test]
    fn test_totp_secret_invalid() {
        let totp_secret = TOTPSecret::new("invalid".to_string());
        assert!(totp_secret.is_err());
    }
    #[test]
    fn test_totp_secret_valid() {
        let totp_secret = TOTPSecret::new("rfffmaz4jsjq3qurwhzna2wljastmywv".to_string());
        assert!(totp_secret.is_ok());
        assert_eq!(
            totp_secret.unwrap().value(),
            "rfffmaz4jsjq3qurwhzna2wljastmywv".to_uppercase()
        );
    }
    #[test]
    fn test_totp_secret_serialize() {
        let totp_secret = TOTPSecret::new("rfffmaz4jsjq3qurwhzna2wljastmywv".to_string());
        assert!(totp_secret.is_ok());
        let totp_secret = totp_secret.unwrap();
        let serialized = serde_json::to_string(&totp_secret).unwrap();
        assert_eq!(serialized, r#"{}"#);
    }
    #[test]
    fn test_totp_secret_deserialize_empty() {
        let totp_secret = serde_json::from_str::<TOTPSecret>(r#"{}"#);
        assert!(totp_secret.is_err());
    }
    #[test]
    fn test_totp_secret_deserialize_invalid() {
        let totp_secret = serde_json::from_str::<TOTPSecret>(r#"{"value":"invalid"}"#);
        assert!(totp_secret.is_err());
    }
    #[test]
    fn test_totp_secret_deserialize_valid() {
        let totp_secret =
            serde_json::from_str::<TOTPSecret>(r#"{"value":"rfffmaz4jsjq3qurwhzna2wljastmywv"}"#);
        assert!(totp_secret.is_ok());
        assert_eq!(
            totp_secret.unwrap().value(),
            "rfffmaz4jsjq3qurwhzna2wljastmywv".to_uppercase()
        );
    }
    #[test]
    fn test_url_empty() {
        let url = Url::new("".to_string());
        assert!(url.is_err());
    }
    #[test]
    fn test_url_invalid() {
        let url = Url::new("invalid".to_string());
        assert!(url.is_err());
    }
    #[test]
    fn test_url_valid_url() {
        let url = Url::new("https://www.example.com".to_string());
        assert!(url.is_ok());
        assert_eq!(url.unwrap().value(), "https://www.example.com");
    }
    #[test]
    fn test_url_valid_ipv4() {
        let url = Url::new("1.1.1.1".to_string());
        assert!(url.is_ok());
        assert_eq!(url.unwrap().value(), "1.1.1.1".to_string());
    }
    #[test]
    fn test_url_valid_ipv6() {
        let url = Url::new("2606:4700:4700::1111".to_string());
        assert!(url.is_ok());
        assert_eq!(url.unwrap().value(), "2606:4700:4700::1111".to_string());
    }
    #[test]
    fn test_url_deserialize_empty() {
        let url = serde_json::from_str::<Url>(r#"{}"#);
        assert!(url.is_err());
    }
    #[test]
    fn test_url_deserialize_invalid() {
        let url = serde_json::from_str::<Url>(r#"{"value":"invalid"}"#);
        assert!(url.is_err());
    }
    #[test]
    fn test_url_deserialize_valid() {
        let url = serde_json::from_str::<Url>(r#"{"value":"https://www.example.com"}"#);
        assert!(url.is_ok());
        assert_eq!(url.unwrap().value(), "https://www.example.com");
    }
    #[test]
    fn test_email_empty() {
        let email = Email::new("".to_string());
        assert!(email.is_err());
    }
    #[test]
    fn test_email_invalid() {
        let email = Email::new("invalid".to_string());
        assert!(email.is_err());
    }
    #[test]
    fn test_email_valid() {
        let email = Email::new("example@email.com".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().value(), "example@email.com".to_string());
    }
    #[test]
    fn test_email_deserialize_empty() {
        let email = serde_json::from_str::<Email>(r#"{}"#);
        assert!(email.is_err());
    }
    #[test]
    fn test_email_deserialize_invalid() {
        let email = serde_json::from_str::<Email>(r#"{"value":"invalid"}"#);
        assert!(email.is_err());
    }
    #[test]
    fn test_email_deserialize_valid() {
        let email = serde_json::from_str::<Email>(r#"{"value":"example@email.com"}"#);
        assert!(email.is_ok());
        assert_eq!(email.unwrap().value(), "example@email.com");
    }
    #[test]
    fn test_phone_number_empty() {
        let phone_number = PhoneNumber::new("".to_string());
        assert!(phone_number.is_err());
    }
    #[test]
    fn test_phone_number_invalid() {
        let phone_number = PhoneNumber::new("invalid".to_string());
        assert!(phone_number.is_err());
    }
    #[test]
    fn test_phone_number_valid() {
        let phone_number = PhoneNumber::new("+14152370800".to_string());
        assert!(phone_number.is_ok());
        assert_eq!(phone_number.unwrap().value(), "+14152370800");
    }
    #[test]
    fn test_phone_number_deserialize_empty() {
        let phone_number = serde_json::from_str::<PhoneNumber>(r#"{}"#);
        assert!(phone_number.is_err());
    }
    #[test]
    fn test_phone_number_deserialize_invalid() {
        let phone_number = serde_json::from_str::<PhoneNumber>(r#"{"value":"invalid"}"#);
        assert!(phone_number.is_err());
    }
    #[test]
    fn test_phone_number_deserialize_valid() {
        let phone_number = serde_json::from_str::<PhoneNumber>(r#"{"value":"+14152370800"}"#);
        assert!(phone_number.is_ok());
        assert_eq!(phone_number.unwrap().value(), "+14152370800");
    }
    #[test]
    fn test_ank_card_number_empty() {
        let bank_card_number = BankCardNumber::new("".to_string());
        assert!(bank_card_number.is_err());
    }
    #[test]
    fn test_bank_card_number_invalid() {
        let bank_card_number = BankCardNumber::new("invalid".to_string());
        assert!(bank_card_number.is_err());
    }
    #[test]
    fn test_bank_card_number_valid() {
        let bank_card_number = BankCardNumber::new("4702932172193242".to_string());
        assert!(bank_card_number.is_ok());
        assert_eq!(bank_card_number.unwrap().value(), "4702932172193242");
    }
    #[test]
    fn test_bank_card_number_serialize() {
        let bank_card_number = BankCardNumber::new("4702932172193242".to_string());
        assert!(bank_card_number.is_ok());
        let bank_card_number = bank_card_number.unwrap();
        let serialized = serde_json::to_string(&bank_card_number).unwrap();
        assert_eq!(serialized, r#"{}"#);
    }
    #[test]
    fn test_bank_card_number_deserialize_empty() {
        let bank_card_number = serde_json::from_str::<BankCardNumber>(r#"{}"#);
        assert!(bank_card_number.is_err());
    }
    #[test]
    fn test_bank_card_number_deserialize_invalid() {
        let bank_card_number = serde_json::from_str::<BankCardNumber>(r#"{"value":"invalid"}"#);
        assert!(bank_card_number.is_err());
    }
    #[test]
    fn test_bank_card_number_deserialize_valid() {
        let bank_card_number =
            serde_json::from_str::<BankCardNumber>(r#"{"value":"4702932172193242"}"#);
        assert!(bank_card_number.is_ok());
        assert_eq!(bank_card_number.unwrap().value(), "4702932172193242");
    }
}
