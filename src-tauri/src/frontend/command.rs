use super::*;
use crate::database::model::{value, Category, Content, Record, Value};

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
        tauri::api::dialog::blocking::message(
            Some(&window),
            "Error",
            "Database does not exist, application will now restart",
        );
        app_handle.restart();
    }

    connection.connect(password, app_handle.clone())?;

    #[cfg(target_os = "macos")]
    app_handle
        .save_window_state(StateFlags::all())
        .unwrap_or_default();

    initialize_window(connection, app_handle)
        .await
        .map_err(|_| "Failed to create main window")?;

    window.close().map_err(|_| "Failed to close window")?;

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
        tauri::api::dialog::blocking::message(
            Some(&window),
            "Error",
            "Database already exists, application will now restart",
        );
        app_handle.restart();
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
        .map_err(|_| "Failed to create main window")?;

    window.close().map_err(|_| "Failed to close window")?;

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
    // Mock data for testing TODO: Remove
    let mut login = Record::new("Login".to_string(), "Subtitle".to_string(), Category::Login);
    login.set_id(1);

    let mut card = Record::new(
        "Card".to_string(),
        "Subtitle".to_string(),
        Category::BankCard,
    );
    card.set_id(2);

    let mut note = Record::new("Note".to_string(), "Subtitle".to_string(), Category::Note);
    note.set_id(3);

    let mut custom = Record::new(
        "Custom".to_string(),
        "Subtitle".to_string(),
        Category::Custom("Custom".to_string()),
    );
    custom.set_id(4);

    return Ok(vec![login, card, note, custom]);
    /*
    if let Ok(guard) = connection.database_mutex.lock() {
        if let Some(database) = guard.as_ref() {
            if let Ok(records) = database.get_all_records() {
                return Ok(records);
            }
        }
    }

/// Returns ids of records that have compromised passwords. A password is considered compromised if it is a common password or if it is exposed in a data breach.
/// # Restart
/// Restarts the application if any error occurs. Errors are shown in blocking dialogs.

    app_handle.restart();
    Err(())*/
}

/// Returns all records from the database that contain compromised passwords. Compromised passwords are common passwords or passwords that have been exposed in a data breach.
/// ## Errors
/// When error occurs, a blocking dialog message will be shown and the application will restart.
#[tauri::command]
pub async fn get_compromised_records<'a>(
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<Vec<Record>, ()> {
    let records = get_all_records(connection.clone(), app_handle.clone(), window.clone()).await?;

    let mut result = vec![];

    for record in records {
        let mut all_content: Vec<Content> = vec![];

        if let Ok(guard) = connection.database_mutex.lock() {
            if let Some(database) = guard.as_ref() {
                if let Ok(content) = database.get_all_content_for_record(record.id()) {
                    all_content = content;
                }
            }
        }

        for content in all_content {
            if let Value::Password(password) = content.value() {
                if utils::password::is_exposed(password.value())
                    .await
                    .unwrap_or(false)
                    || utils::password::is_common_password(password.value())
                {
                    result.push(record);
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
    totp_manager
        .add_secret(0, "xprbaclyjlxlixkxxp5hlx6bkrpm4qmi".to_string())
        .map_err(|_| ())?;
    if record.id() == 0 {
        let mut content = vec![];
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
                    Value::Datetime(value::Datetime::default()),
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
                    Value::Text(value::Text::default()),
                ));
            }
            Category::Custom(_) => {}
        }
        return Ok(content);
    } else if let Ok(guard) = connection.database_mutex.lock() {
        if let Some(database) = guard.as_ref() {
            if let Ok(content) = database.get_all_content_for_record(record.id()) {
                return Ok(content);
            }
        }
    }

    tauri::api::dialog::blocking::message(
        Some(&window),
        "Error",
        "Failed to load record, application will restart",
    );

    Err(app_handle.restart())
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
    window: Window,
) -> Result<(), ()> {
    if tauri::api::dialog::blocking::ask(
        Some(&window),
        "Delete record",
        format!("Are you sure you want to delete {}?", record.title()),
    ) {
        if let Ok(mut guard) = connection.database_mutex.lock() {
            if let Some(database) = guard.as_mut() {
                if database.delete_record(record).is_err() {
                    tauri::api::dialog::blocking::message(
                        Some(&window),
                        "Error",
                        "Failed to delete record",
                    );
                }
            }
        }
    }

    Ok(())
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

#[tauri::command]
pub async fn save_test<'a>(
    record: Record,
    content: Vec<Content>,
    totp_manager: State<'a, TOTPManager>,
) -> Result<(), &'static str> {
    println!("Record: {:?}", record);
    println!("Content: {:?}", content);
    println!("TOTP: {:?}", totp_manager.get_code(&0));
    Ok(())
}
