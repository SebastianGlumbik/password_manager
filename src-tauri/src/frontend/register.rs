use super::*;

pub fn create_register_menu(package_name: &str) -> Menu {
    let mut menu = Menu::default();

    #[cfg(target_os = "macos")]
    {
        menu = Menu::os_default(package_name);
        menu.items.remove(1);
    }

    menu
}

pub fn create_register_window(app_handle: AppHandle) -> tauri::Result<()> {
    if app_handle.get_window("register").is_some() {
        return Ok(());
    }

    let window = WindowBuilder::new(
        &app_handle,
        "register",
        tauri::WindowUrl::App("index.html".into()),
    )
    .title(app_handle.package_info().name.as_str())
    .disable_file_drop_handler()
    .resizable(false)
    .inner_size(600f64, 400f64)
    .menu(create_register_menu(
        app_handle.package_info().name.as_str(),
    ))
    .build()?;

    Ok(())
}

/// Register function for the frontend.
/// - Compares the passwords
/// - Checks if the database exists
/// - Creates the database
#[tauri::command]
pub async fn register<'a, 'b, 'c>(
    password: &'a str,
    confirm_password: &'b str,
    connection: State<'c, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    if password != confirm_password {
        return Err("Passwords do not match.");
    }

    if database_exists(app_handle.clone()).await.is_some() {
        tauri::api::dialog::blocking::message(
            Some(&window),
            "Error",
            "Database already exists, application will now restart",
        );
        app_handle.restart();
    }

    connect_database(password, connection, app_handle.clone()).await?;

    // Must be called in this order

    #[cfg(target_os = "macos")]
    app_handle
        .save_window_state(StateFlags::all())
        .unwrap_or_default();

    create_main_window(app_handle).map_err(|_| "Failed to create main window")?;

    window.close().map_err(|_| "Failed to close window")?;

    Ok(())
}
