# Extract Dictionary App Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extract the dictionary app from `gpui-mobile/example` to `../yomichan_mobile`.

**Architecture:** Moving the example project to a standalone repository structure while maintaining relative paths to dependencies in `gpui-mobile`.

**Tech Stack:** Rust, GPUI, iOS (XcodeGen), Android (Gradle).

---

### Task 1: Create Directory Structure

**Files:**
- Create: `../yomichan_mobile/src`
- Create: `../yomichan_mobile/ios`
- Create: `../yomichan_mobile/android`
- Create: `../yomichan_mobile/docs`

- [ ] **Step 1: Create directories**

Run: `mkdir -p ../yomichan_mobile/src ../yomichan_mobile/ios ../yomichan_mobile/android ../yomichan_mobile/docs`

### Task 2: Copy Files

**Files:**
- Copy: `example/src/*` -> `../yomichan_mobile/src/`
- Copy: `example/ios/*` -> `../yomichan_mobile/ios/`
- Copy: `example/android/*` -> `../yomichan_mobile/android/`
- Copy: `example/Cargo.toml` -> `../yomichan_mobile/Cargo.toml`
- Copy: `example/GEMINI.md` -> `../yomichan_mobile/GEMINI.md`
- Copy: `ADDING-MONOKAKIDO-HOVER.md` -> `../yomichan_mobile/docs/`
- Copy: `HOVER-IMPLEMENTATION.md` -> `../yomichan_mobile/docs/`
- Copy: `run.sh` -> `../yomichan_mobile/run.sh`

- [ ] **Step 1: Copy source files**
Run: `cp -r example/src/* ../yomichan_mobile/src/`

- [ ] **Step 2: Copy iOS files**
Run: `cp -r example/ios/* ../yomichan_mobile/ios/`

- [ ] **Step 3: Copy Android files**
Run: `cp -r example/android/* ../yomichan_mobile/android/`

- [ ] **Step 4: Copy project files**
Run: `cp example/Cargo.toml ../yomichan_mobile/Cargo.toml`
Run: `cp example/GEMINI.md ../yomichan_mobile/GEMINI.md`
Run: `cp ADDING-MONOKAKIDO-HOVER.md ../yomichan_mobile/docs/`
Run: `cp HOVER-IMPLEMENTATION.md ../yomichan_mobile/docs/`
Run: `cp run.sh ../yomichan_mobile/run.sh`

### Task 3: Update Cargo.toml

**Files:**
- Modify: `../yomichan_mobile/Cargo.toml`

- [ ] **Step 1: Update dependencies and add patches**

Update `gpui-mobile` and `yomichan_rs` paths, and add patches for internal crates.

### Task 4: Update iOS project.yml

**Files:**
- Modify: `../yomichan_mobile/ios/project.yml`

- [ ] **Step 1: Set RUST_PROJECT_DIR**

In `preBuildScripts`, set `RUST_PROJECT_DIR: "${PROJECT_DIR}/.."`
