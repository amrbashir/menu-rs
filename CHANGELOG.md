# Changelog

## \[0.16.0]

- [`cf9dcfa`](https://www.github.com/tauri-apps/muda/commit/cf9dcfafd000336db4e9f239ed5581539d9168f6) ([#236](https://www.github.com/tauri-apps/muda/pull/236) by [@pewsheen](https://www.github.com/tauri-apps/muda/../../pewsheen)) Return `bool` in `ContextMenu::show_context_menu_for_hwnd`, `ContextMenu::show_context_menu_for_nsview` and `ContextMenu::show_context_menu_for_gtk_window` to indicate why the context menu was closed.
- [`99ec648`](https://www.github.com/tauri-apps/muda/commit/99ec648de4dfd3b864540b1e30279c2c7afc1abd) ([#244](https://www.github.com/tauri-apps/muda/pull/244) by [@stijnfrishert](https://www.github.com/tauri-apps/muda/../../stijnfrishert)) Add `Accelerator::modifiers` and `Accelerator::key` getter methods.

## \[0.15.3]

- [`11a1ef8`](https://www.github.com/tauri-apps/muda/commit/11a1ef84fa85cbe2f0bfb0c3a986d7a36d84288f) ([#241](https://www.github.com/tauri-apps/muda/pull/241) by [@amrbashir](https://www.github.com/tauri-apps/muda/../../amrbashir)) On Windows, fix changing state of menu items inside a `muda::Menu` not immedietly reflected on the window menu bar.

## \[0.15.2]

- [`3b58a2e`](https://www.github.com/tauri-apps/muda/commit/3b58a2ef973a7fb46ce6b4cfd34942d990929510) ([#237](https://www.github.com/tauri-apps/muda/pull/237) by [@amrbashir](https://www.github.com/tauri-apps/muda/../../amrbashir)) Fix `PredefinedMenuItem::about` sending events where it shouldn't.

## \[0.15.1]

- [`8bf315e`](https://www.github.com/tauri-apps/muda/commit/8bf315ea31f791e44d3c67f5fdb0ac4c47e16aaf) ([#229](https://www.github.com/tauri-apps/muda/pull/229) by [@amrbashir](https://www.github.com/tauri-apps/muda/../../amrbashir)) On Linux, fix `IconMenuItem` overlapping neighbouring items when added to a `Menu`.

## \[0.15.0]

- [`40d06c5`](https://www.github.com/tauri-apps/muda/commit/40d06c5c9712ab4e12a8bc3a9124e5975df595e3) ([#226](https://www.github.com/tauri-apps/muda/pull/226) by [@amrbashir](https://www.github.com/tauri-apps/muda/../../amrbashir)) **Breaking change** Renamed the `acccelerator` method (which has an extra `c`) on `MenuItemBuilder`, `CheckMenuItemBuilder`, and `IconMenuItemBuilder` to `accelerator`.
- [`0d368bb`](https://www.github.com/tauri-apps/muda/commit/0d368bb32728a104f0d6ad100193b0212495dd64) ([#220](https://www.github.com/tauri-apps/muda/pull/220) by [@madsmtm](https://www.github.com/tauri-apps/muda/../../madsmtm)) **Breaking Change** Changed the type of the pointer passed in `show_context_menu_for_nsview` to `c_void`, and make the method `unsafe`.
- [`63c9f28`](https://www.github.com/tauri-apps/muda/commit/63c9f2873c7d2f6c1b477e0d5c7f79ccda90ea85) ([#224](https://www.github.com/tauri-apps/muda/pull/224) by [@Legend-Master](https://www.github.com/tauri-apps/muda/../../Legend-Master)) Fix `set_theme_for_hwnd` always resulting in dark on Windows, and doesn't refresh until losing and regaining focus
- [`f781c0e`](https://www.github.com/tauri-apps/muda/commit/f781c0edd0af7ab166e10f816978ffed2761376b) ([#227](https://www.github.com/tauri-apps/muda/pull/227) by [@amrbashir](https://www.github.com/tauri-apps/muda/../../amrbashir)) **Breaking change** Marked a few methods with `unsafe` to better represent the safety guarantees:

  - `ContextMenu::show_context_menu_for_hwnd`
  - `ContextMenu::attach_menu_subclass_for_hwnd`
  - `ContextMenu::detach_menu_subclass_from_hwnd`
  - `Menu::init_for_hwnd`
  - `Menu::init_for_hwnd_with_theme`
  - `Menu::set_theme_for_hwnd`
  - `Menu::remove_for_hwnd`
  - `Menu::hide_for_hwnd`
  - `Menu::show_for_hwnd`
  - `Menu::is_visible_on_hwnd`
- [`5c8971a`](https://www.github.com/tauri-apps/muda/commit/5c8971a7c28a48669605236ddc097460ffd3b32f) ([#221](https://www.github.com/tauri-apps/muda/pull/221) by [@madsmtm](https://www.github.com/tauri-apps/muda/../../madsmtm)) Use `objc2` internally, leading to much better memory safety. The crate will panic now if used from a thread that is not the main thread.

## \[0.14.1]

- [`07ca638`](https://www.github.com/tauri-apps/muda/commit/07ca6382bc1ae08984c21034b8033cee3eb147c7) ([#213](https://www.github.com/tauri-apps/muda/pull/213)) Fix handling the separator of `CARGO_PKG_AUTHORS` environment variable value in `from_cargo_metadata` macro.
- [`bb40d8c`](https://www.github.com/tauri-apps/muda/commit/bb40d8cec3187eccb3bce868befe1d0bf0dbf93c) On Windows, fix crash when showing a context menu but dropping the Menu before the context menu is closed.

## \[0.14.0]

- [`11d8b7a`](https://www.github.com/tauri-apps/muda/commit/11d8b7a6fefa2b47b5bd0a113c0f33f0ccdf6647) ([#208](https://www.github.com/tauri-apps/muda/pull/208) by [@amrbashir](https://www.github.com/tauri-apps/muda/../../amrbashir)) Added `about_metadata` module and `about_metadata::from_cargo_metadata` macro.
- [`11d8b7a`](https://www.github.com/tauri-apps/muda/commit/11d8b7a6fefa2b47b5bd0a113c0f33f0ccdf6647) ([#208](https://www.github.com/tauri-apps/muda/pull/208) by [@amrbashir](https://www.github.com/tauri-apps/muda/../../amrbashir)) **Breaking Change** Removed `AboutMetadata::from_cargo_metadata` and `AboutMetadataBuilder::with_cargo_metadata` which had incorrect implementation, use the new `about_metadata::from_cargo_metadata` macro instead.
- [`32bff56`](https://www.github.com/tauri-apps/muda/commit/32bff5610f355de86bfff55605ab3383d40b7d42) ([#210](https://www.github.com/tauri-apps/muda/pull/210) by [@amrbashir](https://www.github.com/tauri-apps/muda/../../amrbashir)) Update `window-sys` crate to `0.59`

## \[0.13.5]

- [`20ea54b`](https://www.github.com/tauri-apps/muda/commit/20ea54b69844b53f09e40e6399bb4ea26af58766)([#200](https://www.github.com/tauri-apps/muda/pull/200)) On macOS, close tray menu before removing it to prevent user click on a released menu item resulting in a crash.

## \[0.13.4]

- [`e758002`](https://www.github.com/tauri-apps/muda/commit/e758002bffe95f2d6f2d106967063ca87eb2f253)([#194](https://www.github.com/tauri-apps/muda/pull/194)) On Windows, fix menubar drawing when using a fixed dark theme while Windows itself in Light theme.

## \[0.13.3]

- [`e758002`](https://www.github.com/tauri-apps/muda/commit/e758002bffe95f2d6f2d106967063ca87eb2f253)([#194](https://www.github.com/tauri-apps/muda/pull/194)) On Windows, add `Menu::init_for_hwnd_with_theme` and `Menu::set_theme_for_hwnd` to control the window menu bar theme.

## \[0.13.2]

- [`1dc9d3f`](https://www.github.com/tauri-apps/muda/commit/1dc9d3f193e698b4a688414cc094ff360cb072a4)([#190](https://www.github.com/tauri-apps/muda/pull/190)) On Linux, fix context menu closing immediately when right click is released.

## \[0.13.1]

- [`2edfbf1`](https://www.github.com/tauri-apps/muda/commit/2edfbf1a3a33199eb963532118d83a40f4d99af2)([#176](https://www.github.com/tauri-apps/muda/pull/176)) On macOS, fix a crash when removing a menu item.

## \[0.13.0]

- [`90926d4`](https://www.github.com/tauri-apps/muda/commit/90926d43f7817ef7061aa8baa8072983a91d6a81)([#174](https://www.github.com/tauri-apps/muda/pull/174)) Moved the following items into `dpi` module which is just an export of `dpi` crate:

  - `Pixel`
  - `validate_scale_factor`
  - `LogicalPosition`
  - `PhysicalPosition`
  - `Position`

## \[0.12.2]

- [`8960f0d`](https://www.github.com/tauri-apps/muda/commit/8960f0ddb52b74f4bd9d5fed5e91b62c5db09c77)([#171](https://www.github.com/tauri-apps/muda/pull/171)) On Windows, fix using multiple context menus resulted in receiving events only for the last used one.

## \[0.12.1]

- [`cbb9fc0`](https://www.github.com/tauri-apps/muda/commit/cbb9fc0d7b98c362dfbdc09122fb9d0b3b7edabb)([#166](https://www.github.com/tauri-apps/muda/pull/166)) On Windows, fix events not emitted for other menus after using a menu as a context menu.
- [`a9937ef`](https://www.github.com/tauri-apps/muda/commit/a9937ef98144b3feaf8a16ae95e0e8c55583bd79)([#164](https://www.github.com/tauri-apps/muda/pull/164)) On Windows, fix menubar removed from window when another menu that was used as a conetxt menu is dropped.

## \[0.12.0]

- [`2d7828f`](https://www.github.com/tauri-apps/muda/commit/2d7828fdd9d216d9a245bad7eae8f096b42948c0)([#157](https://www.github.com/tauri-apps/muda/pull/157)) Refactored the errors when parsing accelerator from string:

  - Added `AcceleratorParseError` error enum.
  - Removed `Error::UnrecognizedAcceleratorCode` enum variant
  - Removed `Error::EmptyAcceleratorToken` enum variant
  - Removed `Error::UnexpectedAcceleratorFormat` enum variant
  - Changed `Error::AcceleratorParseError` inner value from `String` to the newly added `AcceleratorParseError` enum.
- [`2d7828f`](https://www.github.com/tauri-apps/muda/commit/2d7828fdd9d216d9a245bad7eae8f096b42948c0)([#157](https://www.github.com/tauri-apps/muda/pull/157)) Avoid panicing when parsing an invalid `Accelerator` from a string such as `SHIFT+SHIFT` and return an error instead.

## \[0.11.5]

- [`f64a62f`](https://www.github.com/tauri-apps/muda/commit/f64a62fc89d39e9fe6f3951250addf620ba8ba29)([#153](https://www.github.com/tauri-apps/muda/pull/153)) On Linux, fix a regression where menubar is not added as the first child when using a `gtk::Box` as the container.

## \[0.11.4]

- [`a7e61fb`](https://www.github.com/tauri-apps/muda/commit/a7e61fb939866897d0ed3d32b78a3a7cba8689ca)([#149](https://www.github.com/tauri-apps/muda/pull/149)) On macOS, fix a crash when appending separators.

## \[0.11.3]

- [`20c45a4`](https://www.github.com/tauri-apps/muda/commit/20c45a4ccc24914f5ad7533300e0d1e6d7d91384)([#147](https://www.github.com/tauri-apps/muda/pull/147)) On macOS, fixed a panic when releasing separator menu item.

## \[0.11.2]

- [`afd3e2e`](https://www.github.com/tauri-apps/muda/commit/afd3e2ecd4b40def7c0f458cd1465723392c1be1)([#143](https://www.github.com/tauri-apps/muda/pull/143)) Fixes menu item's `enabled` state not applied for submenus on macOS.

## \[0.11.1]

- [`07b188f`](https://www.github.com/tauri-apps/muda/commit/07b188f386522f42f47670d6de30afa4da1a24a8)([#141](https://www.github.com/tauri-apps/muda/pull/141)) Update `windows-sys` crate to `0.52`

## \[0.11.0]

- [`ae316bf`](https://www.github.com/tauri-apps/muda/commit/ae316bf2dd3dbefce5f866b98165b0336bb3010e)([#139](https://www.github.com/tauri-apps/muda/pull/139)) Changed `ContextMenu::show_context_menu_for_gtk_window` to take `gtk::Window` instead of `gtk::ApplicationWindow` and relaxed generic gtk constraints on the following methods:

  - `MenuBar::init_for_gtk_window`
  - `MenuBar::remove_for_gtk_window`
  - `MenuBar::hide_for_gtk_window`
  - `MenuBar::show_for_gtk_window`
  - `MenuBar::is_visible_on_gtk_window`
  - `MenuBar::gtk_menubar_for_gtk_window`

## \[0.10.0]

- [`8d95612`](https://www.github.com/tauri-apps/muda/commit/8d95612c27374637895ac5d28f553143501adeff)([#133](https://www.github.com/tauri-apps/muda/pull/133)) Upgrade `gtk` to 0.18 and bump MSRV to 1.70.0.

## \[0.9.4]

- [`3672a0c`](https://www.github.com/tauri-apps/muda/commit/3672a0c663bb711528d18d1f4abee949f1ca26f2)([#130](https://www.github.com/tauri-apps/muda/pull/130)) Add `PredefinedMenuItem::bring_all_to_front` for 'Bring All to Front' menu item on macOS.
- [`e34040e`](https://www.github.com/tauri-apps/muda/commit/e34040e75a97e5b17d31c6d0f6df007cce5ddfc6)([#126](https://www.github.com/tauri-apps/muda/pull/126)) On Windows, draw over the white line under the menubar in dark mode.

## \[0.9.3]

- [`bdd0c9a`](https://www.github.com/tauri-apps/muda/commit/bdd0c9aa0d7d4158535796a20a1c1597799840a4)([#122](https://www.github.com/tauri-apps/muda/pull/122)) On macOS, fix menu crash due to a double freeing the underlying NsMenu.
- [`f7e3030`](https://www.github.com/tauri-apps/muda/commit/f7e3030f3438f92e92dfa724c5f8a6864387de0f)([#125](https://www.github.com/tauri-apps/muda/pull/125)) On Windwos, redraw the menubar when adding a new menu item or a submenu.

## \[0.9.2]

- [`45345ad`](https://www.github.com/tauri-apps/muda/commit/45345ad631187190799bdaea6f9f0d0d7e245d5e) On macOS, fixed autorelease from separator twice.

## \[0.9.1]

- [`c1fbde7`](https://www.github.com/tauri-apps/muda/commit/c1fbde72a83fdef579436f2fc7a55de77fa907c2)([#116](https://www.github.com/tauri-apps/muda/pull/116)) Added `AboutMetadata::from_cargo_metadata` and `AboutMetadataBuilder::with_cargo_metadata` to build the application metadata from Cargo package metadata.

## \[0.9.0]

- [`02e537e`](https://www.github.com/tauri-apps/muda/commit/02e537ea06e74dd08c3bdeaf350d8c71e8e04f19)([#112](https://www.github.com/tauri-apps/muda/pull/112)) Added `into_id` method to `MenuItem`, `CheckMenuItem`, `PredefinedMenuItem`, `Submenu`, `MenuItemKind` and `IsMenuItem` trait. It moves the menu item into its id.
- [`622f30b`](https://www.github.com/tauri-apps/muda/commit/622f30b838a5dee879715ca5da3ff84bcaa48685) Update `keyboard-types` to `0.7`
- [`bce7540`](https://www.github.com/tauri-apps/muda/commit/bce7540bba0d26ae4e55b3e2d164f1579b58743a)([#113](https://www.github.com/tauri-apps/muda/pull/113)) Add `MenuItemKind::id` convenient method to get access to the inner kind id.

## \[0.8.7]

- [`8d832c0`](https://www.github.com/tauri-apps/muda/commit/8d832c06f4e810278c0da129063231f880c3f918) Wrapped the `id` field of the `Menu` struct in an `Rc` to be consistent with other menu structs and make it cheaper to clone.

## \[0.8.6]

- [`4701bb8`](https://www.github.com/tauri-apps/muda/commit/4701bb836e9d5f5c020c7807616d68bd049679a9)([#105](https://www.github.com/tauri-apps/muda/pull/105)) On Windows, fix menu items inside a context menu not firing events if the context menu was used on a Window that doesn't have a menu bar.

## \[0.8.5]

- [`e046132`](https://www.github.com/tauri-apps/muda/commit/e046132dfba47ab75905ff28c354f83b5b27703d) Changed `IconMenuItem::set_native_icon` to take `&self` instead of `&mut self`.

## \[0.8.4]

- [`47d1808`](https://www.github.com/tauri-apps/muda/commit/47d1808090dc8b064417d5aae192b924fdafaf58) Derive `serde` for more types.

## \[0.8.3]

- [`33168fa`](https://www.github.com/tauri-apps/muda/commit/33168fa0a01e2773d9da4628afd4b147bc603f4d)([#98](https://www.github.com/tauri-apps/muda/pull/98)) On Windows, draw a dark menu bar if the Window supports and has dark-mode enabled.
- [`1a527e8`](https://www.github.com/tauri-apps/muda/commit/1a527e87086cf8d93e02405b87ca97992c08ad04)([#100](https://www.github.com/tauri-apps/muda/pull/100)) Add `PartialEq<&str> for &MenuId` and `PartialEq<String> for &MenuId` implementations. Also add a blanket `From<T> for MenuId` where `T: ToString` implementation.

## \[0.8.2]

- [`829051a`](https://www.github.com/tauri-apps/muda/commit/829051a30a1fef1e83b9bc0d4c10ff9874589f65) Dereference `&String` and `&&str` in `PartialEq` for `MenuId` type

## \[0.8.1]

- [`32be0c5`](https://www.github.com/tauri-apps/muda/commit/32be0c58841252db4f07c273c894a01f1fa414ff)([#94](https://www.github.com/tauri-apps/muda/pull/94)) On Windows, reduce some unneccassry string cloning.
- [`32be0c5`](https://www.github.com/tauri-apps/muda/commit/32be0c58841252db4f07c273c894a01f1fa414ff)([#94](https://www.github.com/tauri-apps/muda/pull/94)) Add `MenuId::new` convenience method.

## \[0.8.0]

- [`662e17d`](https://www.github.com/tauri-apps/muda/commit/662e17d0ec75a746a330390ad3818e35bd2be234)([#92](https://www.github.com/tauri-apps/muda/pull/92)) Add `Drop` implementation for the inner types to release memory and OS resources.
- [`7ca4b11`](https://www.github.com/tauri-apps/muda/commit/7ca4b115646a0ec1c19547267959625b6842b288)([#89](https://www.github.com/tauri-apps/muda/pull/89)) **Breaking Change:** On Linux, `Menu::inti_for_gtk_window` has been changed to require the second paramter to extend `gtk::Box`. This ensures that the menu bar is added at the beginning of the box instead of at the bottom.
- [`bb92b56`](https://www.github.com/tauri-apps/muda/commit/bb92b5667eb43d39a162b86f0b779d06e36eca52) On macOS, changed `Submenu::set_windows_menu_for_nsapp` and `Submenu::set_help_menu_for_nsapp` to `Submenu::set_as_windows_menu_for_nsapp` and `Submenu::set_as_help_menu_for_nsapp`
- [`874f345`](https://www.github.com/tauri-apps/muda/commit/874f345f3c719d70065fceeabebed6224d857813) Add `MenuId` struct an changed all `.id()` methods to return `MenuId` instead of a u32.
- [`043026c`](https://www.github.com/tauri-apps/muda/commit/043026c30d46f81a0fc2975d7e6bea10421ceb47)([#93](https://www.github.com/tauri-apps/muda/pull/93)) Add `Menu/Submenu::remove_at` to remove an item at specified index.
- [`662e17d`](https://www.github.com/tauri-apps/muda/commit/662e17d0ec75a746a330390ad3818e35bd2be234)([#92](https://www.github.com/tauri-apps/muda/pull/92)) On Windows, fix `.set_text()` sometimes adding gebberish characters after multiple calls.

## \[0.7.3]

- [`22956ec`](https://www.github.com/tauri-apps/muda/commit/22956ec724673d21e1d6a675c536eff060737e02)([#87](https://www.github.com/tauri-apps/muda/pull/87)) Fix `remove_for_nsapp` not working.

## \[0.7.2]

- [`0bad3ac`](https://www.github.com/tauri-apps/muda/commit/0bad3aca9608df04656757064fe85d757ff17513)([#84](https://www.github.com/tauri-apps/muda/pull/84)) Manually retain/release NSMenu reference.

## \[0.7.1]

- [`7a3bc55`](https://www.github.com/tauri-apps/muda/commit/7a3bc5505f60fa16a34aac4fb209dbb4968db9bf)([#81](https://www.github.com/tauri-apps/muda/pull/81)) On Windows, fix `ContextMenu::detach_menu_subclass_from_hwnd` crashing and terminating the thread.

## \[0.7.0]

- [`ee30bf8`](https://www.github.com/tauri-apps/muda/commit/ee30bf8d29895c35d7cda0d67d9d64b71910380a)([#73](https://www.github.com/tauri-apps/muda/pull/73)) Added the `builders` which contains convenient builder types, like `AboutMetadataBuilder`, `MenuItemBuilder`, `SubmenuBuilder` ...etc.
- [`c7ec320`](https://www.github.com/tauri-apps/muda/commit/c7ec3207388947b5572847e589eb494d0222373d)([#78](https://www.github.com/tauri-apps/muda/pull/78)) **Breaking Change**: `ContextMenu::show_context_menu_for_hwnd`, `ContextMenu::show_context_menu_for_gtk_window` and `ContextMenu::show_context_menu_for_nsview` has been changed to take an optional `Into<Position>` type instead of `x` and `y`. if `None` is provided, it will use the current cursor position.
- [`98701d0`](https://www.github.com/tauri-apps/muda/commit/98701d0b3277dcb63ee50a8a11f5b008ed432307)([#75](https://www.github.com/tauri-apps/muda/pull/75)) **Breaking Change**: Changed `Menu::init_for_gtk_window` to accept a second argument for a container to which the menu bar should be added, if `None` was provided, it will add it to the window directly. The method will no longer create a `gtk::Box` and append it to the window, instead you should add the box to the window yourself, then pass a reference to it to the method so it can be used as the container for the menu bar.
- [`20c05ce`](https://www.github.com/tauri-apps/muda/commit/20c05ceae677338b2b9dbf247a86d4049280cc90)([#79](https://www.github.com/tauri-apps/muda/pull/79)) **Breaking Change**: Removed `MenuItemType` enum and replaced with `MenuItemKind` enum. `Menu::items` and `Submenu::items` methods will now return `Vec<MenuItemKind>` instead of `Vec<Box<dyn MenuItemExt>>`
- [`0000e56`](https://www.github.com/tauri-apps/muda/commit/0000e569746e7cb630a1453a401bf8f6b0568e9d)([#71](https://www.github.com/tauri-apps/muda/pull/71)) **Breaking Change**: Changed `MenuItemExt` trait name to `IsMenuItem`
- [`ee30bf8`](https://www.github.com/tauri-apps/muda/commit/ee30bf8d29895c35d7cda0d67d9d64b71910380a)([#73](https://www.github.com/tauri-apps/muda/pull/73)) Impl `TryFrom<&str>` and `TryFrom<String>` for `Accelerator`.

## \[0.6.0]

- [`ac14222`](https://www.github.com/tauri-apps/muda/commit/ac142229340c8ded63316fbc1cd1c11bf27e0890)([#69](https://www.github.com/tauri-apps/muda/pull/69)) Add `common-controls-v6` feature flag, disabled by default, which could be used to enable usage of `TaskDialogIndirect` API from `ComCtl32.dll` v6 on Windows for The predefined `About` menu item.
- [`7af4477`](https://www.github.com/tauri-apps/muda/commit/7af44778962de62bf6d8b05aab08bb2e689295fe)([#67](https://www.github.com/tauri-apps/muda/pull/67)) Add `libxdo` feature flag, enabled by default, to control whether to link `libxdo` on Linux or not.
- [`fabbbac`](https://www.github.com/tauri-apps/muda/commit/fabbbacb4b8d77c84cd318a21df1951063e7ea14)([#66](https://www.github.com/tauri-apps/muda/pull/66)) Add support for `AboutMetadata` on macOS

## \[0.5.0]

- Add `(MenuItem|CheckMenuItem|IconMenuItem)::set_accelerator` to change or disable the accelerator after creation.
  - [47ba0b4](https://www.github.com/tauri-apps/muda/commit/47ba0b47ed28a93428c253e8bac397e0b9cb8dec) feat: add `set_accelerator` ([#64](https://www.github.com/tauri-apps/muda/pull/64)) on 2023-05-04

## \[0.4.5]

- On Windows, fix panic when click a menu item while the `PredefinedMenuItem::about` dialog is open.
  - [f3883ee](https://www.github.com/tauri-apps/muda/commit/f3883ee2d4d8773e6b77e36700edb4ca7cb0988e) fix(windows): run the about dialog in its own thread, closes [#57](https://www.github.com/tauri-apps/muda/pull/57) ([#60](https://www.github.com/tauri-apps/muda/pull/60)) on 2023-03-27
- On Windows, Fix a panic when adding `CheckMenuItem` to a `Menu`.
  - [059fceb](https://www.github.com/tauri-apps/muda/commit/059fceb13007760d9e41b65068c91442eda64626) fix(windows): downcast check menu item correctly ([#58](https://www.github.com/tauri-apps/muda/pull/58)) on 2023-03-27

## \[0.4.4]

- On Windows, fix `MenuEvent` not triggered for `IconMenuItem`.
  - [88d3520](https://www.github.com/tauri-apps/muda/commit/88d352033ba571126a11bc681ee3b346b7579916) fix(Windows): dispatch menu event for icon menu item ([#53](https://www.github.com/tauri-apps/muda/pull/53)) on 2023-03-06
- On Windows, The `Close` predefined menu item will send `WM_CLOSE` to the window instead of calling `DestroyWindow` to let the developer catch this event and decide whether to close the window or not.
  - [f322ad4](https://www.github.com/tauri-apps/muda/commit/f322ad454dcd206e2802bb7c65f0a55616a8d002) fix(Windows): send `WM_CLOSE` instead of `DestroyWindow` ([#55](https://www.github.com/tauri-apps/muda/pull/55)) on 2023-03-06

## \[0.4.3]

- Implement `PredefinedMenuItemm::maximize` and `PredefinedMenuItemm::hide` on Windows.
  - [d2bd85b](https://www.github.com/tauri-apps/muda/commit/d2bd85bf7ec4b0bc974d487adaacb6a99b82fa91) docs: add docs for `PredefinedMenuItem` ([#51](https://www.github.com/tauri-apps/muda/pull/51)) on 2023-02-28
- Add docs for predefined menu items
  - [d2bd85b](https://www.github.com/tauri-apps/muda/commit/d2bd85bf7ec4b0bc974d487adaacb6a99b82fa91) docs: add docs for `PredefinedMenuItem` ([#51](https://www.github.com/tauri-apps/muda/pull/51)) on 2023-02-28

## \[0.4.2]

- Fix panic when updating a `CheckMenuItem` right after it was clicked.
  - [923af09](https://www.github.com/tauri-apps/muda/commit/923af09abfe885995ae0a4ef30f8a304cc4c20d2) fix(linux): fix multiple borrow panic ([#48](https://www.github.com/tauri-apps/muda/pull/48)) on 2023-02-14

## \[0.4.1]

- Update docs
  - [4b2ebc2](https://www.github.com/tauri-apps/muda/commit/4b2ebc247cfef64bcaab2ab619e30b65db37a72f) docs: update docs on 2023-02-08

## \[0.4.0]

- Bump gtk version: 0.15 -> 0.16
  - [fb3d0aa](https://www.github.com/tauri-apps/muda/commit/fb3d0aa303a0ee4ffff6d3de97cc363f1ef6d34b) chore(deps): bump gtk version 0.15 -> 0.16 ([#38](https://www.github.com/tauri-apps/muda/pull/38)) on 2023-01-26

## \[0.3.0]

- Add `MenuEvent::set_event_handler` to set a handler for new menu events.
  - [f871c68](https://www.github.com/tauri-apps/muda/commit/f871c68e81aa10f9541c386615a05a2e455e5a82) refactor: allow changing the menu event sender ([#35](https://www.github.com/tauri-apps/muda/pull/35)) on 2023-01-03
- **Breaking change** Remove `menu_event_receiver` function, use `MenuEvent::receiver` instead.
  - [f871c68](https://www.github.com/tauri-apps/muda/commit/f871c68e81aa10f9541c386615a05a2e455e5a82) refactor: allow changing the menu event sender ([#35](https://www.github.com/tauri-apps/muda/pull/35)) on 2023-01-03

## \[0.2.0]

- Add `IconMenuItem`
  - [7fc1b02](https://www.github.com/tauri-apps/muda/commit/7fc1b02cac65f2524220cb79263643505e286863) feat: add `IconMenuItem`, closes [#30](https://www.github.com/tauri-apps/muda/pull/30) ([#32](https://www.github.com/tauri-apps/muda/pull/32)) on 2022-12-30

## \[0.1.1]

- Derive `Copy` for `Accelerator` type.
  - [e80c113](https://www.github.com/tauri-apps/muda/commit/e80c113d8c8db8137f97829b071b443772d4805c) feat: derive `Copy` for `Accelerator` on 2022-12-12
- Fix parsing one letter string as valid accelerator without modifiers.
  - [0173987](https://www.github.com/tauri-apps/muda/commit/0173987ed5da605ddc20e49fce57ba884ed0d5f4) fix: parse one letter string to valid accelerator ([#28](https://www.github.com/tauri-apps/muda/pull/28)) on 2022-12-20

## \[0.1.0]

- Initial Release.
  - [0309d10](https://www.github.com/tauri-apps/muda/commit/0309d101b16663ce67b518f8aa1d2c4af0de6dee) chore: prepare for first release on 2022-12-05
