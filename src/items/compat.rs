use std::sync::Arc;

use arc_swap::ArcSwap;

use crate::{ContextMenu, MenuItemKind, PredefinedMenuItemKind};

#[derive(Debug, Clone)]
pub struct CompatStandardItem {
    pub id: String,
    pub label: String,
    pub enabled: bool,
    pub icon: Option<Vec<u8>>,
    pub predefined_menu_item_kind: Option<PredefinedMenuItemKind>,
}

#[derive(Debug, Clone)]
pub struct CompatCheckmarkItem {
    pub id: String,
    pub label: String,
    pub enabled: bool,
    pub checked: bool,
}

#[derive(Debug, Clone)]
pub struct CompatSubMenuItem {
    pub label: String,
    pub enabled: bool,
    pub submenu: Vec<Arc<ArcSwap<CompatMenuItem>>>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum CompatMenuItem {
    Standard(CompatStandardItem),
    Checkmark(CompatCheckmarkItem),
    SubMenu(CompatSubMenuItem),
    Separator,
}

impl From<CompatStandardItem> for CompatMenuItem {
    fn from(item: CompatStandardItem) -> Self {
        CompatMenuItem::Standard(item)
    }
}

impl From<CompatCheckmarkItem> for CompatMenuItem {
    fn from(item: CompatCheckmarkItem) -> Self {
        CompatMenuItem::Checkmark(item)
    }
}

impl From<CompatSubMenuItem> for CompatMenuItem {
    fn from(item: CompatSubMenuItem) -> Self {
        CompatMenuItem::SubMenu(item)
    }
}

impl From<MenuItemKind> for CompatMenuItem {
    fn from(item: MenuItemKind) -> Self {
        match item {
            MenuItemKind::MenuItem(menu_item) => CompatStandardItem {
                id: menu_item.id().0.clone(),
                label: strip_accelerator(menu_item.text()),
                enabled: menu_item.is_enabled(),
                icon: None,
                predefined_menu_item_kind: None,
            }
            .into(),
            MenuItemKind::Submenu(submenu) => CompatSubMenuItem {
                label: strip_accelerator(submenu.text()),
                enabled: submenu.is_enabled(),
                submenu: submenu.compat_items(),
            }
            .into(),
            MenuItemKind::Predefined(predefined_menu_item) => {
                match predefined_menu_item.predefined_item_kind() {
                    Some(PredefinedMenuItemKind::Separator) => CompatMenuItem::Separator,
                    Some(predefined_menu_item_kind) => CompatStandardItem {
                        id: predefined_menu_item.id().0.clone(),
                        label: strip_accelerator(predefined_menu_item.text()),
                        enabled: true,
                        icon: None,
                        predefined_menu_item_kind: Some(predefined_menu_item_kind),
                    }
                    .into(),
                    _ => CompatStandardItem {
                        id: predefined_menu_item.id().0.clone(),
                        label: strip_accelerator(predefined_menu_item.text()),
                        enabled: true,
                        icon: None,
                        predefined_menu_item_kind: None,
                    }
                    .into(),
                }
            }
            MenuItemKind::Check(check_menu_item) => CompatCheckmarkItem {
                id: check_menu_item.id().0.clone(),
                label: strip_accelerator(check_menu_item.text()),
                enabled: check_menu_item.is_enabled(),
                checked: check_menu_item.is_checked(),
            }
            .into(),
            MenuItemKind::Icon(icon_menu_item) => CompatStandardItem {
                id: icon_menu_item.id().0.clone(),
                label: strip_accelerator(icon_menu_item.text()),
                enabled: icon_menu_item.is_enabled(),
                icon: icon_menu_item
                    .icon()
                    .map(|icon| icon.to_pixbuf().save_to_bufferv("png", &[]).unwrap()),
                predefined_menu_item_kind: None,
            }
            .into(),
        }
    }
}

pub fn strip_accelerator(text: impl AsRef<str>) -> String {
    text.as_ref().replace('&', "")
}
