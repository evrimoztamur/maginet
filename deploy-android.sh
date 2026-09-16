#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
if [[ "${1:-}" == --help ]]; then
  echo 'Usage: ./deploy-android.sh [adb-serial]'; echo 'Builds debug APK and release AAB; optional device installs/launches debug APK.'; echo 'Set CONFIGURATION=Release to install a signed release APK. See android/README.md.'; exit 0
fi
export ANDROID_HOME="${ANDROID_HOME:-$HOME/Library/Android/sdk}"
export JAVA_HOME="${JAVA_HOME:-$(/usr/libexec/java_home -v 17 2>/dev/null || true)}"
CONFIGURATION="${CONFIGURATION:-Debug}"
case "$CONFIGURATION" in Debug|Release) ;; *) echo 'CONFIGURATION must be Debug or Release' >&2; exit 1;; esac
if [[ "$CONFIGURATION" == Release ]]; then
  python3 android/scripts/check-release.py
fi
if [[ -z "${WASM_BINDGEN:-}" ]]; then
  WASM_BINDGEN="$PWD/ios/build-tools/wasm-bindgen/bin/wasm-bindgen"
  if [[ "$("$WASM_BINDGEN" --version 2>/dev/null || true)" != 'wasm-bindgen 0.2.91' ]]; then
    cargo install wasm-bindgen-cli --version 0.2.91 --locked --root ios/build-tools/wasm-bindgen
  fi
fi
export WASM_BINDGEN
android/scripts/build-assets.sh
android/gradlew -p android testDebugUnitTest "assemble$CONFIGURATION" bundleRelease
VARIANT=$(echo "$CONFIGURATION" | tr '[:upper:]' '[:lower:]')
APK="$PWD/android/app/build/outputs/apk/$VARIANT/app-$VARIANT.apk"
echo "APK: $APK"
echo "Release bundle: $PWD/android/app/build/outputs/bundle/release/app-release.aab (unsigned unless signing configured)"
if [[ -n "${1:-}" ]]; then
  ADB="$ANDROID_HOME/platform-tools/adb"
  "$ADB" -s "$1" install -r "$APK"
  PACKAGE=zone.evrim.maginet
  [[ "$CONFIGURATION" != Debug ]] || PACKAGE="$PACKAGE.debug"
  "$ADB" -s "$1" shell am start -n "$PACKAGE/zone.evrim.maginet.GameActivity"
fi
