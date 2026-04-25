#!/usr/bin/env bash
set -e

# Scaffolding script for gpui-mobile
# Usage: ./init-mobile.sh <project_dir> <app_name> <bundle_id>

if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <project_dir> <app_name> <bundle_id>"
    echo "Example: $0 . MyApp com.mycompany.myapp"
    exit 1
fi

PROJECT_DIR=$1
APP_NAME=$2
BUNDLE_ID=$3

# Resolve absolute paths
GPUI_MOBILE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
TARGET_DIR="$(cd "$PROJECT_DIR" && pwd)"

echo "🚀 Initializing mobile support for '$APP_NAME' ($BUNDLE_ID) in $TARGET_DIR..."

# 1. Create directories
mkdir -p "$TARGET_DIR/ios"
mkdir -p "$TARGET_DIR/android"

# 2. Copy iOS template
echo "📦 Copying iOS template..."
cp -r "$GPUI_MOBILE_DIR/examples/template/ios/"* "$TARGET_DIR/ios/"

# Customize project.yml
# Note: Using 'sd' as it is available and faster/safer for this
sd "name: GpuiTemplate" "name: $APP_NAME" "$TARGET_DIR/ios/project.yml"
sd "bundleIdPrefix: dev.gpui" "bundleIdPrefix: ${BUNDLE_ID%.*}" "$TARGET_DIR/ios/project.yml"
sd "GpuiTemplate:" "$APP_NAME:" "$TARGET_DIR/ios/project.yml"

# 3. Copy Android template
echo "📦 Copying Android template..."
cp -r "$GPUI_MOBILE_DIR/examples/template/android/"* "$TARGET_DIR/android/"

# Customize Android strings/manifest
sd "GpuiExample" "$APP_NAME" "$TARGET_DIR/android/gradle/app/src/main/res/values/strings.xml"
sd "dev.gpui.mobile" "$BUNDLE_ID" "$TARGET_DIR/android/gradle/app/build.gradle.kts"
sd "dev.gpui.mobile" "$BUNDLE_ID" "$TARGET_DIR/android/gradle/app/src/main/AndroidManifest.xml"

echo "✅ Mobile support initialized!"
echo ""
echo "Next steps:"
echo "1. Ensure you have 'xcodegen' installed (brew install xcodegen)."
echo "2. Run 'cd ios && xcodegen' to generate the Xcode project."
echo "3. Open 'ios/$APP_NAME.xcodeproj' and sign the app in the 'Signing & Capabilities' tab."
echo "4. Build and run!"
