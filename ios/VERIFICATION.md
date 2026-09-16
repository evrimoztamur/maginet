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

## Physical iPhone (iPhone 15)

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
  -destination 'platform=iOS,id=YOUR_DEVICE_ID' \
  -derivedDataPath ios/build-device -allowProvisioningUpdates \
  DEVELOPMENT_TEAM=YOUR_TEAM_ID test \
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
- Physical iPhone: all 3 WebView/UI tests passed with the new build, including campaign-boundary navigation, editor keyboard, online purchase entry, and both orientations. Captured screen images show clean board edges. The updated app was relaunched after testing.
- iPhone 16e simulator: all 3 selected WebView/UI tests passed. After correcting the background to the canvas container color, the targeted WebView test passed again on iPhone 17 Pro and the physical iPhone, verifying rgb(0, 42, 42).

## Mage drag-and-drop (2026-09-15)

- Added mouse/touch dragging to the shared battle/tutorial controller, with a 4-logical-pixel threshold, preserved grab offset, jump-height float, following shadow, and an eight-frame local landing. Invalid drops preserve selection and history. Network turn messages remain unchanged.
- Rust workspace: 55 tests passed. The client has 30 tests, also passing with the `ios` feature. New tests cover gesture retention, cancellation, touch identity, threshold/bob, landing continuity, and one-shot pickup/impact sequencing. The coordinate round-trip test now tests canvas rotation only on web; iOS rotates the native landscape view.
- `scripts/check-drag.cjs`: Chrome mouse and synthetic touch checks cover tutorial and local battles, exact shadow tracking, lift, one sprite, invalid/outside/UI drops, landing continuity, undo and taps, selecting another mage, touch confirmation, multiple fingers, matching cancellation, short gestures, duplicate mouse suppression, backgrounding, resize, and lost focus. It holds AI replies for deterministic position assertions; the native runs exercise the tutorial's real AI.
- `scripts/check-drag-online.cjs`: two controlled browser clients use a disposable local server. Checks verify no history or request on invalid drops, exactly one ordinary `Turn` on valid drops, a held drag surviving empty polling responses, cancellation on a remote turn, and cancellation on replacement history after rematch. No production service or persistent match data is used.
- iPhone 17 Pro and iPhone 16e simulators, plus the physical iPhone: `testTutorialDraggingInBothOrientations` passed, exercising outside-board return, valid dragging, and touch confirmation afterward in both landscape orientations. Screenshots are retained in the Xcode results. Physical-device landing screenshots were inspected in both orientations. The WebView startup/worker/raster check also passed on iPhone 17 Pro and the physical iPhone.
- Release iOS Wasm assets rebuilt with wasm-bindgen 0.2.91. No commits or publication.

Browser regression scripts require `playwright-core` and a Chrome executable (`CHROME_PATH` can override the default). Serve a non-iOS, non-deploy web build from the repository and set `DRAG_URL` if needed (default: `http://127.0.0.1:8789/html/game.html`). The online script builds the local Rust server and removes its temporary directory when finished.

```sh
node scripts/check-drag.cjs
node scripts/check-drag-online.cjs
```

Native drag test selector:

```sh
xcodebuild -project ios/Maginet.xcodeproj -scheme Maginet \
  -destination 'platform=iOS Simulator,name=iPhone 17 Pro' \
  -derivedDataPath ios/build test \
  -only-testing:MaginetUITests/GameUITests/testTutorialDraggingInBothOrientations
```

## Purchase verification diagnostics

- iPhone 17 Pro simulator / iOS 26.2: five Store tests pass, including a direct `Product.purchase()` check that ownership is published immediately and survives foreground refresh and restore.
- A simulated invalid signature reproduces a recorded purchase that cannot unlock. Purchase and restore now report the verification error; clearing the simulated failure allows restore to recover ownership. Unverified transactions never grant access.
- Physical iPhone 15 / iOS 26.6.1: reproduced the purchase failure under an active Xcode StoreKit test session. The purchase returns a verified non-consumable and publishes ownership immediately, but a subsequent refresh returns an empty entitlement sequence and clears ownership. `Transaction.latest(for:)`, `Transaction.all`, and the product-specific entitlement query also return no transaction, while `SKTestSession.allTransactions()` contains the purchase. Restore sometimes delivers a verified update, but recovery is inconsistent.
- The initial device Store suite failed (11 assertions across four purchase tests). The user also reproduced the brief unlock followed by relocking in an interactive Xcode Run.
- Fixed by caching verified permanent ownership in the native, device-only Keychain. An empty StoreKit result no longer removes a verified purchase. Verified refunds remove both live and cached ownership. Release and Debug records are isolated; unit tests use separate accounts. Settings derives its button from live ownership each frame, showing a disabled “Full Version!” after purchase.
- Physical iPhone 15 / iOS 26.6.1: all seven native tests pass after the fix. Coverage includes immediate purchase notification, foreground refresh, restore, ownership loaded synchronously from a new Keychain-cache instance, refund invalidation, unverified-purchase rejection and recovery, bundled rendering/worker execution, and ownership propagation to the live WebView through foreground refresh and reload.
- StoreKit history itself still returns empty on this phone. Cached verified ownership prevents relocking; restoring on a clean installation without a native record still depends on StoreKit delivering a verified transaction. Refunds take effect when StoreKit supplies verified revocation information.

- Final purchase-fix validation: all seven native tests also pass on the iPhone 17 Pro / iOS 26.2 simulator with signing enabled. An unsigned simulator run failed the two Keychain recovery assertions with `errSecMissingEntitlement`; the documented test command now keeps signing enabled. The Release device build succeeds, and the rebuilt app is running on the connected iPhone through Xcode.

## Upload-only update — 16 September 2026

- Archived the latest shared mobile changes: Settings layout, native reviewer access, face-to-face local mage visuals, and winner/Stalemate banners. All 11 bundled game assets match the rebuilt iOS assets; archive signature verified.
- The full iOS simulator native/Web/UI suite passed (`ios/build-review-test/Logs/Test/Test-Maginet-2026.09.16_11-43-31-+0200.xcresult`).
- Uploaded version **1.0, build 7** through the existing Xcode App Store Connect account. Xcode's automatic version management incremented the prior uploaded build 6. Upload completed successfully; Apple processing was pending at upload completion. Logs/archive: `ios/build-final-update/`.
- User explicitly requested upload only. The existing version 1.0 submission remains Waiting for Review; no submission was withdrawn, no attached build changed, and no new App Review submission was made. Build 7 is for the user to select later.
