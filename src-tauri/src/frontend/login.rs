use super::*;

pub fn create_login_menu(package_name: &str) -> Menu {
    let mut menu = Menu::default();

    #[cfg(target_os = "macos")]
    {
        menu = Menu::os_default(package_name);
        if let MenuEntry::Submenu(submenu) = &mut menu.items[1] {
            submenu.inner =
                Menu::new().add_item(CustomMenuItem::new("Start Over".to_string(), "Start Over"));
        }
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
    .menu(create_login_menu(app_handle.package_info().name.as_str()))
    .build()?;

    let window_clone = window.clone();
    window.on_menu_event(move |event| {
        if event.menu_item_id() == "Start Over" {
            block_on(start_over(app_handle.clone(), window_clone.clone()));
        }
    });

    Ok(())
}

/// Login function for the frontend.
/// - Checks if the database exist
/// - Opens the database
#[tauri::command]
pub async fn login<'a, 'b>(
    password: &'a str,
    connection: State<'b, DatabaseConnection>,
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

    // Must be called in this order

    #[cfg(target_os = "macos")]
    app_handle
        .save_window_state(StateFlags::all())
        .unwrap_or_default();

    create_main_window(app_handle).map_err(|_| "Failed to create main window")?;

    window.close().map_err(|_| "Failed to close window")?;

    Ok(())
}
