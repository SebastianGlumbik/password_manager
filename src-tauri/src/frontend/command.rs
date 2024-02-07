use super::*;
use crate::database::model::{value, Category, Content, Record, ToSecretString, Value};
use serde::{Serialize, Serializer};
use zeroize::Zeroize;
use zeroize::__internal::AssertZeroize;

/// Window types that can be created.
#[derive(Clone, serde::Serialize)]
pub enum WindowType {
    Login,
    Register,
    Main,
}

/// Creates specific window based on the database state and returns the window type.
/// # Error
/// Returns an error if the window cannot be created.
#[tauri::command]
pub async fn initialize_window<'a>(
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
) -> tauri::Result<WindowType> {
    if connection.is_connected() {
        create_main_window(app_handle)?;
        Ok(WindowType::Main)
    } else if DatabaseConnection::database_exists(app_handle.clone()) {
        create_login_window(app_handle)?;
        Ok(WindowType::Login)
    } else {
        create_register_window(app_handle)?;
        Ok(WindowType::Register)
    }
}

/// Checks if the database exists, connects to it, opens main window and closes login window.
/// # Error
/// - Connection to the database fails
/// - The new window cannot be created
/// - The current window cannot be closed
/// # Restart
/// Restarts the application if the database does not exist. Error is shown in a blocking dialog.
#[tauri::command]
pub async fn login<'a>(
    password: SecretString,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    if DatabaseConnection::database_exists(app_handle.clone()).not() {
        critical_error("Database does not exist", app_handle, window);
        return Err("Database does not exist");
    }

    connection.connect(password, app_handle.clone())?;

    #[cfg(target_os = "macos")]
    app_handle
        .save_window_state(StateFlags::all())
        .unwrap_or_default();

    initialize_window(connection, app_handle)
        .await
        .map_err(|_| "Failed to initialize window")?;

    window
        .close()
        .map_err(|_| "Failed to close current window")?;

    Ok(())
}

/// Checks if database does not exist, compares passwords, connects to database, creates main window and closes register window.
/// # Error
/// - Passwords do not match
/// - Connection to the database fails
/// - The new window cannot be created
/// - The current window cannot be closed
/// # Restart
/// Restarts the application if the database already exists. Error is shown in a blocking dialog.
#[tauri::command(rename_all = "snake_case")]
pub async fn register<'a>(
    password: SecretString,
    confirm_password: SecretString,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    if DatabaseConnection::database_exists(app_handle.clone()) {
        critical_error("Database already exists", app_handle, window);
        return Err("Database already exists");
    }

    if password.expose_secret() != confirm_password.expose_secret() {
        return Err("Passwords do not match.");
    }

    connection.connect(password, app_handle.clone())?;

    #[cfg(target_os = "macos")]
    app_handle
        .save_window_state(StateFlags::all())
        .unwrap_or_default();

    initialize_window(connection, app_handle)
        .await
        .map_err(|_| "Failed to initialize window")?;

    window
        .close()
        .map_err(|_| "Failed to close current window")?;

    Ok(())
}

/// Returns all records from the database.
/// # Restart
/// Restarts the application if any error occurs. Errors are shown in blocking dialogs.
#[tauri::command]
pub async fn get_all_records<'a>(
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<Vec<Record>, ()> {
    let Ok(guard) = connection.database.lock() else {
        return Err(critical_error("Database lock poisoned", app_handle, window));
    };

    let Some(database) = guard.as_ref() else {
        return Err(critical_error(
            "Database does not exist",
            app_handle,
            window,
        ));
    };

    database
        .get_all_records()
        .map_err(|_| critical_error("Failed to load records", app_handle, window))
}

/// Returns ids of records that have compromised passwords. A password is considered compromised if it is a common password or if it is exposed in a data breach.
/// # Restart
/// Restarts the application if any error occurs. Errors are shown in blocking dialogs.

#[tauri::command]
pub async fn get_compromised_records<'a>(
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<Vec<u64>, ()> {
    let records = get_all_records(connection.clone(), app_handle.clone(), window.clone()).await?;
    let mut result: Vec<u64> = Vec::with_capacity(records.len());

    for record in records {
        let all_content = {
            let Ok(guard) = connection.database.lock() else {
                return Err(critical_error("Database lock poisoned", app_handle, window));
            };

            let Some(database) = guard.as_ref() else {
                return Err(critical_error("Database is not opened", app_handle, window));
            };

            let Ok(content) = database.get_all_content_for_record(record.id()) else {
                return Err(critical_error("Failed to load content", app_handle, window));
            };

            content
        };

        for content in all_content {
            if let Value::Password(password) = content.value() {
                if passwords::analyzer::is_common_password(password.value())
                    || utils::password::is_exposed(password.value())
                        .await
                        .unwrap_or(false)
                {
                    result.push(record.id());
                    break;
                }
            }
        }
    }

    Ok(result)
}

/// Returns all content for a specific record. If Record is new, it returns default content for the category. If content is TOTP secret, it is added to the TOTP manager.
/// # Restart
/// Restarts the application if any error occurs. Errors are shown in blocking dialogs.
#[tauri::command]
pub async fn get_all_content_for_record<'a>(
    record: Record,
    connection: State<'a, DatabaseConnection>,
    totp_manager: State<'a, TOTPManager>,
    app_handle: AppHandle,
    window: Window,
) -> Result<Vec<Content>, ()> {
    if record.id() == 0 {
        let mut content: Vec<Content> = Vec::with_capacity(4);
        match record.category() {
            Category::Login => {
                content.push(Content::new(
                    "Website".to_string(),
                    1,
                    true,
                    Value::Url(value::Url::default()),
                ));
                content.push(Content::new(
                    "User".to_string(),
                    2,
                    true,
                    Value::Text(value::Text::default()),
                ));
                content.push(Content::new(
                    "Password".to_string(),
                    3,
                    true,
                    Value::Password(value::Password::default()),
                ));
            }
            Category::BankCard => {
                content.push(Content::new(
                    "Card number".to_string(),
                    1,
                    true,
                    Value::BankCardNumber(value::BankCardNumber::default()),
                ));
                content.push(Content::new(
                    "CVV".to_string(),
                    2,
                    true,
                    Value::Number(value::Number::default()),
                ));
                content.push(Content::new(
                    "Expiration date".to_string(),
                    3,
                    true,
                    Value::Date(value::Date::default()),
                ));
                content.push(Content::new(
                    "PIN".to_string(),
                    4,
                    true,
                    Value::Number(value::Number::default()),
                ));
            }
            Category::Note => {
                content.push(Content::new(
                    "Note".to_string(),
                    1,
                    true,
                    Value::LongText(value::LongText::default()),
                ));
            }
            Category::Other => {}
        }
        Ok(content)
    } else {
        let Ok(guard) = connection.database.lock() else {
            return Err(critical_error("Database lock poisoned", app_handle, window));
        };

        let Some(database) = guard.as_ref() else {
            return Err(critical_error("Database is not opened", app_handle, window));
        };

        let Ok(content) = database.get_all_content_for_record(record.id()) else {
            return Err(critical_error("Failed to load content", app_handle, window));
        };

        content.iter().for_each(|content| {
            if let Value::TOTPSecret(totp_secret) = content.value() {
                totp_manager
                    .add_secret(content.id(), totp_secret.value().to_string())
                    .unwrap_or_default();
            }
        });

        Ok(content)
    }
}

/// Used for returning content value that are normally not serialized.
pub struct ContentValue(SecretString);

impl Serialize for ContentValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.expose_secret())
    }
}

/// Returns a specific content from the database. Does **not** manage TOTP secrets as [`get_all_content_for_record`] does.
/// # Error
/// Returns an error if the content cannot be loaded.
/// # Restart
/// Restarts the application if the database cannot be accessed.  Error is shown in a blocking dialog.
#[tauri::command]
pub async fn get_content_value<'a>(
    id: u64,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<ContentValue, &'static str> {
    let Ok(guard) = connection.database.lock() else {
        critical_error("Database lock poisoned", app_handle, window);
        return Err("Database lock poisoned");
    };

    let Some(database) = guard.as_ref() else {
        critical_error("Database is not opened", app_handle, window);
        return Err("Database is not opened");
    };

    database
        .get_content(id)
        .map(|content| ContentValue(content.value().to_secret_string()))
        .map_err(|_| "Failed to get content value")
}

/// Saves a record to the database.
/// # Return
/// Returns the saved record. If the record is new, it will have an id.
/// # Error
/// Returns an error if the record cannot be saved.
/// # Restart
/// Restarts the application if the database cannot be accessed. Error is shown in a blocking dialog.
#[tauri::command]
pub async fn save_record<'a>(
    mut record: Record,
    content: Vec<Content>,
    connection: State<'a, DatabaseConnection>,
    totp_manager: State<'a, TOTPManager>,
    app_handle: AppHandle,
    window: Window,
) -> Result<Record, &'static str> {
    let Ok(guard) = connection.database.lock() else {
        critical_error("Database lock poisoned", app_handle, window);
        return Err("Database lock poisoned");
    };

    let Some(database) = guard.as_ref() else {
        critical_error("Database is not opened", app_handle, window);
        return Err("Database is not opened");
    };

    database
        .save_record(&mut record)
        .map_err(|_| "Failed to save record")?;

    for mut content in content {
        if let Value::TOTPSecret(_) = content.value() {
            totp_manager.remove(&content.id());
        }

        database
            .save_content(record.id(), &mut content)
            .map_err(|_| "Failed to save content")?;
    }

    Ok(record)
}

/// Deletes a record from the database.
/// # Error
/// Returns an error if the record cannot be deleted.
/// # Restart
/// Restarts the application if the database cannot be accessed.
#[tauri::command]
pub async fn delete_record<'a>(
    record: Record,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    let Ok(mut guard) = connection.database.lock() else {
        critical_error("Database lock poisoned", app_handle, window);
        return Err("Database lock poisoned");
    };

    let Some(database) = guard.as_mut() else {
        critical_error("Database is not opened", app_handle, window);
        return Err("Database is not opened");
    };

    database
        .delete_record(record)
        .map_err(|_| "Failed to delete record")
}

/// Deletes a content from the database.
/// # Error
/// Returns an error if the content cannot be deleted.
/// # Restart
/// Restarts the application if the database cannot be accessed.
#[tauri::command]
pub async fn delete_content<'a>(
    content: Content,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    let Ok(mut guard) = connection.database.lock() else {
        critical_error("Database lock poisoned", app_handle, window);
        return Err("Database lock poisoned");
    };

    let Some(database) = guard.as_mut() else {
        critical_error("Database is not opened", app_handle, window);
        return Err("Database is not opened");
    };

    database
        .delete_content(content)
        .map_err(|_| "Failed to delete content")
}

/// Takes value from database and copies it to the clipboard.
/// # Error
/// If value cannot be copied to the clipboard
/// # Restart
/// Restarts the application if the database cannot be accessed. Error is shown in a blocking dialog.
#[tauri::command]
pub async fn copy_value_to_clipboard<'a>(
    id: u64,
    connection: State<'a, DatabaseConnection>,
    totp_manager: State<'a, TOTPManager>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    let Ok(guard) = connection.database.lock() else {
        critical_error("Database lock poisoned", app_handle, window);
        return Err("Database lock poisoned");
    };

    let Some(database) = guard.as_ref() else {
        critical_error("Database is not opened", app_handle, window);
        return Err("Database is not opened");
    };

    let content = database
        .get_content(id)
        .map_err(|_| "Failed to load content")?;

    let value = if let Value::TOTPSecret(_) = content.value() {
        let (code, _) = totp_manager
            .get_code(&id)
            .ok_or("Failed to get TOTP code")?;
        SecretString::new(code)
    } else {
        content.value().to_secret_string()
    };

    arboard::Clipboard::new()
        .map_err(|_| "Clipboard is not available")?
        .set_text(value.expose_secret())
        .map_err(|_| "Failed to copy value to clipboard")
}

/// Returns a TOTP code for a specific record.
/// # Error
/// Returns error when TOTP is not loaded into the TOTP manager or TOTP code cannot be generated
#[tauri::command]
pub async fn get_totp_code<'a>(
    id: u64,
    totp_manager: State<'a, TOTPManager>,
) -> Result<(String, u64), &'static str> {
    totp_manager.get_code(&id).ok_or("Failed to get TOTP code")
}

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
/// Returns a tuple with a boolean indicating if the value is valid and an optional error message.
#[tauri::command]
pub async fn valid(kind: SecretString, value: SecretString) -> (bool, Option<String>) {
    match kind.expose_secret().as_str() {
        "Number" => {
            if value
                .expose_secret()
                .parse::<i64>()
                .map(|mut _value| _value.zeroize())
                .is_ok()
            {
                (true, None)
            } else {
                (false, Some("Invalid number".to_string()))
            }
        }
        "LongText" => (true, None),
        "Date" => {
            if value
                .expose_secret()
                .parse::<chrono::NaiveDate>()
                .map(|mut _value| _value = chrono::NaiveDate::default())
                .is_ok()
            {
                (true, None)
            } else {
                (false, Some("Invalid date".to_string()))
            }
        }
        "TOTPSecret" => {
            if let Err(error) = value::TOTPSecret::new(value.expose_secret().to_string()) {
                (false, Some(error.to_string()))
            } else {
                (true, None)
            }
        }
        "Url" => {
            if validator::validate_url(value.expose_secret())
                || validator::validate_ip_v4(value.expose_secret())
                || validator::validate_ip_v6(value.expose_secret())
            {
                (true, None)
            } else {
                (false, Some("Invalid URL".to_string()))
            }
        }
        "Email" => {
            if validator::validate_email(value.expose_secret()) {
                (true, None)
            } else {
                (false, Some("Invalid email".to_string()))
            }
        }
        "PhoneNumber" => {
            if validator::validate_phone(value.expose_secret()) {
                (true, None)
            } else {
                (false, Some("Invalid phone number".to_string()))
            }
        }
        "BankCardNumber" => match card_validate::Validate::from(value.expose_secret()) {
            Ok(_) => (true, None),
            Err(error) => (
                false,
                Some(
                    match error {
                        card_validate::ValidateError::InvalidFormat => "Invalid Format",
                        card_validate::ValidateError::InvalidLength => "Invalid Length",
                        card_validate::ValidateError::InvalidLuhn => "Invalid Luhn",
                        card_validate::ValidateError::UnknownType => "Unknown Type",
                        _ => "Unknown Error",
                    }
                    .to_string(),
                ),
            ),
        },
        _ => {
            if value.expose_secret().trim().is_empty() {
                (false, Some("Value cannot be empty".to_string()))
            } else {
                (true, None)
            }
        }
    }
}

/// Indicating problem with the password.
#[derive(Clone, serde::Serialize)]
pub enum PasswordProblem {
    Common,
    Exposed,
    None,
}

/// Checks if the password is common or exposed. Value is taken from the database.
/// # Error
/// Returns an error if content cannot be loaded, if the content is not a password or if the password cannot be checked.
/// # Restart
/// Restarts the application if the database cannot be accessed. Error is shown in a blocking dialog.
#[tauri::command]
pub async fn check_password<'a>(
    id: u64,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<PasswordProblem, &'static str> {
    let password = {
        let Ok(guard) = connection.database.lock() else {
            critical_error("Database lock poisoned", app_handle, window);
            return Err("Database lock poisoned");
        };

        let Some(database) = guard.as_ref() else {
            critical_error("Database is not opened", app_handle, window);
            return Err("Database is not opened");
        };

        let content = database
            .get_content(id)
            .map_err(|_| "Failed to load content")?;

        let Value::Password(password) = content.value() else {
            return Err("Content is not a password");
        };

        password.to_secret_string()
    };

    if passwords::analyzer::is_common_password(password.expose_secret()) {
        Ok(PasswordProblem::Common)
    } else if utils::password::is_exposed(password.expose_secret())
        .await
        .map_err(|_| "Failed to check password")?
    {
        Ok(PasswordProblem::Exposed)
    } else {
        Ok(PasswordProblem::None)
    }
}

/// Returns the strength of the password ([`passwords::scorer::score`])
#[tauri::command]
pub async fn password_strength(password: SecretString) -> f64 {
    passwords::scorer::score(&passwords::analyzer::analyze(password.expose_secret()))
}

/// Returns the type of the bank card number ([`card_validate::Validate::evaluate_type`]). Value is taken from the database.
/// # Error
/// Returns an error if content cannot be loaded, if the content is not a bank card number or if the card type cannot be evaluated.
/// # Restart
/// Restarts the application if the database cannot be accessed. Error is shown in a blocking dialog.
#[tauri::command]
pub async fn card_type<'a>(
    id: u64,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<String, &'static str> {
    let card_number = {
        let Ok(guard) = connection.database.lock() else {
            critical_error("Database lock poisoned", app_handle, window);
            return Err("Database lock poisoned");
        };

        let Some(database) = guard.as_ref() else {
            critical_error("Database is not opened", app_handle, window);
            return Err("Database is not opened");
        };

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

//TODO Cloud sync
#[tauri::command]
pub async fn save_to_cloud() -> Result<String, &'static str> {
    // Simulate uploading data
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok(format!(
        "Last saved at {}",
        chrono::Local::now().format("%d.%m.%Y %H:%M")
    ))
}
