---
"muda": "minor"
---

**Breaking Change**: `ContextMenu::show_context_menu_for_hwnd`, `ContextMenu::show_context_menu_for_gtk_window` and `ContextMenu::show_context_menu_for_nsview` has been changed to take an optional `Position` type instead of `x` and `y` and if `None` is provided, it will use the current cursor position.
