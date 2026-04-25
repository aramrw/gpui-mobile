# Design Spec: Cross-Platform Text Input (Desktop Support)

## 1. Overview
The goal is to enable text input support for `gpui-mobile` components (like `TextInput`) on desktop platforms (starting with macOS). The current mobile-first design relies on `gpui_mobile::show_keyboard()`, which is a no-op on desktop. We need a unified way to request text input focus that handles the platform-specific "active input" requirements.

## 2. Architecture
We will extend the existing `Platform` abstraction to include text input management.

### Platform Trait Extension
Add the following methods to the `Platform` trait (or an extension trait if the core trait is rigid):
```rust
fn show_keyboard(&self);
fn hide_keyboard(&self);
```

### Platform Implementations
- **iOS/Android:** Existing logic moves into these methods.
- **macOS:**
    - `show_keyboard()`: Ensures the native `GPUIView` becomes the First Responder. This activates the `NSTextInputClient` protocol, allowing the OS to route IME and keyboard events correctly.
    - `hide_keyboard()`: Resigns First Responder if necessary or simply treats the input as "deactivated."

### Component Level
`TextInput` will be updated to call `cx.platform().show_keyboard()` instead of `gpui_mobile::show_keyboard()`.

## 3. Data Flow
1. `TextInput` calls `platform.show_keyboard()` on tap.
2. Platform-specific implementation (e.g., `MacPlatform`) ensures the native view is ready to receive input.
3. User types characters.
4. Native host (macOS `NSTextInputClient`) dispatches key events/text events.
5. `PlatformInput` events flow into GPUI's existing input handling.

## 4. Risks & Considerations
- **Focus Management:** On desktop, `First Responder` management is crucial. We must ensure that clicking outside the `TextInput` properly resigns focus.
- **IME Compatibility:** macOS `NSTextInputClient` handles IME (e.g., Japanese/Chinese input) by default. Ensuring `GPUIView` correctly implements the protocol is key.

## 5. Review
This approach decouples component logic from mobile-specific global functions, making the component library truly platform-agnostic.
