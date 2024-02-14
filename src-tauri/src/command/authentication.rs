use super::*;
use crate::cloud;

/// Register process. Database must not exist. Adds the database to the app state, initializes the main window and closes the current window.
/// # Restart
/// Restarts the application if the database already exists. Error is shown in a blocking dialog.
#[tauri::command(rename_all = "snake_case")]
pub async fn register<'a>(
    password: SecretString,
    confirm_password: SecretString,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    if Database::exists(&app_handle) {
        critical_error("Database already exists", &app_handle, &window);
        return Err("Database already exists");
    }

    if password.expose_secret() != confirm_password.expose_secret() {
        return Err("Passwords do not match.");
    }

    app_handle.manage(Database::open(password.expose_secret(), &app_handle)?);

    #[cfg(target_os = "macos")]
    app_handle
        .save_window_state(StateFlags::all())
        .unwrap_or_default();

    initialize_window(app_handle)
        .await
        .map_err(|_| "Failed to initialize window")?;

    window
        .close()
        .map_err(|_| "Failed to close current window")?;

    Ok(())
}

/// Login process. Database must exist. If cloud storage is enabled, it tries to download the database from the cloud. Adds the database to the app state, initializes the main window and closes the current window.
/// # Restart
/// Restarts the application if the database does not exist. Error is shown in a blocking dialog.
#[tauri::command]
pub async fn login<'a>(
    password: SecretString,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    if Database::exists(&app_handle).not() {
        critical_error("Database does not exist", &app_handle, &window);
        return Err("Database does not exist");
    }

    let mut database = Database::open(password.expose_secret(), &app_handle)?;

    if cloud::is_enabled(&database) {
        if let Err(error) = cloud::download(&window, &app_handle, &database).await {
            if tauri::api::dialog::blocking::ask(
                Some(&window),
                error,
                "Do you wish to continue without cloud storage?",
            )
            .not()
            {
                return Err(error);
            }
        }
        database = Database::open(password.expose_secret(), &app_handle)?;
    }

    database.delete_data_breach_cache_older_24h()?;

    app_handle.manage(database);

    #[cfg(target_os = "macos")]
    app_handle
        .save_window_state(StateFlags::all())
        .unwrap_or_default();

    initialize_window(app_handle)
        .await
        .map_err(|_| "Failed to initialize window")?;

    window
        .close()
        .map_err(|_| "Failed to close current window")?;

    Ok(())
}

/// Changes the master password.
#[tauri::command(rename_all = "snake_case")]
pub async fn change_password<'a>(
    password: SecretString,
    confirm_password: SecretString,
    database: State<'a, Database>,
) -> Result<(), &'static str> {
    if password.expose_secret() != confirm_password.expose_secret() {
        return Err("Passwords do not match.");
    }

    database.change_key(password.expose_secret())?;

    Ok(())
}
