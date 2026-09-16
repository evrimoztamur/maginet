# Maginet for Android

Kotlin / Android WebView shell for the full bundled Rust/Wasm game. Android 8 (API 26) minimum, API 36 target, package `zone.evrim.maginet`. Tutorial and Basics I–IV, local battles, AI, and editor are free. The permanent **Full Game** purchase (`zone.evrim.maginet.all`) unlocks the remaining campaign and online modes.

This is release-prepared implementation, **not verified for publication**. The unsigned release bundle needs your Play public key and signing configuration before distribution. See [VERIFICATION.md](VERIFICATION.md) for actual results and outstanding checks.

## Build and install

Requirements: JDK 17, Android SDK platform 36 / platform-tools, Rust with `wasm32-unknown-unknown`, and network access for first-time tool downloads. Gradle 8.13 is pinned by the checked-in wrapper and distribution checksum. The build script installs/reuses wasm-bindgen-cli **0.2.91**, matching Cargo.lock. SDK licenses must already be accepted. Set `ANDROID_HOME` when the SDK is outside `~/Library/Android/sdk` (the macOS default).

```sh
rustup target add wasm32-unknown-unknown
./deploy-android.sh                 # Build debug APK, release AAB, and run JVM tests
./deploy-android.sh emulator-5554   # Also install and launch debug APK
```

Outputs:

- `android/app/build/outputs/apk/debug/app-debug.apk`: installable debug APK, package `zone.evrim.maginet.debug`.
- `android/app/build/outputs/bundle/release/app-release.aab`: release bundle, unsigned until release signing is configured.

The script rebuilds the full catalogue with `--locked --features android`; do not add `demo`. Bundled assets and build output are ignored. Android Studio can open `android/`; run the asset script before building there. `WASM_BINDGEN=/path/to/wasm-bindgen android/scripts/build-assets.sh` builds assets independently. Java and Android sources need no Rust NDK library: the game runs as Wasm.

Debug uses a separate application ID, private cache, Keystore alias, and WebView data directory. There is no ownership override in either variant. Debug purchases require a separately configured Play app/key for that debug package. Use the **signed release** app from an internal track for testing the production product. Do not run debug and release simultaneously: they intentionally use the same fixed localhost port.

## Signing and Play Console

Keep the upload keystore and passwords outside this repository. Configure these environment variables through a local secret manager or CI secret store:

```text
MAGINET_PLAY_PUBLIC_KEY  Base64 RSA licensing/public key from this app's Play Console record
MAGINET_KEYSTORE         Absolute path to upload keystore
MAGINET_STORE_PASSWORD   Keystore password
MAGINET_KEY_ALIAS        Upload key alias
MAGINET_KEY_PASSWORD     Upload key password
```

The public key is not secret, but must be the correct key for this application. Without it the build runs free content and refuses purchases. The verification key is compiled into BuildConfig. Then run:

```sh
CONFIGURATION=Release ./deploy-android.sh              # Signed APK and AAB
CONFIGURATION=Release ./deploy-android.sh DEVICE_SERIAL # Also install signed APK
```

Increment `versionCode` in `app/build.gradle.kts` for each Play upload. Enroll in Play App Signing, protect the upload key, and retain it for upgrades. A release bundle built without credentials is useful for inspection only; do not upload it. Do not sign release with the debug key.

Create the Play application with package `zone.evrim.maginet`, and create a **one-time, non-consumable** product with ID `zone.evrim.maginet.all`. Configure one standard purchase option, available countries and localized listing. Target US$1.99 / €1.99 using regional price overrides and review other currencies. The app displays Google's formatted price and uses the corresponding offer token; it never hardcodes a checkout price. Complete merchant setup, tax/banking, product activation, content rating, target audience, screenshots, support contact, and store listing.

Upload the signed AAB to an internal testing track, add license testers and track testers, and install through their opt-in Play Store link. Test completed, cancelled and pending purchases; pending-to-purchased transitions; acknowledgement/network failure recovery; restore after reinstall; account changes; refund/revocation; and offline relaunch after verified ownership. A successful current purchase query with no owned product clears cached access. Temporary failures and invalid signed records retain an already verified cache and report failures on explicit restore.

Google references: [billing integration and client acknowledgement](https://developer.android.com/google/play/billing/integrate), [billing testing](https://developer.android.com/google/play/billing/test), [app signing](https://developer.android.com/studio/publish/app-signing). This app uses Billing Library 8.3.0; confirm Play's submission requirements when publishing.

## Architecture

- Cargo's `ios` and `android` features enable shared `mobile` behavior. `static/js/mobile-access.js` handles access, keyboard, network errors and lifecycle. iOS retains WKScriptMessageHandler; Android uses AndroidX WebKit's origin-scoped message listener. The iOS origin, rendering and storage keys are unchanged.
- Android binds a bounded HTTP server only to `127.0.0.1:18743`. That fixed WebView origin preserves localStorage campaign progress across same-package, same-signature upgrades. Uninstalling or clearing app data deletes progress. There are no accounts, cloud saves, or cross-store entitlements.
- The listener checks Host, rejects malformed/duplicate headers, chunked encoding and traversal, limits headers to 16 KiB and bodies to 1 MB, and closes each connection. The proxy requires same-origin Referer/Origin and native ownership, allows only existing game routes/methods, strips caller headers, and forwards only to production HTTPS. Redirects are not followed; connect/read timeouts are 15 seconds and responses are bounded to 2 MB. Static asset query strings are supported for versioned artwork URLs.
- Cleartext is permitted only for `127.0.0.1`. Navigation and resource loads are restricted to the local origin. Only local main-frame string messages are processed; there is no `addJavascriptInterface` or web-accessible grant operation. JavaScript state never authorizes the proxy.
- RSA signatures are verified against the configured Play key before inspecting package, product, purchased state, and token. Signed ownership records are encrypted with AES-GCM using an Android Keystore key, stored in `noBackupFilesDir`, and reverified at launch. Backup and device transfer are disabled. Billing queries run on startup, foreground, and Restore. Revision tracking prevents older queries from overwriting newer purchase grants. Failed acknowledgements retry with exponential delays (six retries); subsequent refreshes retry any still-unacknowledged purchases. No product is consumed.
- Both landscape orientations are supported on phones. Android fills the available landscape height, matching iPhone scaling. Sprites render at logical resolution with smoothing disabled before the final display scale; there is no whole-pixel letterboxing on phones. Narrower/larger windows retain the same fixed layout, limiting height only when needed to fit its width. Safe insets feed the Rust touch/render coordinate mapping. Dedicated tablet design remains deferred.
- Native dialogs provide purchase/restore and text entry. Back closes a dialog first; otherwise it asks before leaving the app. Background events suspend game ticks and audio and terminate active AI workers before WebView timers pause. Resume refreshes ownership and resumes the game.

## Privacy and release checks

Publish an accurate privacy policy and complete Play's Data safety form after reviewing both this app and the existing production server. Offline play stores campaign/settings locally. Optional online play sends the game's session identifier, lobby data and moves to `https://maginet.evrim.zone`; the service also sees normal network metadata such as IP addresses. Google Play handles payments; signed purchase records/tokens are stored privately on the device and used for billing acknowledgement. This wrapper adds no advertising, analytics, account registration, contacts, location, or storage permissions. Do not infer the server's retention/deletion practices from the client—verify those before making store disclosures.

Physical-device checks must cover both orientations and cutouts, short taps/drags, real keyboard entry, completing free campaign progression, locked paid portals and online modes, offline relaunch, actual audible background/resume, and persistent progress after a signed upgrade. Test large windows, API 26, and a current Android release. Publication is a separate action.

## Tests

```sh
cargo test --workspace --locked
cargo check --locked --target wasm32-unknown-unknown
cargo check --locked --target wasm32-unknown-unknown --features ios
cargo check --locked --target wasm32-unknown-unknown --features android
android/gradlew -p android testDebugUnitTest lintDebug
```

For the debug WebView/device suite, install the debug APK and launch it on an unlocked emulator/device. Install Python `websocket-client` in a virtual environment, then:

```sh
python android/scripts/test-device.py emulator-5554 \
  --install android/app/build/outputs/apk/debug/app-debug.apk
```

The suite uses adb and WebView's debug-only DevTools socket. It exercises real bundled Wasm/module workers, full-height raster scaling, native gates, malformed messages, unauthorized requests, keyboard and Back, lifecycle, and persistence through APK replacement. It never simulates a verified entitlement or purchases anything. It briefly uses a disposable localStorage test key and localhost port 19222. Start at the main menu with purchases unconfigured. Actual paid progression and Play purchases need the external setup above.

## Closed testing alpha upload

See [the alpha release handoff](play/README.md). `./prepare-play-alpha.sh` builds a signed, IAP-enabled bundle and draft alpha metadata from local signing configuration. It does not upload. `MAGINET_VERSION_CODE` / `MAGINET_VERSION_NAME` select the release version; Play access and the registered upload key must be verified before upload.
