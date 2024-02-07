use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};

/// Default macOS menu for non-resizable windows.
/// ### Removed native menu items
///  - Window > Zoom
///  - View
#[cfg(target_os = "macos")]
pub fn menu_os_default_non_resizable(package_name: &str) -> Menu {
    let mut menu = Menu::os_default(package_name);
    if let Some(submenu) = menu.items.iter_mut().find_map(|item| {
        if let MenuEntry::Submenu(submenu) = item {
            if submenu.title == "Window" {
                return Some(submenu);
            }
        }

        None
    }) {
        submenu
            .inner
            .items
            .iter()
            .position(|item| matches!(item, MenuEntry::NativeItem(MenuItem::Zoom)))
            .map(|index| submenu.inner.items.remove(index));
    }

    menu.items
        .iter()
        .position(|item| matches!(item, MenuEntry::Submenu(submenu) if submenu.title == "View"))
        .map(|index| menu.items.remove(index));

    menu
}

/// Creates a menu specific for the non-resizable login window.
/// See [menu_os_default_non_resizable] for more information.
/// # Added custom menu items
/// - File > Start Over
pub fn create_login_menu(package_name: &str) -> Menu {
    #[allow(unused)]
    let mut menu = Menu::default();

    #[cfg(target_os = "macos")]
    {
        menu = menu_os_default_non_resizable(package_name);
        menu.items.iter_mut().for_each(|item| {
            if let MenuEntry::Submenu(submenu) = item {
                if submenu.title == "File" {
                    submenu.inner = Menu::new()
                        .add_item(CustomMenuItem::new("Start Over".to_string(), "Start Over"));
                }
            }
        });
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

/// Creates a menu specific for the non-resizable register window.
/// See [menu_os_default_non_resizable] for more information.
/// # Added custom menu items
/// - File > Choose database
pub fn create_register_menu(package_name: &str) -> Menu {
    #[allow(unused)]
    let mut menu = Menu::default();

    #[cfg(target_os = "macos")]
    {
        menu = menu_os_default_non_resizable(package_name);
        menu.items.iter_mut().for_each(|item| {
            if let MenuEntry::Submenu(submenu) = item {
                if submenu.title == "File" {
                    submenu.inner = Menu::new().add_item(CustomMenuItem::new(
                        "Choose database".to_string(),
                        "Choose database",
                    ));
                }
            }
        });
    }

    #[cfg(target_os = "linux")]
    {
        menu = menu.add_submenu(Submenu::new(
            "File".to_string(),
            Menu::new().add_item(CustomMenuItem::new(
                "Choose database".to_string(),
                "Choose database",
            )),
        ));
    }

    menu
}

/// Creates a menu specific for the resizable main window.
pub fn create_main_menu(package_name: &str) -> Menu {
    #[allow(unused)]
    let mut menu = Menu::default();
    #[allow(unused)]
    let mut file_menu = Menu::new()
        .add_submenu(Submenu::new(
            "New".to_string(),
            Menu::new()
                .add_item(CustomMenuItem::new("New Login".to_string(), "Login"))
                .add_item(CustomMenuItem::new(
                    "New Bank Card".to_string(),
                    "Bank Card",
                ))
                .add_item(CustomMenuItem::new("New Note".to_string(), "Note"))
                .add_item(CustomMenuItem::new("New Other".to_string(), "Other")),
        ))
        .add_native_item(MenuItem::Separator)
        .add_submenu(Submenu::new(
            "Import".to_string(),
            Menu::new().add_item(CustomMenuItem::new("Import CSV".to_string(), "CSV")),
        ))
        .add_submenu(Submenu::new(
            "Export".to_string(),
            Menu::new()
                .add_item(CustomMenuItem::new("Export CSV".to_string(), "CSV"))
                .add_item(CustomMenuItem::new("Database".to_string(), "Database")),
        ));

    #[cfg(target_os = "macos")]
    {
        menu = Menu::os_default(package_name);
        if let Some(submenu) = menu.items.iter_mut().find_map(|item| {
            if let MenuEntry::Submenu(submenu) = item {
                if submenu.title == "File" {
                    return Some(submenu);
                }
            }

            None
        }) {
            submenu.inner = file_menu;
        }

        if let Some(submenu) = menu.items.iter_mut().find_map(|item| {
            if let MenuEntry::Submenu(submenu) = item {
                if submenu.title == package_name {
                    return Some(submenu);
                }
            }

            None
        }) {
            submenu.inner = submenu
                .inner
                .clone()
                .add_native_item(MenuItem::Separator)
                .add_item(CustomMenuItem::new("Log out".to_string(), "Log out"));
        }
    }

    #[cfg(target_os = "linux")]
    {
        file_menu = file_menu
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("Log out".to_string(), "Log out"));
        menu = menu.add_submenu(Submenu::new("File".to_string(), file_menu));
    }

    menu
}
