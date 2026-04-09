# Known Bugs

## 1. PopupModal Event Leakage (iOS)
- **Status:** Open
- **Description:** Clicks on the modal content (e.g., a `TextInput`) are being received by the backdrop sibling even when structural isolation (siblings) and event consumption (`on_mouse_down` handlers) are used.
- **Symptoms:** Tapping an input box inside a `PopupModal` triggers the backdrop's `on_close` handler, closing the modal immediately.
- **Logs:** 
  ```
  PopupModal: Content box mouse_down (stopping propagation)
  PopupModal: Backdrop mouse_down
  ```
- **Workaround:** Trigger keyboard/focus programmatically on modal open so the user doesn't have to tap the input field.
