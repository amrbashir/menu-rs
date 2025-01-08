// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;
#[cfg(target_os = "linux")]
#[path = "gtk/mod.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod platform;

#[cfg(target_os = "linux")]
pub use platform::AboutDialog;

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

#[cfg(all(feature = "linux-ksni", target_os = "linux"))]
use std::sync::Arc;

#[cfg(all(feature = "linux-ksni", target_os = "linux"))]
use arc_swap::ArcSwap;

use crate::{IsMenuItem, MenuItemKind};

pub(crate) use self::platform::*;

impl dyn IsMenuItem + '_ {
    fn child(&self) -> Rc<RefCell<MenuChild>> {
        match self.kind() {
            MenuItemKind::MenuItem(i) => i.inner,
            MenuItemKind::Submenu(i) => i.inner,
            MenuItemKind::Predefined(i) => i.inner,
            MenuItemKind::Check(i) => i.inner,
            MenuItemKind::Icon(i) => i.inner,
        }
    }
}

/// Internal utilities
impl MenuChild {
    #[cfg(not(target_os = "linux"))]
    fn kind(&self, c: Rc<RefCell<MenuChild>>) -> MenuItemKind {
        use crate::{items::*, MenuItemType};

        match self.item_type() {
            MenuItemType::Submenu => {
                let id = c.borrow().id().clone();
                MenuItemKind::Submenu(Submenu {
                    id: Rc::new(id),
                    inner: c,
                })
            }
            MenuItemType::MenuItem => {
                let id = c.borrow().id().clone();
                MenuItemKind::MenuItem(MenuItem {
                    id: Rc::new(id),
                    inner: c,
                })
            }
            MenuItemType::Predefined => {
                let id = c.borrow().id().clone();
                MenuItemKind::Predefined(PredefinedMenuItem {
                    id: Rc::new(id),
                    inner: c,
                })
            }
            MenuItemType::Check => {
                let id = c.borrow().id().clone();
                MenuItemKind::Check(CheckMenuItem {
                    id: Rc::new(id),
                    inner: c,
                })
            }
            MenuItemType::Icon => {
                let id = c.borrow().id().clone();
                MenuItemKind::Icon(IconMenuItem {
                    id: Rc::new(id),
                    inner: c,
                })
            }
        }
    }
}

#[allow(unused)]
impl MenuItemKind {
    pub(crate) fn as_ref(&self) -> &dyn IsMenuItem {
        match self {
            MenuItemKind::MenuItem(i) => i,
            MenuItemKind::Submenu(i) => i,
            MenuItemKind::Predefined(i) => i,
            MenuItemKind::Check(i) => i,
            MenuItemKind::Icon(i) => i,
        }
    }

    pub(crate) fn child(&self) -> Ref<MenuChild> {
        match self {
            MenuItemKind::MenuItem(i) => i.inner.borrow(),
            MenuItemKind::Submenu(i) => i.inner.borrow(),
            MenuItemKind::Predefined(i) => i.inner.borrow(),
            MenuItemKind::Check(i) => i.inner.borrow(),
            MenuItemKind::Icon(i) => i.inner.borrow(),
        }
    }

    pub(crate) fn child_mut(&self) -> RefMut<MenuChild> {
        match self {
            MenuItemKind::MenuItem(i) => i.inner.borrow_mut(),
            MenuItemKind::Submenu(i) => i.inner.borrow_mut(),
            MenuItemKind::Predefined(i) => i.inner.borrow_mut(),
            MenuItemKind::Check(i) => i.inner.borrow_mut(),
            MenuItemKind::Icon(i) => i.inner.borrow_mut(),
        }
    }

    #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
    pub(crate) fn compat_child(&self) -> Arc<ArcSwap<crate::CompatMenuItem>> {
        use crate::items::*;
        match self {
            MenuItemKind::MenuItem(i) => i.compat.clone(),
            MenuItemKind::Submenu(i) => i.compat.clone(),
            MenuItemKind::Predefined(i) => i.compat.clone(),
            MenuItemKind::Check(i) => i.compat.clone(),
            MenuItemKind::Icon(i) => i.compat.clone(),
        }
    }
}
