mod database;
mod frontend;
mod utils;

use database::Database;
use frontend::*;
use tauri::async_runtime::block_on;
use tauri::Manager;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

//TODO Comments for whole project
//TODO csv export/import, cloud backup

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
        .manage(DatabaseConnection::default())
        .invoke_handler(tauri::generate_handler![
            initialize_window,
            login,
            register,
            get_all_records,
            get_compromised_records,
            get_all_content_for_record,
            delete_record,
            save_to_cloud,
            save_test
        ]);

    #[cfg(target_os = "macos")]
    let app_builder = app_builder.plugin(tauri_plugin_window_state::Builder::default().build());

    let app = app_builder.build(tauri::generate_context!())?;

    block_on(initialize_window(app.state(), app.app_handle()))?;

    app.run(|_app_handle, _event| {
        // Can react to events
    });

    Ok(())
}

/*
async fn console_test() -> Result<(), &'static str> {
    let mut database = Database::open("password")?;

    let mut record1 = Record::new("Example".to_string(), Category::Login);
    database
        .save_record(&mut record1)
        .map_err(|_| "Failed to save record")?;

    let mut record2 = Record::new(
        "Testing".to_string(),
        Category::Custom("Testing".to_string()),
    );
    database
        .save_record(&mut record2)
        .map_err(|_| "Failed to save record")?;

    let mut record1_content1 = Content::Email(specific::Email::new(
        "email".to_string(),
        true,
        "example@email.com".to_string(),
    )?);
    database
        .save_content(record1.id(), &mut record1_content1)
        .map_err(|_| "Failed to save content")?;

    let mut record1_content2 = Content::Url(specific::Url::new(
        "website".to_string(),
        true,
        "www.example.com".to_string(),
    )?);
    database
        .save_content(record1.id(), &mut record1_content2)
        .map_err(|_| "Failed to save content")?;

    let pg = utils::password::PasswordGenerator {
        length: 8,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };
    let mut record1_content3 = Content::Password(specific::Password::new(
        "password".to_string(),
        true,
        pg.generate_one()?,
    ));
    database
        .save_content(record1.id(), &mut record1_content3)
        .map_err(|_| "Failed to save content")?;

    for record in database
        .get_all_records()
        .map_err(|_| "Failed to get records")?
    {
        println!("{:?}", record);
    }

    for content in database
        .get_all_content_for_record(record1.id())
        .map_err(|_| "Failed to get contents")?
    {
        println!("{:?}", content);
    }

    let mut record1_content4 = Content::Text(basic::Text::new(
        "note".to_string(),
        false,
        "This is a note".to_string(),
        basic::TextType::Normal,
    ));
    database
        .save_content(record1.id(), &mut record1_content4)
        .map_err(|_| "Failed to save content")?;

    for content in database
        .get_all_content_for_record(record1.id())
        .map_err(|_| "Failed to get contents")?
    {
        println!("{:?}", content);

        if let Content::Password(password) = &content {
            println!(
                "Exposed password: {}",
                utils::password::is_exposed(password.value()).await.unwrap()
            );
            println!(
                "Password score: {}",
                utils::password::score(&utils::password::analyze(password.value()))
            );
        }
    }
    Ok(())
}
*/
