# Cross-Platform Text Input Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable desktop text input by bridging the GPUI `Platform` trait to native platform IME systems (starting with macOS).

**Architecture:** Extend the `Platform` trait to include `show_keyboard()`/`hide_keyboard()` and implement them in the desktop platform layer to manage IME focus (First Responder status).

**Tech Stack:** Rust, GPUI, macOS Cocoa/Obj-C (via `objc` / `msg_send!`).

---

### Task 1: Extend the `Platform` trait

We need to add the input management methods to the trait used by `TextInput`.

**Files:**
- Modify: `crates/gpui/src/platform.rs` (or equivalent file defining the `Platform` trait). *Note: Since GPUI is a git dependency, we will need to confirm where the trait is defined.*

- [ ] **Step 1: Locate and modify the `Platform` trait.**
    - If `gpui` is a submodule, we may need to modify it or add an extension trait. Let's find the trait file.
- [ ] **Step 2: Add method signatures.**
    ```rust
    fn show_keyboard(&self);
    fn hide_keyboard(&self);
    ```

### Task 2: Implement for macOS

Update the macOS platform implementation to handle the input focus transition.

**Files:**
- Modify: `crates/gpui_macos/src/platform.rs`
- Modify: `crates/gpui_macos/src/window.rs`

- [ ] **Step 1: Implement `show_keyboard` for `MacPlatform`.**
    - This will likely need to iterate through windows or be called on the active window.
- [ ] **Step 2: Implement `hide_keyboard` for `MacPlatform`.**
- [ ] **Step 3: Update `MacWindow` to ensure it can become `FirstResponder`.**
    - Ensure `NSView` / `NSWindow` interactions properly trigger focus.

### Task 3: Update `TextInput` Component

Update the cross-platform UI component to use the new trait methods instead of global `gpui_mobile` functions.

**Files:**
- Modify: `src/components/material/text_input.rs`

- [ ] **Step 1: Refactor `TextInput` to access `cx.platform()`.**
- [ ] **Step 2: Replace calls to `gpui_mobile::show_keyboard_with_type(...)` with `cx.platform().show_keyboard()`.**
- [ ] **Step 3: Remove mobile-specific conditional logic if possible.**

---

### Task 4: Verification

- [ ] **Step 1: Run desktop example.**
    - Run the example app using `cargo run -p example` (adjusting for desktop target).
- [ ] **Step 2: Verify focus.**
    - Test that tapping a `TextInput` on macOS correctly activates the system input context.

---
