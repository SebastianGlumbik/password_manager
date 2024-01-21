use super::*;

pub fn create_main_menu(package_name: &str) -> Menu {
    #[allow(unused)]
    let mut menu = Menu::default();

    #[cfg(target_os = "macos")]
    {
        menu = Menu::os_default(package_name);
        // TODO add menu items for macOS
    }

    #[cfg(target_os = "linux")]
    {
        // TODO add menu items for Linux
    }

    menu
}

/// Creates main window with specific menu.
/// # Errors
/// Returns an error if the window cannot be created. It will **not** return an error if the window already exists.
pub fn create_main_window(app_handle: AppHandle) -> tauri::Result<()> {
    if app_handle.get_window("main").is_some() {
        return Ok(());
    }

    let window = WindowBuilder::new(
        &app_handle,
        "main",
        tauri::WindowUrl::App("index.html".into()),
    )
    .title(app_handle.package_info().name.as_str())
    .disable_file_drop_handler()
    .resizable(true)
    .min_inner_size(640f64, 480f64)
    .menu(create_main_menu(app_handle.package_info().name.as_str()));

    #[cfg(target_os = "macos")]
    let window = window.visible(false);

    let window = window.build()?;

    window.on_menu_event(move |event| {
        // TODO handle menu events, window.emit() can be used to send events to the frontend
    });

    Ok(())
}

#[tauri::command]
pub async fn load_data() -> Result<String, &'static str> {
    // Simulate loading data
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Ok("Oops, it's not built yet.".to_string())
}
