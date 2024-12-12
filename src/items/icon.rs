// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.inner
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, mem, rc::Rc};

#[cfg(all(feature = "ksni", target_os = "linux"))]
use std::sync::Arc;

#[cfg(all(feature = "ksni", target_os = "linux"))]
use arc_swap::ArcSwap;

use crate::{
    accelerator::Accelerator,
    icon::{Icon, NativeIcon},
    sealed::IsMenuItemBase,
    IsMenuItem, MenuId, MenuItemKind,
};

/// An icon menu item inside a [`Menu`] or [`Submenu`]
/// and usually contains an icon and a text.
///
/// [`Menu`]: crate::Menu
/// [`Submenu`]: crate::Submenu
#[derive(Debug, Clone)]
pub struct IconMenuItem {
    pub(crate) id: Rc<MenuId>,
    pub(crate) inner: Rc<RefCell<crate::platform_impl::MenuChild>>,
    #[cfg(all(feature = "ksni", target_os = "linux"))]
    pub(crate) compat: Arc<ArcSwap<crate::CompatMenuItem>>,
}

impl IsMenuItemBase for IconMenuItem {}
impl IsMenuItem for IconMenuItem {
    fn kind(&self) -> MenuItemKind {
        MenuItemKind::Icon(self.clone())
    }

    fn id(&self) -> &MenuId {
        self.id()
    }

    fn into_id(self) -> MenuId {
        self.into_id()
    }
}

impl IconMenuItem {
    #[cfg(all(feature = "ksni", target_os = "linux"))]
    pub(crate) fn compat_menu_item(
        item: &crate::platform_impl::MenuChild,
    ) -> crate::CompatMenuItem {
        crate::CompatStandardItem {
            id: item.id().0.clone(),
            label: super::strip_mnemonic(item.text()),
            enabled: item.is_enabled(),
            icon: item
                .icon
                .as_ref()
                .map(|icon| icon.to_pixbuf().save_to_bufferv("png", &[]).unwrap()),
            predefined_menu_item_kind: None,
        }
        .into()
    }

    /// Create a new icon menu item.
    ///
    /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
    ///   for this icon menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn new<S: AsRef<str>>(
        text: S,
        enabled: bool,
        icon: Option<Icon>,
        accelerator: Option<Accelerator>,
    ) -> Self {
        let inner = crate::platform_impl::MenuChild::new_icon(
            text.as_ref(),
            enabled,
            icon,
            accelerator,
            None,
        );

        #[cfg(all(feature = "ksni", target_os = "linux"))]
        let compat = Self::compat_menu_item(&inner);

        Self {
            id: Rc::new(inner.id().clone()),
            inner: Rc::new(RefCell::new(inner)),
            #[cfg(all(feature = "ksni", target_os = "linux"))]
            compat: Arc::new(ArcSwap::from_pointee(compat)),
        }
    }

    /// Create a new icon menu item with the specified id.
    ///
    /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
    ///   for this icon menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn with_id<I: Into<MenuId>, S: AsRef<str>>(
        id: I,
        text: S,
        enabled: bool,
        icon: Option<Icon>,
        accelerator: Option<Accelerator>,
    ) -> Self {
        let id = id.into();
        let inner = crate::platform_impl::MenuChild::new_icon(
            text.as_ref(),
            enabled,
            icon,
            accelerator,
            Some(id.clone()),
        );

        #[cfg(all(feature = "ksni", target_os = "linux"))]
        let compat = Self::compat_menu_item(&inner);

        Self {
            id: Rc::new(id),
            inner: Rc::new(RefCell::new(inner)),
            #[cfg(all(feature = "ksni", target_os = "linux"))]
            compat: Arc::new(ArcSwap::from_pointee(compat)),
        }
    }

    /// Create a new icon menu item but with a native icon.
    ///
    /// See [`IconMenuItem::new`] for more info.
    ///
    /// ## Platform-specific:
    ///
    /// - **Windows / Linux**: Unsupported.
    pub fn with_native_icon<S: AsRef<str>>(
        text: S,
        enabled: bool,
        native_icon: Option<NativeIcon>,
        accelerator: Option<Accelerator>,
    ) -> Self {
        let inner = crate::platform_impl::MenuChild::new_native_icon(
            text.as_ref(),
            enabled,
            native_icon,
            accelerator,
            None,
        );

        #[cfg(all(feature = "ksni", target_os = "linux"))]
        let compat = Self::compat_menu_item(&inner);

        Self {
            id: Rc::new(inner.id().clone()),
            inner: Rc::new(RefCell::new(inner)),
            #[cfg(all(feature = "ksni", target_os = "linux"))]
            compat: Arc::new(ArcSwap::from_pointee(compat)),
        }
    }

    /// Create a new icon menu item but with the specified id and a native icon.
    ///
    /// See [`IconMenuItem::new`] for more info.
    ///
    /// ## Platform-specific:
    ///
    /// - **Windows / Linux**: Unsupported.
    pub fn with_id_and_native_icon<I: Into<MenuId>, S: AsRef<str>>(
        id: I,
        text: S,
        enabled: bool,
        native_icon: Option<NativeIcon>,
        accelerator: Option<Accelerator>,
    ) -> Self {
        let id = id.into();
        let inner = crate::platform_impl::MenuChild::new_native_icon(
            text.as_ref(),
            enabled,
            native_icon,
            accelerator,
            Some(id.clone()),
        );

        #[cfg(all(feature = "ksni", target_os = "linux"))]
        let compat = Self::compat_menu_item(&inner);

        Self {
            id: Rc::new(id),
            inner: Rc::new(RefCell::new(inner)),
            #[cfg(all(feature = "ksni", target_os = "linux"))]
            compat: Arc::new(ArcSwap::from_pointee(compat)),
        }
    }

    /// Returns a unique identifier associated with this submenu.
    pub fn id(&self) -> &MenuId {
        &self.id
    }

    /// Get the text for this icon menu item.
    pub fn text(&self) -> String {
        self.inner.borrow().text()
    }

    /// Set the text for this icon menu item. `text` could optionally contain
    /// an `&` before a character to assign this character as the mnemonic
    /// for this icon menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn set_text<S: AsRef<str>>(&self, text: S) {
        let mut inner = self.inner.borrow_mut();
        inner.set_text(text.as_ref());

        #[cfg(all(feature = "ksni", target_os = "linux"))]
        self.compat.store(Arc::new(Self::compat_menu_item(&inner)));
        
        #[cfg(all(feature = "ksni", target_os = "linux"))]
        crate::send_menu_update();
    }

    /// Get whether this icon menu item is enabled or not.
    pub fn is_enabled(&self) -> bool {
        self.inner.borrow().is_enabled()
    }

    /// Enable or disable this icon menu item.
    pub fn set_enabled(&self, enabled: bool) {
        let mut inner = self.inner.borrow_mut();
        inner.set_enabled(enabled);

        #[cfg(all(feature = "ksni", target_os = "linux"))]
        self.compat.store(Arc::new(Self::compat_menu_item(&inner)));
        
        #[cfg(all(feature = "ksni", target_os = "linux"))]
        crate::send_menu_update();
    }

    /// Set this icon menu item accelerator.
    pub fn set_accelerator(&self, accelerator: Option<Accelerator>) -> crate::Result<()> {
        self.inner.borrow_mut().set_accelerator(accelerator)
    }

    /// Get the icon for this icon menu item.
    pub fn icon(&self) -> Option<Icon> {
        self.inner.borrow().icon.clone()
    }

    /// Change this menu item icon or remove it.
    pub fn set_icon(&self, icon: Option<Icon>) {
        let mut inner = self.inner.borrow_mut();
        inner.set_icon(icon);

        #[cfg(all(feature = "ksni", target_os = "linux"))]
        self.compat.store(Arc::new(Self::compat_menu_item(&inner)));
        
        #[cfg(all(feature = "ksni", target_os = "linux"))]
        crate::send_menu_update();
    }

    /// Change this menu item icon to a native image or remove it.
    ///
    /// ## Platform-specific:
    ///
    /// - **Windows / Linux**: Unsupported.
    #[cfg(target_os = "macos")]
    pub fn set_native_icon(&self, icon: Option<NativeIcon>) {
        let mut item = self.inner.borrow_mut();
        item.set_native_icon(icon);
    }

    /// Change this menu item icon to a native image or remove it.
    ///
    /// ## Platform-specific:
    ///
    /// - **Windows / Linux**: Unsupported.
    #[cfg(not(target_os = "macos"))]
    pub fn set_native_icon(&self, _icon: Option<NativeIcon>) {}

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
