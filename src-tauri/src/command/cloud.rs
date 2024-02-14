use super::*;
use crate::cloud;
use crate::database::model::SecretValue;

/// For sending cloud data to the frontend
#[derive(Clone, serde::Serialize)]
pub struct CloudData {
    address: SecretValue,
    username: SecretValue,
}

/// Returns cloud data if cloud is enabled.
#[tauri::command]
pub async fn cloud_data<'a>(database: State<'a, Database>) -> Result<CloudData, &'static str> {
    if cloud::is_enabled(&database) {
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
    cloud::enable(
        address.expose_secret(),
        username.expose_secret(),
        password.expose_secret(),
        &app_handle,
        &window,
        &database,
    )
    .await
}

/// Disables cloud storage and deletes the credentials.
#[tauri::command]
pub async fn disable_cloud<'a>(database: State<'a, Database>) -> Result<(), &'static str> {
    cloud::disable(&database).await
}

/// Uploads the database to the cloud.
#[tauri::command]
pub async fn cloud_upload<'a>(
    window: Window,
    app_handle: AppHandle,
    database: State<'a, Database>,
) -> Result<String, &'static str> {
    if cloud::is_enabled(&database) {
        cloud::upload(&window, &app_handle, &database).await
    } else {
        Ok("Cloud is not enabled".to_string())
    }
}

/// Downloads the database from the cloud.
#[tauri::command]
pub async fn cloud_download<'a>(
    window: Window,
    app_handle: AppHandle,
    database: State<'a, Database>,
) -> Result<String, &'static str> {
    if cloud::is_enabled(&database) {
        cloud::download(&window, &app_handle, &database).await
    } else {
        Ok("Cloud is not enabled".to_string())
    }
}
