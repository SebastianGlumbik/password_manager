use super::*;
use crate::cloud;
use crate::database::model::SecretValue;
use std::os::unix::fs::MetadataExt;

/// For sending cloud data to the frontend
#[derive(Clone, serde::Serialize)]
pub struct CloudData {
    address: SecretValue,
    username: SecretValue,
}

/// Returns cloud data if cloud is enabled.
#[tauri::command]
pub async fn cloud_data<'a>(database: State<'a, Database>) -> Result<CloudData, &'static str> {
    if cloud::CloudManager::is_enabled(&database) {
        let address = database
            .get_setting("cloud_address")
            .map_err(|_| "Failed to load address")?;
        let username = database
            .get_setting("cloud_username")
            .map_err(|_| "Failed to load username")?;
        Ok(CloudData { address, username })
    } else {
        Err("Cloud is not enabled")
    }
}

/// Enables cloud storage and saves the credentials.
#[tauri::command]
pub async fn enable_cloud<'a>(
    address: SecretString,
    username: SecretString,
    password: SecretString,
    app_handle: AppHandle,
    window: Window,
    database: State<'a, Database>,
) -> Result<(), &'static str> {
    let manager = cloud::CloudManager::enable(
        address.expose_secret(),
        username.expose_secret(),
        password.expose_secret(),
        &app_handle,
        &database,
    )?;

    if manager.exists()? && tauri::api::dialog::blocking::MessageDialogBuilder::new("Database detected", "Database detected on cloud, which version do you want to use? (the other one will be overwritten)")
        .buttons(tauri::api::dialog::MessageDialogButtons::OkCancelWithLabels("Cloud (restart app)".to_string(), "Local".to_string())).kind(tauri::api::dialog::MessageDialogKind::Warning).parent(&window).show() {
        app_handle.restart();
    }

    tauri::api::dialog::message(
        Some(&window),
        "Success",
        "From now on your database will be uploaded to the cloud",
    );

    window
        .emit("upload", ())
        .map_err(|_| "Failed to start upload")?;

    Ok(())
}

/// Disables cloud storage and deletes the credentials.
#[tauri::command]
pub async fn disable_cloud<'a>(database: State<'a, Database>) -> Result<(), &'static str> {
    cloud::CloudManager::disable(&database)
}

/// Uploads the database to the cloud.
#[tauri::command]
pub async fn cloud_upload<'a>(
    window: Window,
    app_handle: AppHandle,
    database: State<'a, Database>,
) -> Result<String, &'static str> {
    if cloud::CloudManager::is_enabled(&database) {
        let manager = cloud::CloudManager::connect_from_database(&database, &app_handle)?;
        if manager.exists()? {
            let cloud_mtime =
                chrono::DateTime::from_timestamp(manager.m_time().unwrap_or_default(), 0)
                    .ok_or("Failed to get cloud mtime")?;

            let local_database_path =
                Database::path(&app_handle).ok_or("Failed to get database path")?;
            let local_mtime = chrono::DateTime::from_timestamp(
                std::fs::metadata(local_database_path)
                    .map_err(|_| "Failed to get local metadata")?
                    .mtime(),
                0,
            )
            .ok_or("Failed to get local mtime")?;

            if local_mtime < cloud_mtime && !tauri::api::dialog::blocking::MessageDialogBuilder::new("Cloud version is newer", format!("The cloud version is newer ({}) than the local one ({}). Which version do you want to use?", cloud_mtime.format("%Y-%m-%d %H:%M:%S"), local_mtime.format("%Y-%m-%d %H:%M:%S")))
                .buttons(tauri::api::dialog::MessageDialogButtons::OkCancelWithLabels("Local".to_string(), "Cloud".to_string())).kind(tauri::api::dialog::MessageDialogKind::Warning).parent(&window).show()
            {
                return Err("Canceled by user");
            }
        }

        manager.upload().await?;
        Ok(format!(
            "Last sync: {}",
            chrono::Local::now().time().format("%H:%M:%S")
        ))
    } else {
        Err("Cloud is not enabled")
    }
}
