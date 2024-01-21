use super::*;

/// Creates a menu specific for the login window.
/// # Added custom menu items
/// - Start Over
/// # Removed native menu items
/// - macOS
///     - Window > Zoom
///     - View
fn create_login_menu(package_name: &str) -> Menu {
    #[allow(unused)]
    let mut menu = Menu::default();

    #[cfg(target_os = "macos")]
    {
        menu = Menu::os_default(package_name);
        menu.items.iter_mut().for_each(|item| {
            if let MenuEntry::Submenu(submenu) = item {
                if submenu.title == "File" {
                    submenu.inner = Menu::new()
                        .add_item(CustomMenuItem::new("Start Over".to_string(), "Start Over"));
                } else if submenu.title == "Window" {
                    let index = submenu
                        .inner
                        .items
                        .iter()
                        .position(|item| matches!(item, MenuEntry::NativeItem(MenuItem::Zoom)));
                    if let Some(index) = index {
                        submenu.inner.items.remove(index);
                    }
                }
            }
        });

        menu.items
            .iter()
            .position(|item| matches!(item, MenuEntry::Submenu(submenu) if submenu.title == "View"))
            .map(|index| menu.items.remove(index));
    }

    #[cfg(target_os = "linux")]
    {
        menu = menu.add_submenu(Submenu::new(
            "File".to_string(),
            Menu::new().add_item(CustomMenuItem::new("Start Over".to_string(), "Start Over")),
        ));
    }

    menu
}

/// Creates login window with specific menu.
/// # Errors
/// Returns an error if the window cannot be created. It will **not** return an error if the window already exists.
pub fn create_login_window(app_handle: AppHandle) -> tauri::Result<()> {
    if app_handle.get_window("login").is_some() {
        return Ok(());
    }

    let window = WindowBuilder::new(
        &app_handle,
        "login",
        tauri::WindowUrl::App("index.html".into()),
    )
    .title(app_handle.package_info().name.as_str())
    .disable_file_drop_handler()
    .resizable(false)
    .inner_size(400f64, 400f64)
    .menu(create_login_menu(app_handle.package_info().name.as_str()));

    #[cfg(target_os = "macos")]
    let window = window.visible(false);

    let window = window.build()?;

    let window_clone = window.clone();
    window.on_menu_event(move |event| {
        if event.menu_item_id() == "Start Over" {
            block_on(start_over(app_handle.clone(), window_clone.clone()));
        }
    });

    Ok(())
}

/// Checks if the database exists, connects to it, opens main window and closes login window.
/// # Dialogs
/// If the database does not exist, a blocking dialog message will be shown and the application will restart.
/// # Errors
/// Returns an error if:
/// - Connection to the database fails
/// - The main window cannot be created
/// - The login window cannot be closed
#[tauri::command]
pub async fn login<'a>(
    password: SecretString,
    connection: State<'a, DatabaseConnection>,
    app_handle: AppHandle,
    window: Window,
) -> Result<(), &'static str> {
    if database_exists(app_handle.clone()).await.is_none() {
        tauri::api::dialog::blocking::message(
            Some(&window),
            "Error",
            "Database does not exist, application will now restart",
        );
        app_handle.restart();
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
