use super::*;

/// Validates value based on its kind.
/// - Number: Must be a valid number
/// - LongText: Always valid
/// - Date: Must be a valid date (YYYY-MM-DD)
/// - TOTPSecret: Must be a valid TOTP secret ([`TOTPSecret::new`])
/// - Url: Must be a valid URL ([`validator::validate_url`])
/// - Email: Must be a valid email address ([`validator::validate_email`])
/// - PhoneNumber: Must be a valid phone number ([`validator::validate_phone`])
/// - BankCardNumber: Must be a valid bank card number ([`validate::card::from`])
/// - Other: Must not be empty
/// # Return
/// If the value is valid, returns `None`. If the value is invalid, returns an error message.
#[tauri::command]
pub async fn validate(kind: SecretString, value: SecretString) -> Option<String> {
    match kind.expose_secret().as_str() {
        "Number" => {
            if value
                .expose_secret()
                .parse::<i64>()
                .map(|mut _value| _value.zeroize())
                .is_ok()
            {
                None
            } else {
                Some("Invalid number".to_string())
            }
        }
        "LongText" => None,
        "Date" => {
            if value
                .expose_secret()
                .parse::<chrono::NaiveDate>()
                .map(|mut _value| _value = chrono::NaiveDate::default())
                .is_ok()
            {
                None
            } else {
                Some("Invalid date".to_string())
            }
        }
        "TOTPSecret" => {
            if let Err(error) = value::TOTPSecret::new(value.expose_secret().to_string()) {
                Some(error.to_string())
            } else {
                None
            }
        }
        "Url" => {
            if validator::validate_url(value.expose_secret())
                || validator::validate_ip_v4(value.expose_secret())
                || validator::validate_ip_v6(value.expose_secret())
            {
                None
            } else {
                Some("Invalid URL".to_string())
            }
        }
        "Email" => {
            if validator::validate_email(value.expose_secret()) {
                None
            } else {
                Some("Invalid email".to_string())
            }
        }
        "PhoneNumber" => {
            if validator::validate_phone(value.expose_secret()) {
                None
            } else {
                Some("Invalid phone number".to_string())
            }
        }
        "BankCardNumber" => match card_validate::Validate::from(value.expose_secret()) {
            Ok(_) => None,
            Err(error) => Some(
                match error {
                    card_validate::ValidateError::InvalidFormat => "Invalid Format",
                    card_validate::ValidateError::InvalidLength => "Invalid Length",
                    card_validate::ValidateError::InvalidLuhn => "Invalid Luhn",
                    card_validate::ValidateError::UnknownType => "Unknown Type",
                    _ => "Unknown Error",
                }
                .to_string(),
            ),
        },
        _ => {
            if value.expose_secret().trim().is_empty() {
                Some("Value cannot be empty".to_string())
            } else {
                None
            }
        }
    }
}

/// Returns the type of the bank card number ([`card_validate::Validate::evaluate_type`]). Value is loaded from the database.
/// # Error
/// Returns an error if content cannot be loaded, if the content is not a bank card number or if the card type cannot be evaluated.
#[tauri::command]
pub async fn card_type<'a>(id: u64, database: State<'a, Database>) -> Result<String, &'static str> {
    let card_number = {
        let content = database
            .get_content(id)
            .map_err(|_| "Failed to load content")?;

        let Value::BankCardNumber(card_number) = content.value() else {
            return Err("Content is not a password");
        };

        card_number.to_secret_string()
    };

    Ok(
        match card_validate::Validate::evaluate_type(card_number.expose_secret())
            .map_err(|_| "Failed to evaluate card type")?
        {
            card_validate::Type::VisaElectron => "Visa Electron".to_string(),
            card_validate::Type::Maestro => "Maestro".to_string(),
            card_validate::Type::Forbrugsforeningen => "Forbrugsforeningen".to_string(),
            card_validate::Type::Dankort => "Dankort".to_string(),
            card_validate::Type::Visa => "Visa".to_string(),
            card_validate::Type::MIR => "MIR".to_string(),
            card_validate::Type::MasterCard => "MasterCard".to_string(),
            card_validate::Type::Amex => "American Express".to_string(),
            card_validate::Type::DinersClub => "Diners Club".to_string(),
            card_validate::Type::Discover => "Discover".to_string(),
            card_validate::Type::UnionPay => "UnionPay".to_string(),
            card_validate::Type::JCB => "JCB".to_string(),
            _ => "Unknown".to_string(),
        },
    )
}
