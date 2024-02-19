use super::*;
use crate::cloud;
use std::os::unix::fs::MetadataExt;

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

    initialize_window(app_handle).map_err(|_| "Failed to initialize window")?;

    window
        .close()
        .map_err(|_| "Failed to close current window")?;

    Ok(())
}

/// Helper function for login process. Checks databases versions and downloads the cloud database if it is newer. Shows a dialog if the local version is newer.
async fn login_download(
    app_handle: &AppHandle,
    window: &Window,
    database: &Database,
) -> Result<(), &'static str> {
    let manager = cloud::CloudManager::connect_from_database(database, app_handle)?;
    if manager.exists()? {
        let cloud_mtime = chrono::DateTime::from_timestamp(manager.m_time()?, 0)
            .ok_or("Failed to get cloud mtime")?;

        let local_database_path =
            Database::path(app_handle).ok_or("Failed to get database path")?;
        let local_mtime = chrono::DateTime::from_timestamp(
            std::fs::metadata(local_database_path)
                .map_err(|_| "Failed to get local metadata")?
                .mtime(),
            0,
        )
        .ok_or("Failed to get local mtime")?;

        if local_mtime <= cloud_mtime || tauri::api::dialog::blocking::MessageDialogBuilder::new("Local version is newer", format!("The local version is newer ({}) than the cloud one ({}). Which version do you want to use?", local_mtime.format("%Y-%m-%d %H:%M:%S"), cloud_mtime.format("%Y-%m-%d %H:%M:%S")))
            .buttons(tauri::api::dialog::MessageDialogButtons::OkCancelWithLabels("Cloud".to_string(), "Local".to_string())).kind(tauri::api::dialog::MessageDialogKind::Warning).parent(window).show()
        {
            manager.download().await?;
        }
    }

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

    if cloud::CloudManager::is_enabled(&database) {
        if let Err(error) = login_download(&app_handle, &window, &database).await {
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

    initialize_window(app_handle).map_err(|_| "Failed to initialize window")?;

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
