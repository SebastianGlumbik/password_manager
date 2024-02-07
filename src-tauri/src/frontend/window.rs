use super::*;
use tauri::WindowBuilder;

/// Creates login window with specific menu ([create_login_menu]). Does not create the window content.
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
        menu_event(event, app_handle.clone(), window_clone.clone());
    });

    Ok(())
}

/// Creates register window with specific menu ([create_register_menu]). Does not create the window content.
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

    let window = window.build()?;

    let window_clone = window.clone();
    window.on_menu_event(move |event| {
        menu_event(event, app_handle.clone(), window_clone.clone());
    });

    Ok(())
}

/// Creates main window with specific menu ([create_main_menu]). Does not create the window content.
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
    .min_inner_size(720f64, 350f64)
    .menu(create_main_menu(app_handle.package_info().name.as_str()));

    #[cfg(target_os = "macos")]
    let window = window.visible(false);

    let window = window.build()?;

    let window_clone = window.clone();
    window.on_menu_event(move |event| {
        menu_event(event, app_handle.clone(), window_clone.clone());
    });

    Ok(())
}
