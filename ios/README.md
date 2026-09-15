# Maginet for iPhone

UIKit / WKWebView wrapper for the bundled Rust game. Minimum iOS 17; landscape left and right; bundle ID `zone.evrim.maginet`. The app starts locally without contacting the game server. Full Game (`zone.evrim.maginet.all`) is a permanent non-consumable unlocking the campaign beyond Tutorial / Basics I–IV and all online modes. Local battles, AI, and the editor remain free.

## Build and install

Requirements: Xcode with an iOS SDK, Rust with `wasm32-unknown-unknown`, and **wasm-bindgen-cli 0.2.91** matching Cargo.lock. XcodeGen is only needed when regenerating the checked-in project.

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.91 --locked --root /tmp/maginet-bindgen
WASM_BINDGEN=/tmp/maginet-bindgen/bin/wasm-bindgen ios/scripts/build-assets.sh
open ios/Maginet.xcodeproj
```

Select your development team in Signing & Capabilities, select an iPhone or simulator, and Run. Signing is automatic. The checked-in project has In-App Purchase enabled. Assets are intentionally ignored and **must be rebuilt after Rust or JavaScript changes**. Audio is embedded in Wasm; the generated folder contains the atlas, module workers, JS, and Wasm. The build uses Cargo.lock, the full catalogue, and the `ios` feature; do not add `demo` to the iOS build.

To regenerate project metadata after editing `project.yml`:

```sh
xcodegen generate --spec ios/project.yml
```

For release, regenerate assets, select a real-device destination and your distribution team, and use Product → Archive. The default Debug Run scheme selects the local StoreKit configuration. Disable that configuration when testing production sandbox products. No release has been published by this implementation.

## Command-line device updates

From the repository root, run `./deploy-ios.sh` for a Release device build, or `./deploy-ios.sh <device-id>` to build, install, and launch on a connected iPhone. Exact device names also work: `./deploy-ios.sh My iPhone`. The script rebuilds game assets and manages the pinned wasm-bindgen CLI. See [the root README](../README.md#update-the-iphone-app) for overrides.

## App icon

The AppIcon asset catalog uses the existing `static/png/appicon.png` artwork, scaled to an opaque 1024×1024 PNG with nearest-neighbor sampling. Transparent icon pixels use a dark backing (`#160e22`). Xcode generates the iPhone icon sizes from that source; `ASSETCATALOG_COMPILER_APPICON_NAME` selects it in both build configurations.

## StoreKit testing

`FullGame.storekit` supplies a local $1.99 non-consumable, independent of App Store Connect. Run Product → Test or:

```sh
xcodebuild -project ios/Maginet.xcodeproj -scheme Maginet \
  -destination 'platform=iOS Simulator,name=iPhone 17 Pro' \
  -derivedDataPath ios/build test DEVELOPMENT_TEAM=YOUR_TEAM_ID
```

Keep signing enabled for simulator tests: the native ownership cache uses Keychain, which rejects unsigned test hosts.

### Testing on a physical iPhone

Local StoreKit testing works on a real iPhone. Select the phone as Xcode’s run destination, then **Product → Scheme → Edit Scheme → Run → Options → StoreKit Configuration → FullGame.storekit**, and launch with **Product → Run**. Buy Full Game in the game, then restore and check that campaign/online access remains unlocked. Keep the same testing environment for purchase and restore.

`deploy-ios.sh` installs and launches through `devicectl`; it does not activate the scheme’s local StoreKit test session, even with `CONFIGURATION=Debug`. To test App Store Connect sandbox purchases instead, choose **None** for StoreKit Configuration and use a Sandbox Apple Account on the device. Local Xcode purchases and sandbox purchases are separate histories.

If Xcode reports a transaction but the game stays locked, inspect the app’s `Store` console category. Purchase and restore now report a verification failure separately from an empty purchase history; Debug builds include the underlying verification reason. Check **FullGame.storekit → Editor** for simulated verification failures. A payment success alone does not mean StoreKit verified the transaction. Unverified transactions remain unfinished and cannot create cached ownership; restore retries verification after the test condition is fixed. If a previously verified purchase exists in the native cache, a temporary empty or unverified response does not remove it.

Tests cover fresh demo access, verified ownership recovery, refund/revocation, cancellation, purchase failure, pending approval, product loading failure while owned, the proxy allowlist, device-resolution WebView rendering, a real Wasm AI module worker, and touch purchase entry points in both landscape orientations, including campaign and editor gates. Native text entry is also exercised. Test transactions never charge money. Test results retain screenshots. StoreKit's current entitlements can propagate asynchronously; tests wait for verified state to converge.

Use Xcode's Debug → StoreKit → Manage Transactions for manual purchase/restore/refund checks. Verify campaign stars before and after unlocking, actual airplane-mode relaunch, audio through background/foreground transitions, keyboard editing, tutorial completion, campaign battles, and Chaos rematches before release. Audible output and subjective play quality require a person with the device.

The multiplayer protocol smoke test launches a disposable local server on an unused loopback port, uses two clients, and cleans up its data:

```sh
python3 ios/scripts/test-server.py
```

This tests server creation/joining, move synchronization, rematches, and missing-lobby errors. It is distinct from production HTTPS proxy acceptance; production online play needs a verified Full Game entitlement and the live service.

## Architecture and trust boundaries

- The HTTP listener binds only `127.0.0.1:18743`. Never change the port casually: it is the origin for persistent WebView storage. No CORS dependency or remote game assets.
- The server serves only bundle files; traversal and non-local Host headers are rejected. `/api` permits only the existing game routes and methods, rejects redirects, forwards to `https://maginet.evrim.zone`, and checks native ownership before forwarding. The proxy uses an ephemeral URLSession, bounded request bodies, and a request timeout.
- Only main-frame script messages from this exact local origin are accepted. Messages request state, purchase, restore, or display a connection error; they cannot set an entitlement. StoreKit verified transactions are authoritative. No web-storage flag grants ownership.
- Product lookup is independent of entitlement lookup. Product/network failures do not clear ownership. Verified permanent ownership is cached in the native, device-only Keychain and restored immediately on launch. Empty StoreKit history responses retain that ownership; verified revocations from transaction updates or the latest-transaction query remove it. Debug, test-host, and Release caches are separate, and Release rejects Xcode ownership records. Purchase completion is applied immediately. Overlapping entitlement refreshes cannot overwrite a newer verified transaction. While StoreKit returns no history, a refund cannot be detected until a verified revocation becomes available.
- Rendering retains the 272-unit height and fixed interface layout, with game and interface layers rasterized and composited at 272 pixels high, then enlarged once to device resolution with nearest-neighbor sampling. All three buffers resize together, keeping their respective logical and display resolutions. Safe horizontal insets constrain the interface origin, shared with touch conversion. Extra horizontal space allows particle overflow. Browser builds retain their existing sizing. The page and native shell share the web canvas background (#002a2a); web selection and touch callouts are disabled. The full campaign map remains browsable in demo mode, with the purchase action only at Patterns I.
- Canvas text actions use a native UIKit code-entry sheet so keyboard activation does not depend on a JavaScript user-gesture token. Done and Cancel return to the existing game input flow.
- Background events suspend audio, stop game ticks, terminate worker jobs, and dismiss input. Returning resumes audio and safely consumes the cancelled AI result. Very short iOS taps are queued until the next game frame; compatibility mouse events are ignored on iOS.

## App Store Connect setup

Create the explicit App ID and app record for `zone.evrim.maginet`. [Apple enables In-App Purchase by default for explicit App IDs](https://developer.apple.com/help/account/identifiers/register-an-app-id); no extra service entitlement is needed.

Create **Full Game**, type **Non-Consumable**, ID **`zone.evrim.maginet.all`**. Describe the permanent campaign and online unlock. Target **US$1.99** and **€1.99** with storefront overrides; review Apple's generated local prices elsewhere. The app always displays `Product.displayPrice`, never a hard-coded checkout price. Complete agreements, tax/banking, localization, review screenshot, availability, and review notes in App Store Connect. Submit the product with the first app version. Test the sandbox product on a signed installation before release.

StoreKit documentation: [verified current entitlements](https://developer.apple.com/documentation/storekit/transaction/currententitlements).

## Device results

See [VERIFICATION.md](VERIFICATION.md) for the actual runs and outstanding acceptance checks. Simulator success is recorded separately from physical-device success.
