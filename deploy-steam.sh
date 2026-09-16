#!/usr/bin/env bash
set -euo pipefail

# Build both Steam apps from this checkout; default branches are activated in
# Steamworks after uploading (SteamCMD's SetLive only supports beta branches).
BUILD_ONLY=0
case "${1:-}" in
  "") ;;
  --build-only) BUILD_ONLY=1 ;;
  --help|-h)
    echo "Usage: STEAM_USER=youruser $0 [--build-only]"
    echo "Builds Windows/macOS packages for Maginet and Maginet Demo, then uploads both."
    echo "--build-only builds packages and VDFs without logging in or uploading."
    exit 0
    ;;
  *) echo "Usage: STEAM_USER=youruser $0 [--build-only]" >&2; exit 1 ;;
esac
[[ $# -le 1 ]] || { echo "Too many arguments." >&2; exit 1; }

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$REPO_ROOT"
TOOLS="$REPO_ROOT/platforms/steam"
STEAMCMD="${STEAMCMD:-$TOOLS/sdk/tools/ContentBuilder/builder_osx/steamcmd.sh}"
BUILD_ROOT="${STEAM_BUILD_ROOT:-$REPO_ROOT/target/steam}"

if [[ "$BUILD_ONLY" == 0 ]]; then
  [[ -n "${STEAM_USER:-}" ]] || { echo "Set STEAM_USER to your Steam builder account." >&2; exit 1; }
  command -v "$STEAMCMD" >/dev/null || {
    echo "SteamCMD not found. Set STEAMCMD to its executable or install the SDK in platforms/steam/sdk." >&2
    exit 1
  }
  # Resolve before changing directory or giving SteamCMD a separate home.
  STEAMCMD="$(command -v "$STEAMCMD")"
  if [[ "$STEAMCMD" != /* ]]; then STEAMCMD="$REPO_ROOT/$STEAMCMD"; fi
fi
for tool in cargo wasm-pack node npm rsync lipo; do
  command -v "$tool" >/dev/null || { echo "Required tool not found: $tool" >&2; exit 1; }
done

mkdir -p "$BUILD_ROOT"
BUILD_ROOT="$(cd "$BUILD_ROOT" && pwd)"
# VDF values are quoted; reject paths requiring VDF escaping.
case "$BUILD_ROOT" in
  *\"*|*\\*|*$'\n'*|*$'\r'*) echo "STEAM_BUILD_ROOT cannot contain quotes, backslashes, or newlines." >&2; exit 1 ;;
esac
CONTENT="$BUILD_ROOT/content"
SCRIPTS="$BUILD_ROOT/scripts"
OUTPUT="$BUILD_ROOT/output"
mkdir -p "$CONTENT" "$SCRIPTS" "$OUTPUT"

npm ci --prefix "$TOOLS" --include=dev --no-audit --no-fund
ELECTRON_VERSION="$(node -p "require('./platforms/steam/package.json').config.electronVersion")"
DESCRIPTION="maginet $(git rev-parse --short HEAD) $(date -u '+%Y-%m-%d %H:%M UTC')"
STAGING="$(mktemp -d "$BUILD_ROOT/staging.XXXXXX")"
trap 'rm -rf "$STAGING"' EXIT

build_app() {
  local variant="$1" name="$2" appid="$3" windows_depot="$4" macos_depot="$5" features="$6"
  local source="$STAGING/$variant" packages="$STAGING/packages-$variant"
  echo "Building $name (AppID $appid)..."
  mkdir -p "$source/static/js"
  cp "$TOOLS/app/main.cjs" "$source/"
  # Keep the established product names, executable names and save directories.
  node -e 'const fs = require("node:fs"); fs.writeFileSync(process.argv[1], JSON.stringify({name: process.argv[2], version: "0.1.0", main: "main.cjs"}, null, 2));' \
    "$source/package.json" "$name"
  cp html/itch.html "$source/index.html"
  # Never copy a previous Wasm build or source-only artwork into either app.
  rsync -a --exclude='/js/pkg/' --exclude='.DS_Store' --exclude='*.aseprite' \
    --exclude='*.psd' --exclude='*.blend' static/ "$source/static/"
  wasm-pack build --release --target web --out-dir "$source/static/js/pkg" \
    --out-name maginet -- --locked --no-default-features --features "$features"

  "$TOOLS/node_modules/.bin/electron-packager" "$source" "$name" \
    --electron-version="$ELECTRON_VERSION" --platform=win32 --arch=x64 \
    --icon="$REPO_ROOT/static/png/appicon" --out="$packages" --overwrite
  "$TOOLS/node_modules/.bin/electron-packager" "$source" "$name" \
    --electron-version="$ELECTRON_VERSION" --platform=darwin --arch=universal \
    --icon="$REPO_ROOT/static/png/appicon" --out="$packages" --overwrite

  [[ -f "$packages/$name-win32-x64/$name.exe" ]] || { echo "Missing Windows executable for $name." >&2; exit 1; }
  [[ -x "$packages/$name-darwin-universal/$name.app/Contents/MacOS/$name" ]] || { echo "Missing macOS executable for $name." >&2; exit 1; }
  rm -rf "$CONTENT/$variant"
  mkdir -p "$CONTENT/$variant" "$OUTPUT/$variant"
  mv "$packages/$name-win32-x64" "$CONTENT/$variant/windows"
  mv "$packages/$name-darwin-universal" "$CONTENT/$variant/macos"

  cat > "$SCRIPTS/app_build_$variant.vdf" <<EOF
"AppBuild"
{
  "AppID" "$appid"
  "Desc" "$DESCRIPTION ($variant)"
  "BuildOutput" "$OUTPUT/$variant/"
  "ContentRoot" "$CONTENT/$variant/"
  "Depots"
  {
    "$windows_depot"
    {
      "FileMapping"
      {
        "LocalPath" "windows/*"
        "DepotPath" "."
        "recursive" "1"
      }
    }
    "$macos_depot"
    {
      "FileMapping"
      {
        "LocalPath" "macos/*"
        "DepotPath" "."
        "recursive" "1"
      }
    }
  }
}
EOF
}

build_app main "Maginet"      2441960 2441961 2441962 deploy
build_app demo "Maginet Demo" 2529900 2529901 2529902 deploy,demo

echo "Packages: $CONTENT"
echo "SteamPipe scripts: $SCRIPTS"
if [[ "$BUILD_ONLY" == 1 ]]; then
  echo "Build complete; nothing uploaded."
  exit 0
fi

# One login, two app builds, four depots. All packaging must succeed before
# uploading either app. Steam handles each app separately, not atomically.
# Keep builder credentials separate from the desktop Steam client.
STEAM_HOME="${STEAM_HOME:-$TOOLS/.steamhome}"
mkdir -p "$STEAM_HOME"
STEAM_HOME="$(cd "$STEAM_HOME" && pwd)"
echo "Uploading both apps; Steam may prompt for your password and Steam Guard."
UPLOAD_LOG="$OUTPUT/upload.log"
env HOME="$STEAM_HOME" "$STEAMCMD" +@ShutdownOnFailedCommand 1 \
  +login "$STEAM_USER" \
  +run_app_build "$SCRIPTS/app_build_main.vdf" \
  +run_app_build "$SCRIPTS/app_build_demo.vdf" +quit | tee "$UPLOAD_LOG"
# SteamCMD can exit successfully even when an individual app build fails.
for appid in 2441960 2529900; do
  if ! grep -Eq "Successfully finished AppID $appid build" "$UPLOAD_LOG"; then
    echo "Upload not confirmed for AppID $appid. Check $UPLOAD_LOG and $OUTPUT before publishing." >&2
    exit 1
  fi
done
echo "Both apps uploaded. Set each new build live on its default branch in Steamworks:"
echo "  Maginet:      https://partner.steamgames.com/apps/builds/2441960"
echo "  Maginet Demo: https://partner.steamgames.com/apps/builds/2529900"
