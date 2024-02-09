mod database;
mod frontend;
mod utils;

use frontend::*;
use tauri::async_runtime::block_on;
use tauri::Manager;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

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
        .manage(TOTPManager::default())
        .invoke_handler(tauri::generate_handler![
            initialize_window,
            login,
            register,
            change_password,
            get_all_records,
            get_compromised_records,
            save_record,
            delete_record,
            get_all_content_for_record,
            get_content_value,
            delete_content,
            get_totp_code,
            valid,
            password_strength,
            check_password,
            copy_value_to_clipboard,
            card_type,
            generate_password,
            cloud
        ]);

    #[cfg(target_os = "macos")]
    let app_builder = app_builder.plugin(tauri_plugin_window_state::Builder::default().build());

    let app = app_builder.build(tauri::generate_context!())?;

    block_on(initialize_window(app.app_handle()))?;

    app.run(|_app_handle, _event| {
        // Can react to events
    });

    Ok(())
}
