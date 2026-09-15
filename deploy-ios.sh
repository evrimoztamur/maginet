#!/usr/bin/env sh
set -eu

# ./deploy-ios.sh                            # Release build only
# ./deploy-ios.sh YOUR_DEVICE_ID            # Build, install, and launch
# ./deploy-ios.sh My iPhone                 # An exact device name also works
# Overrides: CONFIGURATION, DEVELOPMENT_TEAM, WASM_BINDGEN.

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    echo "Usage: $0 [device UDID, CoreDevice identifier, or exact device name]"
    echo "No device: build only. With a device: build, install, and launch."
    echo "Default: CONFIGURATION=Release. Set DEVELOPMENT_TEAM or select a team in Xcode."
    exit 0
fi

REPO_ROOT="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
cd "$REPO_ROOT"

PROJECT="ios/Maginet.xcodeproj"
SCHEME="Maginet"
BUNDLE_ID="zone.evrim.maginet"
CONFIGURATION="${CONFIGURATION:-Release}"
DEVELOPMENT_TEAM="${DEVELOPMENT_TEAM:-}"
DERIVED="$REPO_ROOT/ios/build-device"
DEVICE="$*"
DESTINATION="generic/platform=iOS"
UDID=""

case "$CONFIGURATION" in
    Debug|Release) ;;
    *) echo "Error: CONFIGURATION must be Debug or Release." >&2; exit 1 ;;
esac

if [ -n "$DEVICE" ]; then
    DEVICE_JSON="$(mktemp -t maginet-ios-devices)"
    trap 'rm -f "$DEVICE_JSON"' EXIT HUP INT TERM
    xcrun devicectl list devices --json-output "$DEVICE_JSON" >/dev/null
    UDID="$(python3 - "$DEVICE_JSON" "$DEVICE" <<'PY'
import json
import sys

with open(sys.argv[1]) as source:
    devices = json.load(source)["result"]["devices"]
devices = [d for d in devices if d.get("hardwareProperties", {}).get("platform") == "iOS"
           and d["hardwareProperties"].get("reality") == "physical"]
query = sys.argv[2].casefold()
matches = [d for d in devices if query in {
    d.get("identifier", "").casefold(),
    d["hardwareProperties"].get("udid", "").casefold(),
    d.get("deviceProperties", {}).get("name", "").casefold(),
}]
if len(matches) != 1:
    reason = "No iOS device matches" if not matches else "Multiple devices match"
    print(f"Error: {reason} {sys.argv[2]!r}. Connect and unlock your phone, or use its UDID.", file=sys.stderr)
    for device in devices:
        print(f"  {device.get('deviceProperties', {}).get('name', '(unnamed)')}: "
              f"{device['hardwareProperties'].get('udid', '(no UDID)')}", file=sys.stderr)
    sys.exit(1)
print(matches[0]["hardwareProperties"]["udid"])
PY
    )"
    DESTINATION="platform=iOS,id=$UDID"
fi

# Game assets are built separately from Xcode and must match Cargo.lock.
if [ -z "${WASM_BINDGEN:-}" ]; then
    if [ "$(wasm-bindgen --version 2>/dev/null || true)" = "wasm-bindgen 0.2.91" ]; then
        WASM_BINDGEN="$(command -v wasm-bindgen)"
    else
        BINDGEN_ROOT="$REPO_ROOT/ios/build-tools/wasm-bindgen"
        WASM_BINDGEN="$BINDGEN_ROOT/bin/wasm-bindgen"
        if [ "$("$WASM_BINDGEN" --version 2>/dev/null || true)" != "wasm-bindgen 0.2.91" ]; then
            cargo install wasm-bindgen-cli --version 0.2.91 --locked --root "$BINDGEN_ROOT"
        fi
    fi
fi
export WASM_BINDGEN
echo "Rebuilding bundled iOS game assets..."
ios/scripts/build-assets.sh

echo "Building $SCHEME ($CONFIGURATION) for ${DEVICE:-generic iOS device}..."
set -- -project "$PROJECT" -scheme "$SCHEME" \
    -configuration "$CONFIGURATION" -destination "$DESTINATION" \
    -derivedDataPath "$DERIVED" -allowProvisioningUpdates
if [ -n "$DEVELOPMENT_TEAM" ]; then
    set -- "$@" "DEVELOPMENT_TEAM=$DEVELOPMENT_TEAM"
fi
xcodebuild "$@" build

APP="$DERIVED/Build/Products/$CONFIGURATION-iphoneos/$SCHEME.app"
if [ ! -d "$APP" ]; then
    echo "Error: built app not found at $APP" >&2
    exit 1
fi

if [ -z "$UDID" ]; then
    echo "Build complete: $APP"
    exit 0
fi

echo "Installing on $DEVICE ($UDID)..."
xcrun devicectl device install app --device "$UDID" "$APP"
echo "Launching $BUNDLE_ID..."
xcrun devicectl device process launch --terminate-existing --device "$UDID" "$BUNDLE_ID"
echo "Deployed $SCHEME to $DEVICE."
