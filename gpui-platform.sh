#!/bin/bash

# gpui-platform.sh
# Unified platform orchestrator for GPUI apps.
# Handles Desktop (macOS), iOS, and Android.

set -e

# --- Configuration ---
GPUI_MOBILE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATES_DIR="$GPUI_MOBILE_DIR/templates"

# --- Defaults ---
PLATFORM=""
RELEASE=false
SIMULATOR=false
VERBOSE=false

# --- Helper: Print Usage ---
usage() {
    echo "Usage: $0 [--macos|--ios|--android] [--release] [--simulator] [--verbose]"
    echo ""
    echo "Options:"
    echo "  --macos        Build/Run for macOS (Desktop) - DEFAULT"
    echo "  --ios          Build/Run for iOS"
    echo "  --android      Build/Run for Android"
    echo "  --release      Build in release mode"
    echo "  --simulator    Build for iOS Simulator"
    echo "  --verbose      Enable verbose output"
    exit 1
}

# --- Parse Arguments ---
while [[ $# -gt 0 ]]; do
    case "$1" in
        --macos) PLATFORM="macos" ;;
        --ios) PLATFORM="ios" ;;
        --android) PLATFORM="android" ;;
        --release) RELEASE=true ;;
        --simulator) SIMULATOR=true ;;
        --verbose) VERBOSE=true ;;
        *) usage ;;
    esac
    shift
done

# --- Interactive Menu if no platform specified ---
if [[ -z "$PLATFORM" ]]; then
    echo "GPUI Platform Orchestrator"
    echo "-------------------------"
    echo "1) macOS (Desktop)"
    echo "2) iOS"
    echo "3) Android"
    echo ""
    read -p "Select platform [1]: " -n 1 -r
    echo ""
    case "$REPLY" in
        2) PLATFORM="ios" ;;
        3) PLATFORM="android" ;;
        *) PLATFORM="macos" ;;
    esac
fi

# --- Detection ---
if [[ ! -f "Cargo.toml" ]]; then
    echo "Error: No Cargo.toml found in current directory."
    exit 1
fi

# --- Desktop (macOS) Logic ---
if [[ "$PLATFORM" == "macos" ]]; then
    echo "Building for macOS..."
    FLAGS=""
    if $RELEASE; then FLAGS="--release"; fi
    RUSTFLAGS="-Awarnings" cargo run $FLAGS
    exit 0
fi

# --- Mobile (iOS/Android) Logic ---

# Extract metadata from Cargo.toml
APP_NAME=$(grep -m 1 "^name =" Cargo.toml | sed 's/name = "\(.*\)"/\1/' | tr '-' '_')
LIB_NAME=$(grep -m 1 "^name =" Cargo.toml -A 10 | grep -A 5 "\[lib\]" | grep "^name =" | sed 's/name = "\(.*\)"/\1/' || echo "$APP_NAME")
if [[ -z "$LIB_NAME" ]]; then LIB_NAME="$APP_NAME"; fi

SAFE_APP_NAME=$(echo "$APP_NAME" | sed 's/[^a-zA-Z0-9]//g')

# Bundle ID (default or from metadata)
RANDOM_ID=$(LC_ALL=C tr -dc 'a-z0-9' < /dev/urandom | head -c 8)
BUNDLE_ID=$(grep "bundle_id =" Cargo.toml | sed -n 's/.*bundle_id = "\(.*\)".*/\1/p')
if [[ -z "$BUNDLE_ID" ]]; then
    BUNDLE_ID="dev.gpui.mobile.$RANDOM_ID"
fi

# --- Pre-flight Checks ---
check_tool() {
    if ! command -v "$1" &> /dev/null; then
        echo "Error: $1 is not installed. $2"
        exit 1
    fi
}

if [[ "$PLATFORM" == "ios" ]]; then
    check_tool xcodegen "Install with 'brew install xcodegen'"
    
    # --- Scaffolding ---
    TARGET_DIR="target/gpui-mobile/$PLATFORM"
    rm -rf "$TARGET_DIR" # Clean old scaffolding to ensure fresh templates
    mkdir -p "$TARGET_DIR"

    echo "Preparing $PLATFORM scaffolding in $TARGET_DIR..."
    cp -r "$TEMPLATES_DIR/ios/"* "$TARGET_DIR/"
    
    # Dynamic Configuration via sed
    sed -i '' "s/GpuiExample/$SAFE_APP_NAME/g" "$TARGET_DIR/project.yml"
    sed -i '' "s/dev.fanni.gpui.mobile.test1/$BUNDLE_ID/g" "$TARGET_DIR/project.yml"
    sed -i '' "s/libgpui_mobile_example.a/lib${LIB_NAME}.a/g" "$TARGET_DIR/project.yml"
    sed -i '' "s|\$(PROJECT_DIR)/../target|\$(PROJECT_DIR)/../../../target|g" "$TARGET_DIR/project.yml"
    sed -i '' "s|RUST_PROJECT_DIR=\"\${PROJECT_DIR}/..\"|RUST_PROJECT_DIR=\"\${PROJECT_DIR}/../../..\"|g" "$TARGET_DIR/project.yml"

    # --- Initial Rust Build ---
    # This prevents the "Build input file not found" error in Xcode
    RUST_TARGET="aarch64-apple-ios"
    if $SIMULATOR; then RUST_TARGET="aarch64-apple-ios-sim"; fi
    
    echo "Performing initial Rust build for $RUST_TARGET..."
    rustup target add "$RUST_TARGET" 2>/dev/null || true
    cargo build --target "$RUST_TARGET" --release # Forced release as per template

    # --- Xcode Generation ---
    echo "Generating Xcode project..."
    cd "$TARGET_DIR" && xcodegen generate > /dev/null && cd - > /dev/null

    echo ""
    read -p "Would you like to open the project in Xcode? (y/N) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        open "$TARGET_DIR/${SAFE_APP_NAME}.xcodeproj"
    else
        echo "Project is available at: $TARGET_DIR/${SAFE_APP_NAME}.xcodeproj"
    fi
fi
