use super::*;
use crate::database::model::{Category, Content, Record, Value};

/// Window types that can be created.
#[derive(Clone, serde::Serialize)]
pub enum WindowType {
    Login,
    Register,
    Main,
}

/// Creates specific window based on the database state and returns the window type.
/// ## Errors
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
/// ## Dialogs
/// If the database does not exist, a blocking dialog message will be shown and the application will restart.
/// ## Errors
/// Returns an error if:
/// - Connection to the database fails
/// - The main window cannot be created
/// - The login window cannot be closed
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
/// ## Dialogs
/// If the database does exist, a dialog will be shown and the application will restart.
/// ## Errors
/// Returns an error if:
/// - Passwords do not match
/// - Connection to the database fails
/// - The main window cannot be created
/// - The register window cannot be closed
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
/// ## Errors
/// When error occurs, a blocking dialog message will be shown and the application will restart.
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

    tauri::api::dialog::blocking::message(
        Some(&window),
        "Error",
        "Failed to load records, application will restart",
    );

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
        let all_content = get_all_content_for_record(
            record.id(),
            connection.clone(),
            app_handle.clone(),
            window.clone(),
        )
        .await?;

        for content in &all_content {
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

/// Returns all content for a specific record.
/// Errors
/// When error occurs, a blocking dialog message will be shown and the application will restart.
#[tauri::command]
pub async fn get_all_content_for_record<'a>(
    id: u64,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<Vec<Content>, ()> {
    if let Ok(guard) = connection.database_mutex.lock() {
        if let Some(database) = guard.as_ref() {
            if let Ok(content) = database.get_all_content_for_record(id) {
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
/// ## Dialogs
/// - Has a confirmation dialog before deleting the record
/// - Has a message dialog if the record could not be deleted
/// ## Errors
///
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
pub async fn save_test(record: Record, content: Content) -> Result<(), &'static str> {
    println!("Record: {:?}", record);
    println!("Content: {:?}", content);
    Ok(())
}
