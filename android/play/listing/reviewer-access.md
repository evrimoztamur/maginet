# Reviewer access (Android v2 and iOS)

No app account is needed. Settings → Reviewer Access opens a native text field; enter the private reviewer code and choose Unlock. This grants all Full Game content without fabricating a purchase. Normal campaign progression remains in place. End review removes only the independent review entitlement, preserving genuine purchases.

The code is kept at `~/.config/maginet/reviewer-code.txt`, outside the repository. Only its SHA-256 verifier is embedded. `scripts/configure-reviewer-code.py` synchronizes the Android and iOS verifier; `--rotate` creates a replacement code and invalidates old cached reviewer access after the next update. Both platforms store the verifier separately from purchase ownership, using Android Keystore/noBackup or iOS ThisDeviceOnly Keychain. Anyone possessing the code can unlock the game, so share it only through reviewer instructions.

Play Console instructions explicitly require version 2 or later. Do not submit the older version 1 with these instructions. Reviewer access is separately tested from real Play billing; End review before testing purchases.

Validation on 16 September 2026: Android real-code native entry, separate encrypted cache, process restart, restore retention, End review and native proxy relock passed on the API 36 emulator and physical Pixel 9a. iOS simulator tests passed for normalization, invalid codes, Keychain persistence, verifier rotation, removal, purchase separation and refunds; the native invalid-code dialog UI also passed. Real Play purchase testing requires the uploaded release and a license-test account.
