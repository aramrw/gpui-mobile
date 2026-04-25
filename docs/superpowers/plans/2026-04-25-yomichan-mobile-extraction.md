# Yomichan Mobile Extraction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extract the Yomichan dictionary app into a standalone repository (`yomichan_mobile`) and refactor `gpui-mobile` into a clean library crate with its own examples.

**Architecture:** 
1.  Standalone `yomichan_mobile` crate at `../yomichan_mobile`.
2.  `gpui-mobile` as a path dependency (`{ path = "../gpui-mobile" }`).
3.  Locked git revisions for all GPUI and Zed dependencies to ensure permanent stability.
4.  Preserve existing iOS build logic (xcodegen, signing, optimization flags).

**Tech Stack:** Rust, GPUI, Xcode (UIKit/Metal), Android NDK (Vulkan).

---

### Phase 1: Application Extraction & Verification

#### Task 1: Initialize New Repository
- [ ] **Step 1: Create the directory structure**
```bash
mkdir -p ../yomichan_mobile/src
mkdir -p ../yomichan_mobile/ios
mkdir -p ../yomichan_mobile/android
mkdir -p ../yomichan_mobile/docs
```

- [ ] **Step 2: Copy core application files**
```bash
cp -r example/src/* ../yomichan_mobile/src/
cp -r example/ios/* ../yomichan_mobile/ios/
cp -r example/android/* ../yomichan_mobile/android/
cp example/Cargo.toml ../yomichan_mobile/Cargo.toml
cp example/GEMINI.md ../yomichan_mobile/GEMINI.md
cp ADDING-MONOKAKIDO-HOVER.md ../yomichan_mobile/docs/
cp HOVER-IMPLEMENTATION.md ../yomichan_mobile/docs/
cp run.sh ../yomichan_mobile/run.sh
```

- [ ] **Step 3: Update `yomichan_mobile/Cargo.toml`**
Modify `../yomichan_mobile/Cargo.toml` to point to the correct paths and lock revisions.
```toml
[package]
name = "yomichan-mobile"
version = "0.1.0"
edition = "2021"

[dependencies]
gpui-mobile = { path = "../gpui-mobile", features = ["file_selector"] }
gpui = { git = "https://github.com/zed-industries/zed", rev = "4dd42a0f77b11d0bed2a072919bcd9180b9a577c", package = "gpui", default-features = false }
yomichan_rs = { path = "../yomichan_rs", features = ["anki"] }
# ... (keep other dependencies exactly as they were in example/Cargo.toml)

[patch."https://github.com/zed-industries/zed"]
gpui_util = { path = "../gpui-mobile/crates/gpui_util" }
collections = { path = "../gpui-mobile/crates/collections" }
gpui_macos = { path = "../gpui-mobile/crates/gpui_macos" }
```

- [ ] **Step 4: Update `yomichan_mobile/ios/project.yml`**
Update the `RUST_PROJECT_DIR` in `preBuildScripts` to `..` since the project root is now the parent of `ios/`.
```yaml
preBuildScripts:
  - name: "Build Rust Static Library"
    script: |
      # ... (existing setup)
      RUST_PROJECT_DIR="${PROJECT_DIR}/.."
      cd "${RUST_PROJECT_DIR}"
      # ... (existing build commands)
```

- [ ] **Step 5: Verify macOS Build**
Run: `cd ../yomichan_mobile && cargo check`
Expected: Success

- [ ] **Step 6: Verify iOS Build in Xcode**
Open the generated project in Xcode and run the build.
Expected: Success

---

### Phase 2: Framework Cleanup & Library Refactor

#### Task 2: Remove App Logic from `gpui-mobile`
- [ ] **Step 1: Delete extracted files**
```bash
rm -rf example/
rm ADDING-MONOKAKIDO-HOVER.md
rm HOVER-IMPLEMENTATION.md
rm run.sh
```

- [ ] **Step 2: Commit cleanup**
```bash
git add .
git commit -m "refactor: extract dictionary app to yomichan_mobile"
```

#### Task 3: Refactor `gpui-mobile` as a Library
- [ ] **Step 1: Update root `Cargo.toml`**
Ensure it's a library crate only, removing workspace members if they exist.
```toml
[package]
name = "gpui-mobile"
# ... existing metadata
```

---

### Phase 3: Framework Examples

#### Task 4: Create Minimal Example Template
- [ ] **Step 1: Initialize `examples/hello_world`**
Create a standard Rust example in `gpui-mobile/examples/hello_world.rs`.
```rust
use gpui::{prelude::*, App, WindowOptions};

fn main() {
    App::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| HelloWorld)
        }).unwrap();
    });
}

struct HelloWorld;
impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().flex().items_center().justify_center().size_full().bg(gpui::white()).child("Hello, GPUI Mobile!")
    }
}
```

- [ ] **Step 2: Update README**
Document how to use `gpui-mobile` as a dependency and how to run examples.
```markdown
## Usage
Add to your `Cargo.toml`:
```toml
gpui-mobile = { git = "https://github.com/your-username/gpui-mobile" }
```

## Examples
```bash
cargo run --example hello_world
```
```

- [ ] **Step 3: Final Verification**
Run: `cargo run --example hello_world` (on macOS)
Expected: A window showing "Hello, GPUI Mobile!"
