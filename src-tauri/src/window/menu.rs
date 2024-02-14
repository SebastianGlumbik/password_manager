pub mod event;
use super::*;
use tauri::{AboutMetadata, CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};

/// Default macOS menu for non-resizable windows.
/// # Removed native menu items
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
    let mut menu = Menu::default();

    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            package_name,
            Menu::new()
                .add_native_item(MenuItem::About(
                    package_name.to_string(),
                    AboutMetadata::default(),
                ))
                .add_native_item(MenuItem::Separator)
                .add_item(CustomMenuItem::new("Settings".to_string(), "Settings"))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

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
        .add_native_item(MenuItem::Separator);

    #[cfg(target_os = "linux")]
    {
        file_menu = file_menu
            .add_item(CustomMenuItem::new("Settings".to_string(), "Settings"))
            .add_native_item(MenuItem::Separator);
    }

    file_menu = file_menu.add_submenu(Submenu::new(
        "Export".to_string(),
        Menu::new().add_item(CustomMenuItem::new(
            "Export Database".to_string(),
            "Database",
        )),
    ));

    menu = menu.add_submenu(Submenu::new("File", file_menu));

    #[cfg(target_os = "macos")]
    {
        menu = menu
            .add_submenu(Submenu::new(
                "Edit",
                Menu::new()
                    .add_native_item(MenuItem::Undo)
                    .add_native_item(MenuItem::Redo)
                    .add_native_item(MenuItem::Separator)
                    .add_native_item(MenuItem::Cut)
                    .add_native_item(MenuItem::Copy)
                    .add_native_item(MenuItem::Paste)
                    .add_native_item(MenuItem::SelectAll),
            ))
            .add_submenu(Submenu::new(
                "View",
                Menu::new().add_native_item(MenuItem::EnterFullScreen),
            ))
            .add_submenu(Submenu::new(
                "Window",
                Menu::new()
                    .add_native_item(MenuItem::Minimize)
                    .add_native_item(MenuItem::Zoom)
                    .add_native_item(MenuItem::Separator)
                    .add_native_item(MenuItem::CloseWindow),
            ));
    }

    menu
}
