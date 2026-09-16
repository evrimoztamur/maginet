# Android verification — 2026-09-16

Status: **release-prepared implementation; not verified for publication**. Signed version 1 (1.0) was uploaded to Play Console and saved in the Alpha draft on 2026-09-16. No rollout or publication has completed.

## Automated results

| Check | Result |
| --- | --- |
| `cargo test --workspace --locked` | PASS: 76 tests, no failures |
| Browser Wasm target check | PASS |
| iOS Wasm target check and pinned release asset rebuild | PASS |
| Android Wasm target check and pinned release asset rebuild | PASS |
| `testDebugUnitTest` | PASS: four suites of assertions covering signature/package/product/state rejection, ownership query races/removal/offline retention/restore, acknowledgement retry/duplicate suppression/closure, and proxy URL/route policy |
| `assembleDebug`, `bundleRelease` | PASS: installable debug APK; subsequent release build produced signed APK/AAB with the actual Console billing public key |
| `lintDebug`, release vital lint | PASS: no errors; warnings for intentionally pinned dependencies, JavaScript, landscape orientation, and conservative feature-check analysis |
| `deploy-android.sh emulator-5554` | PASS: rebuild, unit tests, APK/AAB, install, launch |
| `git diff --check` | PASS |

## Emulator results

Pixel 6 AVD, Android 16 / API 36, Android System WebView 133.0.6943.137. `android/scripts/test-device.py` passed in both landscape orientations after setting the emulator's acceleration sensor to opposite landscape directions. This is an emulator result, not a physical-phone result.

Verified on the real bundled app:

- Local startup, full-height canvas, 272-pixel logical raster and nearest-neighbor compositing. The final display scale is fractional (approximately 3.978 on this emulator). **The user's request to remove excess top/bottom padding supersedes the original integer display-scale plan.** Native whole-pixel letterboxing was removed; iPhone rendering remains unchanged.
- Real module Worker loading Wasm and returning a legal AI move from the shared fixture.
- Native purchase gate reached by short Android touch events; missing-key build explains that purchases are unconfigured.
- Free editor entry, native Level code dialog, Done returning focus to the game, Back dismissing a native purchase notice first, and Back exit confirmation with Keep playing.
- Actual Home/foreground lifecycle changes reaching the mobile module; app timers pause/resume without losing the page.
- A localStorage sentinel retained through `adb install -r` of the APK. This proves same-origin storage persistence, not completion of the campaign.
- Unauthorized `/api/session` and disallowed API paths rejected, including after forged JavaScript ownership state; subsequent native state restores the locked UI.
- Malformed/non-string bridge fields and unknown grant actions ignored. A same-origin iframe cannot open the native purchase UI; external navigation attempts leave the app at its local origin.
- Native HTTP listener rejects non-local Host, duplicate Host headers, chunked requests, oversized bodies and traversal. Versioned atlas URLs load successfully.

Screenshot: [full-height landscape menu](verification/landscape.png).

The emulator initially showed keyboard/stylus onboarding UI during input automation. That OS onboarding was dismissed and emulator stylus handwriting disabled before the successful suite. Native keyboard appearance and typing must still be checked on a physical phone.

## Outstanding release acceptance

These checks have **not** passed by implication from the automated results:

- Installation from the Play closed track, and real Play purchase/restore/pending/cancel/refund/account-change flows. The Full Game product now has an Active `full-game` Buy option; base pricing is EUR 1.99 and the US price was verified as USD 1.99. The Console public key and upload signing are now configured; jarsigner verified the release bundle and Play accepted version 1. The earlier emulator run used the missing-key debug build, so it does not validate live billing.
- Physical Android phone: both rotations/cutouts, complete keyboard typing, audio audibility and focus behavior, rapid background/resume, actual offline owned relaunch, signed progress-preserving upgrade, complete campaign progression through Tutorial / Basics I–IV, paid portal gate, online lobby play and rematches.
- Runtime encrypted-cache persistence with an actual verified Play purchase and real acknowledgement outage/retry. JVM tests cover the decision logic and signatures; emulator free mode does not prove live billing/Keystore recovery.
- API 26 device coverage, large-window/tablet usability and current WebView versions. The shipped layout remains phone-first.
- Native iOS simulator/device regression suite after the shared module rename. Rust tests, iOS Wasm checks and bundled asset generation passed; the Swift test's resource lookup was updated, but Xcode tests were not run in this session.
- Privacy policy, production server retention review, Data safety, screenshots/listing, content rating, current Play submission requirements, and human play-quality review.

Build artifacts are generated and ignored by Git. Reproduce them with `./deploy-android.sh`; supply release credentials as documented in [README.md](README.md) before creating a distributable release.


## Version 2 validation — 16 September 2026

- Signed `android/build/play-alpha/maginet-2.aab` prepared, versionCode 2/versionName 1.0. SHA-256: `4c463d26cadf1eae69b986b21f559646f0080e472d6c4714585868fa9e72f1ca`. `jarsigner` verified the bundle; `apksigner` verified the release APK. Upload pending.
- Android unit tests: 5 passed (purchase signatures, ownership transitions, acknowledgement retries, proxy policy and reviewer-code verification). API 36 emulator acceptance passed for Wasm/real worker, touch, dialogs, lifecycle and restart persistence.
- Reviewer access: actual native code entry, encrypted independent cache, restart, restore retention, End review and proxy relock passed on the emulator. Keyboard-based input was used after touch automation hit moving IME coordinates.
- iOS simulator (iOS 26.2): full native/Web/UI suite passed, plus the reviewer dialog invalid-code UI test. Reviewer tests cover persistence, normalization, invalid code, verifier rotation, purchase separation and refund. Tests were corrected to wait for asynchronous StoreKit updates and use the new Settings coordinates.
- Rust workspace: 76 tests passed. Browser Wasm check and iOS/Android release Wasm asset builds passed.
- Shared `nativeFetch` network-error handling corrected: its URL parameter no longer shadows the native request function.
- Physical Pixel 9a: the full debug acceptance checks and real reviewer-code/persistence/removal checks passed. Live Google Play billing still requires track installation and license testing.

- Signed v2 release APK installed and rendered the main menu on the emulator. Stop the debug app before launching release: both variants use the fixed loopback port 18743. The initial parallel launch reported a port conflict; it succeeded after stopping debug.

- Console: reviewer instructions and support contact saved; target audience 9–12 and older saved; Data safety final save confirmed. User corrected and submitted the rating for non-human fantasy creatures. Store artwork and v2 upload remain manual handoff steps. No Alpha submission performed.

### Final landscape adjustments

- Android exports the larger horizontal system inset on both sides, preserving full-height rendering. The device acceptance script now checks symmetric native inset values.
- Local battles with on-screen controls rotate blue board mages 180 degrees, including shadows, selection arrows, mana bars and attached particle origins/velocities. Roster controls retain their existing rotation; campaign, AI, online and controls-off rendering remain unchanged.
- Rebuilt signed v2 bundle (checksum above), Android debug/release and iOS Wasm assets. Rust workspace (76), browser Wasm, Android unit tests and full iOS simulator suite passed. Android emulator acceptance passed after restarting from the main menu; the first attempt had a local battle still open, so menu coordinates did not reach the purchase dialog.
- Visually checked local battle rendering on the Android emulator and unlocked Pixel 9a: `build/verification/local-facing.png` (Pixel capture). Both landscape rotation settings reported equal left/right insets on each device: emulator 48.761906 CSS px; Pixel 57.904762 CSS px.

- Added a Settings-style result banner above Rematch: House Ruby wins (ruby), Azur Clan wins (blue), or Stalemate! (olive), displayed after the result animation completes. Rebuilt signed v2 and both mobile Wasm bundles; workspace tests, browser Wasm check and Android unit tests passed.

## Version 3 upload replacement

The user confirmed an earlier v2 was uploaded. Release configuration now persists versionCode 3 (versionName 1.0). Signed `android/build/play-alpha/maginet-3.aab` includes all final changes. SHA-256: `f5866ed47d0c8f30148acd73f111a423c5149e1ee475690fcbb334b5f14eb4ac`. Android unit tests and release assembly passed; bundle signature verified by preparation script and APK signature/version verified separately. No upload performed. The final local v2 receipt does not identify which earlier v2 build the user uploaded.

## Alpha submission — 16 September 2026

User uploaded v3 and completed artwork assignment. Default listing status confirmed ready for review; all initial setup blockers cleared. Alpha v3 final validation had no errors, only the optional deobfuscation-file warning. Saved the release and confirmed submission of 15 changes. Publishing overview shows **Changes in review** for `1.0 alpha (3)` with automated quick checks running (up to 13 minutes remaining when observed). Managed publishing is off; approval will publish to the closed Alpha test automatically. No production rollout. Google approval and live Play billing checks remain outstanding.
