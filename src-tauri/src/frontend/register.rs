use super::*;

/// Creates a menu specific for the register window.
/// # Removed native menu items
/// - macOS
///     - File
///     - Window > Zoom
///     - View
pub fn create_register_menu(package_name: &str) -> Menu {
    #[allow(unused)]
    let mut menu = Menu::default();

    #[cfg(target_os = "macos")]
    {
        menu = Menu::os_default(package_name);
        if let Some(window_submenu) = menu.items.iter_mut().find_map(|item| match item {
            MenuEntry::Submenu(submenu) if submenu.title == "Window" => Some(submenu),
            _ => None,
        }) {
            window_submenu
                .inner
                .items
                .iter()
                .position(|item| matches!(item, MenuEntry::NativeItem(MenuItem::Zoom)))
                .map(|index| window_submenu.inner.items.remove(index));
        }

        menu.items.retain(|item| {
            !matches!(item, MenuEntry::Submenu(submenu) if submenu.title == "File" || submenu.title == "View")
        });
    }

    menu
}

/// Creates register window with specific menu.
/// # Errors
/// Returns an error if the window cannot be created. It will **not** return an error if the window already exists.
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
    ));

    #[cfg(target_os = "macos")]
    let window = window.visible(false);

    window.build()?;

    Ok(())
}

/// Checks if database does not exist, compares passwords, connects to database, creates main window and closes register window.
/// # Dialogs
/// If the database does exist, a dialog will be shown and the application will restart.
/// # Errors
/// Returns an error if:
/// - Passwords do not match
/// - Connection to the database fails
/// - The main window cannot be created
/// - The register window cannot be closed
#[tauri::command]
pub async fn register<'a>(
    password: SecretString,
    confirm_password: SecretString,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    if database_exists(app_handle.clone()).await.is_some() {
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

    connect_database(password, connection, app_handle.clone()).await?;

    #[cfg(target_os = "macos")]
    app_handle
        .save_window_state(StateFlags::all())
        .unwrap_or_default();

    create_main_window(app_handle).map_err(|_| "Failed to create main window")?;

    window.close().map_err(|_| "Failed to close window")?;

    Ok(())
}
