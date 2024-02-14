use super::password::{check_password, PasswordProblem};
use super::*;
use crate::database::model::SecretValue;

/// Returns all records from the database.
/// # Restart
/// Restarts the application if any error occurs. Errors are shown in blocking dialogs.
#[tauri::command]
pub async fn get_all_records<'a>(
    database: State<'a, Database>,
    app_handle: AppHandle,
    window: Window,
) -> Result<Vec<Record>, ()> {
    database
        .get_all_records()
        .map_err(|_| critical_error("Failed to load records", &app_handle, &window))
}

/// Returns ids of records that have compromised passwords. A password is considered compromised if it is a common password or if it is exposed in a data breach.
/// # Restart
/// Restarts the application if any error occurs. Errors are shown in blocking dialogs.
#[tauri::command]
pub async fn get_compromised_records<'a>(
    database: State<'a, Database>,
    app_handle: AppHandle,
    window: Window,
) -> Result<Vec<u64>, ()> {
    let records = get_all_records(database.clone(), app_handle.clone(), window.clone()).await?;
    let mut result: Vec<u64> = Vec::with_capacity(records.len());

    for record in records {
        let all_content = database
            .get_all_content_for_record(record.id())
            .map_err(|_| critical_error("Failed to load content", &app_handle, &window))?;

        for content in all_content {
            if let Value::Password(password) = content.value() {
                match check_password(password.to_secret_string(), database.clone()).await {
                    Ok(PasswordProblem::Common) | Ok(PasswordProblem::Exposed) => {
                        result.push(record.id());
                        break;
                    }
                    _ => continue,
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
    database: State<'a, Database>,
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
        let content = database
            .get_all_content_for_record(record.id())
            .map_err(|_| critical_error("Failed to load content", &app_handle, &window))?;

        totp_manager.reset();
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

/// Returns a specific content from the database. Does **not** manage TOTP secrets as [`get_all_content_for_record`] does.
/// # Error
/// Returns an error if the content cannot be loaded.
#[tauri::command]
pub async fn get_content_value<'a>(
    id: u64,
    database: State<'a, Database>,
) -> Result<SecretValue, &'static str> {
    database
        .get_content(id)
        .map(|content| SecretValue::new(content.value().to_secret_string()))
        .map_err(|_| "Failed to get content value")
}

/// Saves a record to the database.
/// # Return
/// Returns record id.
/// # Error
/// Returns an error if the record cannot be saved.
#[tauri::command]
pub async fn save_record<'a>(
    mut record: Record,
    content: Vec<Content>,
    database: State<'a, Database>,
) -> Result<u64, &'static str> {
    database
        .save_record(&mut record)
        .map_err(|_| "Failed to save record")?;

    for mut content in content {
        database
            .save_content(record.id(), &mut content)
            .map_err(|_| "Failed to save content")?;
    }

    Ok(record.id())
}

/// Deletes a record from the database.
/// # Error
/// Returns an error if the record cannot be deleted.
#[tauri::command]
pub async fn delete_record<'a>(
    record: Record,
    database: State<'a, Database>,
) -> Result<(), &'static str> {
    database
        .delete_record(record)
        .map_err(|_| "Failed to delete record")
}

/// Deletes a content from the database.
/// # Error
/// Returns an error if the content cannot be deleted.
#[tauri::command]
pub async fn delete_content<'a>(
    content: Content,
    database: State<'a, Database>,
) -> Result<(), &'static str> {
    database
        .delete_content(content)
        .map_err(|_| "Failed to delete content")
}
