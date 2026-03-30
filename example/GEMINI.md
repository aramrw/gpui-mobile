# GPUI Development Lessons Learned

This file documents critical architectural patterns, API changes, and "gotchas" discovered during development to ensure consistency and prevent regressions.

## 1. API Changes & Deprecations

### `refresh()` → `notify()`
In the current version of GPUI (Zed-based), `ViewContext::refresh()` is no longer available.
- **Problem:** Attempting to call `cx.refresh()` results in a compilation error: `no method named refresh found`.
- **Solution:** Use `cx.notify()` to signal that the view's state has changed and needs re-rendering.

## 2. Asynchronous Tasks & Type Inference

### Explicit Type Annotations in `cx.spawn`
GPUI's `cx.spawn` often fails to infer the correct types for its closure parameters, especially in complex views.
- **Pattern:** Always provide explicit type annotations for the closure parameters.
- **Typical Signature:**
  ```rust
  cx.spawn(|this: WeakEntity<MyView>, cx: &mut AsyncApp| {
      let mut cx = cx.clone();
      async move {
          // ... async logic ...
          this.update(&mut cx, |this, cx: &mut Context<'_, MyView>| {
              // ... update state ...
              cx.notify();
          }).ok();
      }
  })
  ```
- **Note:** `AsyncApp` is the expected type for the second parameter of `cx.spawn` on a `ViewContext`. The inner `update` closure should explicitly type `cx` as `&mut Context<'_, MyView>`.

## 3. Window & Context Types

- **`Context<'_, T>`**: The standard context for an entity/view of type `T`.
- **`AsyncApp`**: A thread-safe handle to the application, used inside `spawn`.
- **`AsyncWindowContext`**: A thread-safe handle specifically for a window's context.

## 4. Common "Gotchas"

- **Unused Results:** `this.update(...)` returns a `Result`. Always handle it (e.g., with `.ok()` or `let _ = ...`) to avoid "unused result" warnings.
- **Closure Cloning:** Inside an `async move` block spawned by `cx.spawn`, ensure you clone `cx` if you need to use it multiple times or within nested closures.
