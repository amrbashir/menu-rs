use std::{cell::RefCell, mem, rc::Rc};

#[cfg(feature = "ksni")]
use std::sync::Arc;

#[cfg(feature = "ksni")]
use arc_swap::ArcSwap;

use crate::{
    accelerator::Accelerator, sealed::IsMenuItemBase, IsMenuItem, MenuId, MenuItemKind,
};

/// A menu item inside a [`Menu`] or [`Submenu`] and contains only text.
///
/// [`Menu`]: crate::Menu
/// [`Submenu`]: crate::Submenu
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub(crate) id: Rc<MenuId>,
    pub(crate) inner: Rc<RefCell<crate::platform_impl::MenuChild>>,
    #[cfg(feature = "ksni")]
    pub(crate) compat: Arc<ArcSwap<crate::CompatMenuItem>>,
}

impl IsMenuItemBase for MenuItem {}
impl IsMenuItem for MenuItem {
    fn kind(&self) -> MenuItemKind {
        MenuItemKind::MenuItem(self.clone())
    }

    fn id(&self) -> &MenuId {
        self.id()
    }

    fn into_id(self) -> MenuId {
        self.into_id()
    }
}

impl MenuItem {
    #[cfg(feature = "ksni")]
    pub(crate) fn compat_menu_item(
        item: &crate::platform_impl::MenuChild,
    ) -> crate::CompatMenuItem {
        crate::CompatStandardItem {
            id: item.id().0.clone(),
            label: super::strip_mnemonic(item.text()),
            enabled: item.is_enabled(),
            icon: None,
            predefined_menu_item_kind: None,
        }
        .into()
    }

    /// Create a new menu item.
    ///
    /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
    ///   for this menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn new<S: AsRef<str>>(text: S, enabled: bool, accelerator: Option<Accelerator>) -> Self {
        let inner = crate::platform_impl::MenuChild::new(text.as_ref(), enabled, accelerator, None);

        #[cfg(feature = "ksni")]
        let compat = Self::compat_menu_item(&inner);

        Self {
            id: Rc::new(inner.id().clone()),
            inner: Rc::new(RefCell::new(inner)),
            #[cfg(feature = "ksni")]
            compat: Arc::new(ArcSwap::from_pointee(compat)),
        }
    }

    /// Create a new menu item with the specified id.
    ///
    /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
    ///   for this menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn with_id<I: Into<MenuId>, S: AsRef<str>>(
        id: I,
        text: S,
        enabled: bool,
        accelerator: Option<Accelerator>,
    ) -> Self {
        let id = id.into();
        let inner = crate::platform_impl::MenuChild::new(
            text.as_ref(),
            enabled,
            accelerator,
            Some(id.clone()),
        );

        #[cfg(feature = "ksni")]
        let compat = Self::compat_menu_item(&inner);

        Self {
            id: Rc::new(id),
            inner: Rc::new(RefCell::new(inner)),
            #[cfg(feature = "ksni")]
            compat: Arc::new(ArcSwap::from_pointee(compat)),
        }
    }

    /// Returns a unique identifier associated with this menu item.
    pub fn id(&self) -> &MenuId {
        &self.id
    }

    /// Set the text for this menu item.
    pub fn text(&self) -> String {
        self.inner.borrow().text()
    }

    /// Set the text for this menu item. `text` could optionally contain
    /// an `&` before a character to assign this character as the mnemonic
    /// for this menu item. To display a `&` without assigning a mnemenonic, use `&&`.
    pub fn set_text<S: AsRef<str>>(&self, text: S) {
        let mut inner = self.inner.borrow_mut();
        inner.set_text(text.as_ref());

        #[cfg(feature = "ksni")]
        self.compat.store(Arc::new(Self::compat_menu_item(&inner)));

        #[cfg(feature = "ksni")]
        crate::send_menu_update();
    }

    /// Get whether this menu item is enabled or not.
    pub fn is_enabled(&self) -> bool {
        self.inner.borrow().is_enabled()
    }

    /// Enable or disable this menu item.
    pub fn set_enabled(&self, enabled: bool) {
        let mut inner = self.inner.borrow_mut();
        inner.set_enabled(enabled);

        #[cfg(feature = "ksni")]
        self.compat.store(Arc::new(Self::compat_menu_item(&inner)));

        #[cfg(feature = "ksni")]
        crate::send_menu_update();
    }

    /// Set this menu item accelerator.
    pub fn set_accelerator(&self, accelerator: Option<Accelerator>) -> crate::Result<()> {
        self.inner.borrow_mut().set_accelerator(accelerator)
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
