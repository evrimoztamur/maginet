# Verification — 2026-09-15

## Passed

- Rust workspace tests: 50 passed, including campaign progression, hidden portal totals, AI search, and simulation tests.
- Web demo tests: 22 passed, including the new short-touch regression test.
- Native-host Rust check with `ios`; release Wasm build with `ios` and the pinned wasm-bindgen 0.2.91 CLI.
- Xcode 26.3 simulator builds and signed arm64 device builds; deployment target iOS 17.
- iPhone 17 Pro / iOS 26.2: StoreKit tests for fresh demo, verified purchase, restore, reopened ownership, revocation, pending/approved purchase, cancellation, purchase failure, and product-loading failure while owned. WebView test loads bundled Wasm and atlas, checks device-pixel resolution, and runs a real module worker against a valid game snapshot. Background/foreground events preserve the canvas and stored progress marker.
- iPhone 17 Pro: automated touch tests for Settings and Online purchase sheets in both landscape orientations, campaign demo-boundary unlock, editor preview online gate, and native level-code keyboard entry/dismissal. Screenshots are retained in the Xcode test result bundles.
- iPhone 16e / iOS 26.2: all six native tests passed with the final safe-bottom layout, including purchase scenarios, the WebView worker, both landscape orientations, campaign/editor gates, and native keyboard entry.
- Two controlled local HTTP clients: sessions, lobby creation, joining, move synchronization, rematches, and recoverable missing-lobby responses passed using `scripts/test-server.py`. The server and match records were confined to a temporary directory.
- The existing Maginet icon is now included as an opaque 1024×1024 AppIcon source. Device and simulator asset-catalog builds pass; the installed iPhone bundle selects AppIcon.
- `git diff --check` passed. Generated assets, build output, signing user data, and overview PNGs are ignored. No commits or publication.

## Physical iPhone — Monolith II (iPhone 15)

**All three selected physical-device tests passed on iOS 26.6.1.** The updated app, including the existing Maginet icon, is installed and a successful standalone launch was confirmed.

The first physical run exposed a bundle-path normalization bug: the bundle root used `/private/var`, while the resolved file used `/var`. The local HTTP server rejected its own bundled content. Normalizing the root before the containment check fixed loading. The WebView test now records startup diagnostics on failure.

Verified on the phone:

- Bundled canvas startup, native pixel resolution, and an actual Wasm AI worker computation.
- Canvas identity and a stored progress marker survive background/foreground events.
- Settings and Online purchase entry points in both landscape orientations.
- Campaign demo-boundary and editor-preview online purchase gates.
- Native level-code keyboard entry and dismissal.

Rerun these checks with:

```sh
xcodebuild -project ios/Maginet.xcodeproj -scheme Maginet \
  -destination 'platform=iOS,id=00008120-000E0C122272201E' \
  -derivedDataPath ios/build-device -allowProvisioningUpdates \
  DEVELOPMENT_TEAM=CCHS7A8LHS test \
  -only-testing:MaginetTests/WebTests -only-testing:MaginetUITests
```

Those selected tests do not buy products. Purchase simulations are confined to the simulator StoreKit tests.

## Remaining release acceptance

- Physical-device tutorial completion, campaign battles, editor play, Chaos rematches, audible audio activation/suspension, and genuine airplane-mode ownership after relaunch.
- A human check of board proportions, particle overflow and all edge controls on the physical phone, including both landscape orientations and keyboard return.
- Full campaign-star retention and same-screen campaign expansion during a purchase in an interactive session. Storage and native entitlement tests pass separately; these do not replace the combined interactive check.
- End-to-end production HTTPS multiplayer through the entitled native proxy. The protocol smoke test and native route allowlist tests are separate checks, not a live production proxy match.
- App Store Connect app/product creation, storefront prices (€1.99 and US$1.99 overrides), sandbox purchase validation, screenshots, and release review metadata. The local StoreKit configuration is included; no App Store Connect product was created, and no real-money purchase or publishing was initiated.

Build/test output contains existing Rust unused-code warnings and Apple's simulator WebKit/StoreKit diagnostics. The final physical-device run passed. Audible audio, complete gameplay sessions, and the other release checks above remain separate acceptance work.

## Campaign and raster fixes (2026-09-15)

- Runtime demo now retains the full campaign catalogue and existing secret-reveal rules. Only Patterns I offers the campaign purchase action; subsequent paid entries stay locked. Ownership changes preserve the map position and saved stars.
- Identified direct fractional sprite rendering into device-resolution game/interface buffers. Both layers now rasterize and composite at 272 pixels high before one nearest-neighbor display enlargement. Touch and final display scaling remain uniform.
- Page and native shell backgrounds now match the web canvas container (#002a2a). Web content disables selection and touch callouts; native code entry remains usable.
- Workspace tests: 51 passed. Compile-time demo tests: 23 passed. iOS Wasm assets rebuilt successfully.
- iPhone 17 Pro simulator: all 6 native, StoreKit, WebView and UI tests passed. WebView regression checks the source raster dimensions, identity transform, disabled smoothing, selection/callout styles, and background.
- Monolith II: all 3 WebView/UI tests passed with the new build, including campaign-boundary navigation, editor keyboard, online purchase entry, and both orientations. Captured screen images show clean board edges. The updated app was relaunched after testing.
- iPhone 16e simulator: all 3 selected WebView/UI tests passed. After correcting the background to the canvas container color, the targeted WebView test passed again on iPhone 17 Pro and Monolith II, verifying rgb(0, 42, 42).
