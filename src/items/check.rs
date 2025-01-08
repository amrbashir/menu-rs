// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.inner
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, mem, rc::Rc};

#[cfg(all(feature = "linux-ksni", target_os = "linux"))]
use std::sync::Arc;

#[cfg(all(feature = "linux-ksni", target_os = "linux"))]
use arc_swap::ArcSwap;

use crate::{accelerator::Accelerator, sealed::IsMenuItemBase, IsMenuItem, MenuId, MenuItemKind};

/// A check menu item inside a [`Menu`] or [`Submenu`]
/// and usually contains a text and a check mark or a similar toggle
/// that corresponds to a checked and unchecked states.
///
/// [`Menu`]: crate::Menu
/// [`Submenu`]: crate::Submenu
#[derive(Debug, Clone)]
pub struct CheckMenuItem {
    pub(crate) id: Rc<MenuId>,
    pub(crate) inner: Rc<RefCell<crate::platform_impl::MenuChild>>,
    #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
    pub(crate) compat: Arc<ArcSwap<crate::CompatMenuItem>>,
}

impl IsMenuItemBase for CheckMenuItem {}
impl IsMenuItem for CheckMenuItem {
    fn kind(&self) -> MenuItemKind {
        MenuItemKind::Check(self.clone())
    }

    fn id(&self) -> &MenuId {
        self.id()
    }

    fn into_id(self) -> MenuId {
        self.into_id()
    }
}

impl CheckMenuItem {
    #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
    pub(crate) fn compat_menu_item(
        item: &crate::platform_impl::MenuChild,
    ) -> crate::CompatMenuItem {
        crate::CompatCheckmarkItem {
            id: item.id().0.clone(),
            label: super::strip_mnemonic(item.text()),
            enabled: item.is_enabled(),
            checked: item.is_checked(),
        }
        .into()
    }

    /// Create a new check menu item.
    ///
    /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
    ///   for this check menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn new<S: AsRef<str>>(
        text: S,
        enabled: bool,
        checked: bool,
        accelerator: Option<Accelerator>,
    ) -> Self {
        let inner = crate::platform_impl::MenuChild::new_check(
            text.as_ref(),
            enabled,
            checked,
            accelerator,
            None,
        );

        #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
        let compat = Self::compat_menu_item(&inner);

        Self {
            id: Rc::new(inner.id().clone()),
            inner: Rc::new(RefCell::new(inner)),
            #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
            compat: Arc::new(ArcSwap::from_pointee(compat)),
        }
    }

    /// Create a new check menu item with the specified id.
    ///
    /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
    ///   for this check menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn with_id<I: Into<MenuId>, S: AsRef<str>>(
        id: I,
        text: S,
        enabled: bool,
        checked: bool,
        accelerator: Option<Accelerator>,
    ) -> Self {
        let id = id.into();
        let inner = crate::platform_impl::MenuChild::new_check(
            text.as_ref(),
            enabled,
            checked,
            accelerator,
            Some(id.clone()),
        );

        #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
        let compat = Self::compat_menu_item(&inner);

        Self {
            id: Rc::new(id),
            inner: Rc::new(RefCell::new(inner)),
            #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
            compat: Arc::new(ArcSwap::from_pointee(compat)),
        }
    }

    /// Returns a unique identifier associated with this submenu.
    pub fn id(&self) -> &MenuId {
        &self.id
    }

    /// Get the text for this check menu item.
    pub fn text(&self) -> String {
        self.inner.borrow().text()
    }

    /// Set the text for this check menu item. `text` could optionally contain
    /// an `&` before a character to assign this character as the mnemonic
    /// for this check menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn set_text<S: AsRef<str>>(&self, text: S) {
        let mut inner = self.inner.borrow_mut();
        inner.set_text(text.as_ref());

        #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
        self.compat.store(Arc::new(Self::compat_menu_item(&inner)));

        #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
        crate::send_menu_update();
    }

    /// Get whether this check menu item is enabled or not.
    pub fn is_enabled(&self) -> bool {
        self.inner.borrow().is_enabled()
    }

    /// Enable or disable this check menu item.
    pub fn set_enabled(&self, enabled: bool) {
        let mut inner = self.inner.borrow_mut();
        inner.set_enabled(enabled);

        #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
        self.compat.store(Arc::new(Self::compat_menu_item(&inner)));

        #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
        crate::send_menu_update();
    }

    /// Set this check menu item accelerator.
    pub fn set_accelerator(&self, accelerator: Option<Accelerator>) -> crate::Result<()> {
        self.inner.borrow_mut().set_accelerator(accelerator)
    }

    /// Get whether this check menu item is checked or not.
    pub fn is_checked(&self) -> bool {
        self.inner.borrow().is_checked()
    }

    /// Check or Uncheck this check menu item.
    pub fn set_checked(&self, checked: bool) {
        #[cfg(target_os = "macos")]
        {
            let inner = self.inner.borrow();
            inner.set_checked(checked);
        }

        #[cfg(not(target_os = "macos"))]
        {
            let mut inner = self.inner.borrow_mut();
            inner.set_checked(checked);

            #[cfg(all(feature = "linux-ksni", target_os = "linux"))]
            {
                self.compat.store(Arc::new(Self::compat_menu_item(&inner)));
                crate::send_menu_update();
            }
        }
    }

    /// Convert this menu item into its menu ID.
    pub fn into_id(mut self) -> MenuId {
        // Note: `Rc::into_inner` is available from Rust 1.70
        if let Some(id) = Rc::get_mut(&mut self.id) {
            mem::take(id)
        } else {
            self.id().clone()
        }
    }
}
