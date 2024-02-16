mod cloud;
mod command;
mod database;
mod totp;
mod window;

use command::authentication::*;
use command::cloud::*;
use command::database::*;
use command::password::*;
use command::totp::*;
use command::validation::*;
use command::window::*;
use command::*;
use tauri::{AppHandle, Manager, Window};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use totp::TOTPManager;

/// Shows a critical error message and restarts the application.
///
/// NOTE: blocking, can not be used on the main thread.
fn critical_error(message: &str, app_handle: &AppHandle, window: &Window) {
    tauri::api::dialog::blocking::MessageDialogBuilder::new(
        "Critical Error",
        format!("{}\nApplication will now restart", message),
    )
    .kind(tauri::api::dialog::MessageDialogKind::Error)
    .buttons(tauri::api::dialog::MessageDialogButtons::Ok)
    .parent(window)
    .show();
    app_handle.restart();
}

/// Payload for single instance plugin.
#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

/// Runs the tauri application.
/// Used plugins:
/// - https://crates.io/crates/tauri-plugin-context-menu
/// - https://github.com/tauri-apps/plugins-workspace/tree/v1/plugins/single-instance
/// - https://github.com/tauri-apps/plugins-workspace/tree/v1/plugins/window-state
///
/// Note: The window-state plugin is only used on macOS due to bug on Linux contained in the plugin.
pub fn run() -> anyhow::Result<()> {
    let app_builder = tauri::Builder::default()
        .plugin(tauri_plugin_context_menu::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap_or_default();
        }))
        .manage(TOTPManager::new(50))
        .invoke_handler(tauri::generate_handler![
            initialize_window,
            login,
            register,
            change_password,
            get_all_records,
            get_compromised_records,
            get_all_content_for_record,
            get_content_value,
            save_record,
            delete_record,
            delete_content,
            get_totp_code,
            copy_value_to_clipboard,
            check_password,
            check_password_from_database,
            password_strength,
            generate_password,
            validate,
            card_type,
            cloud_data,
            enable_cloud,
            disable_cloud,
            cloud_upload,
        ]);

    #[cfg(target_os = "macos")]
    let app_builder = app_builder.plugin(tauri_plugin_window_state::Builder::default().build());

    let app = app_builder.build(tauri::generate_context!())?;

    initialize_window(app.app_handle())?;

    app.run(|_app_handle, _event| { /* Can react to events */ });

    Ok(())
}
