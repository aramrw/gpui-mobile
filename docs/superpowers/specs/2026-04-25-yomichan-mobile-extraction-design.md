# Design Spec: Yomichan Mobile Extraction

Extract the "Yomichan" dictionary application from the `gpui-mobile` framework fork into a standalone repository (`yomichan_mobile`) to decouple the application logic from the platform framework.

## 1. Goal
- **`gpui-mobile`**: Becomes a dedicated library/framework repository for iOS/macOS/Android GPUI support.
- **`yomichan_mobile`**: Becomes a standalone application crate that depends on `gpui-mobile` and `yomichan_rs`.
- **Stability**: Lock all dependencies to specific git revisions to prevent regressions from upstream Zed changes.
- **Persistence**: Maintain existing Xcode build phases, signing, and optimization flags (`-Awarnings`, forced `--release`).

## 2. Repository Structure
The extraction will create a sibling repository structure in the user's workspace:
```text
/Users/aramsamifanni/Programming/yomichanrs/
├── gpui-mobile/      (The Framework)
├── yomichan_mobile/  (The App - NEW)
└── yomichan_rs/      (The Engine)
```

## 3. Migration Plan (Phased)

### Phase 1: Application Extraction & Verification
1.  **Create `yomichan_mobile`**: Create the new directory at `../yomichan_mobile`.
2.  **File Move**: Copy (not move yet, to ensure safety) the following:
    - `example/src/` -> `yomichan_mobile/src/`
    - `example/ios/` -> `yomichan_mobile/ios/`
    - `example/android/` -> `yomichan_mobile/android/`
    - `example/Cargo.toml` -> `yomichan_mobile/Cargo.toml`
    - `example/GEMINI.md` -> `yomichan_mobile/GEMINI.md`
    - Documents (`ADDING-MONOKAKIDO-HOVER.md`, `HOVER-IMPLEMENTATION.md`) -> `yomichan_mobile/docs/`
3.  **Dependency Alignment**:
    - Update `yomichan_mobile/Cargo.toml` to point to `../gpui-mobile` and `../yomichan_rs`.
    - Ensure `[patch]` sections correctly reference sub-crates in `../gpui-mobile`.
4.  **Verification**: 
    - Compile for macOS.
    - Compile for iOS via Xcode.
    - **GATE**: Ensure both platforms work perfectly before proceeding.

### Phase 2: Framework Refactoring
1.  **Cleanup `gpui-mobile`**: Once verification passes, delete the `example/` directory and app-specific docs from the framework repo.
2.  **Internal Refactor**: Adjust `gpui-mobile` to be a cleaner library crate (e.g., ensure `lib.rs` exports what's needed without assuming an internal example context).

### Phase 3: New Framework Example
1.  **Template Creation**: Add a minimal, generic "Hello World" example to `gpui-mobile/example`.
2.  **Documentation**: Update the framework `README.md` to reflect how to use it as a library.

### 3.3 iOS Build System
- **`ios/project.yml`**:
    - Update `RUST_PROJECT_DIR` to `"."` or `".."` as appropriate for the new root.
    - Preserve `preBuildScripts` including `RUSTFLAGS="-Awarnings"` and forced `CARGO_FLAGS="--release"`.
    - Retain the custom `PRODUCT_BUNDLE_IDENTIFIER` and `DEVELOPMENT_TEAM` to ensure signing remains working in the GUI.

## 4. Success Criteria
1.  `yomichan_mobile` builds successfully in Xcode via the "Build Rust Static Library" phase.
2.  The app runs on iOS and macOS with all dictionary logic (Yomichan init, DB, Hover) intact.
3.  `gpui-mobile` contains no dictionary-specific code.
4.  `cargo update` in the app does not break the build due to locked revisions.
