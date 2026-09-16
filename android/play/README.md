# Closed testing alpha: release preparation

Target app: `zone.evrim.maginet`. Target closed-testing track: `alpha`.

Version **1 (1.0)** was accepted by Play Console on 16 September 2026 and attached to the saved closed-testing **Alpha** draft, **1.0 alpha (1)**, with en-GB release notes. It is not yet available to testers. The signed bundle includes this app's Play billing public key.

Release configuration and the dedicated upload keystore are outside the repository in `~/.config/maginet/`. Back up that directory securely; future uploads must use the same upload key. No upload certificate existed before this first upload. The bundle SHA-256 is `2c7636b1b75d4b73d5324461a8ab4138d131c6a757cb5d01549a914c01d57217`.

Console rollout still requires completion of dashboard setup. Listing text is saved as a draft. Financial features, health, ads, advertising ID, and government declarations are saved. Content rating is only partly filled; target audience, Data safety, privacy policy, and reviewer access are not complete. Alpha targets all 177 available countries/regions. Both the existing Alpha friends list (5 members) and Maginet Alpha (1 member, `cloud@evrim.zone`) are saved as track testers. Maginet Alpha is also saved as a developer-account license tester list with RESPOND_NORMALLY.

## Inputs required

- The existing app's Google Play Console link, or confirmation that this is a new app.
- The app's Base64 RSA billing/licensing public key from Play Console.
- Existing upload keystore, key alias and passwords. Reuse the registered upload key for an existing app. For a new app, generate an upload key and enroll in Play App Signing; do not invent a replacement key for an already-registered app.
- An unused version code, checked against all previously uploaded artifacts.
- For an API upload: a service-account JSON file, Google Play Developer API enabled in its Cloud project, and that account granted access to this app in Play Console. Scope release access to testing tracks. Product configuration requires the applicable monetization permissions. Alternatively perform the upload in the signed-in Play Console.

Keep credentials outside the repository. `release.env.example` is a blank template, not a file containing credentials. Copy it to `~/.config/maginet/android-release.env`, set permissions to `600`, and fill it locally. The shell file is trusted code and is sourced by Bash; only use your own configuration.

## Prepare the actual upload package

```sh
./prepare-play-alpha.sh
# Or pass the location of your local release environment:
./prepare-play-alpha.sh /absolute/path/to/android-release.env
```

This validates required inputs, checks the RSA public-key format and keystore access, builds/tests the signed release with the pinned Wasm assets, verifies the AAB signature, and writes:

- `android/build/play-alpha/maginet-VERSION_CODE.aab`
- `android/build/play-alpha/track.json`: draft `alpha` track release metadata with English release notes.
- `android/build/play-alpha/receipt.json`: package/version/product/track and bundle SHA-256, explicitly marked not uploaded.

The preparation command has **no upload side effects**. It stops before building if credentials are missing. `MAGINET_VERSION_CODE` and `MAGINET_VERSION_NAME` override the local defaults without source edits. Signing passwords are passed to keytool via environment, not command-line arguments or log output.

## Full Game product

| Field | Value |
| --- | --- |
| Product ID | `zone.evrim.maginet.all` |
| Name | Full Game |
| Description | Permanently unlock the remaining campaign and online multiplayer modes. |
| Product model | One-time product, non-consumable; standard buy option |
| Price targets | USD 1.99 and EUR 1.99; review regional prices and availability in Console |
| Free content | Tutorial, Basics I–IV, local battles, AI and editor |

Create or inspect this exact product in Console. Activate its standard buy option for the intended test countries, complete the payments profile if required, and verify the public key belongs to this app. Do not create a subscription or consume purchases. On 16 September 2026, `zone.evrim.maginet.all` was created in Console with the `full-game` Buy option, and Console confirmed **Active**. The base regional price was set to EUR 1.99; the United States override was verified as USD 1.99. The default Console tax category is Digital app sales. Real Play billing tests remain outstanding.

## Upload and enable testing

1. Inspect the existing app, upload certificate, track, and version history. For a new app, complete initial Console setup and Play App Signing. The first app upload may need to be completed in Console before API edits are available.
2. Upload the signed AAB to **Testing → Closed testing → alpha**. Use `release-notes-en-US.txt`. API automation uses the edits/bundles/track workflow, constrained to the named app and `alpha` track. The prepared metadata starts as a draft; drafting alone does not make it installable for testers.
3. Complete the Console-required app content, country availability, store listing and review steps. Add the intended testers and make the closed release available when Console permits. The Console tester lists described above are configured; the preparation script itself does not modify them.
4. Add purchase-test accounts as **license testers**, as well as closed-track testers. Install through the closed-test opt-in link. Track membership alone does not make payments free.
5. Verify localized product price, purchase, pending/cancelled/declined payment, acknowledgement, restore, offline owned relaunch and revocation. Test the release package, not `zone.evrim.maginet.debug`.

References: [API access](https://developers.google.com/android-publisher/getting_started), [app edits and first-upload prerequisites](https://developers.google.com/android-publisher/edits), [bundle upload](https://developers.google.com/android-publisher/api-ref/rest/v3/edits.bundles/upload), [tracks](https://developers.google.com/android-publisher/tracks), [billing tests](https://developer.android.com/google/play/billing/test).

## Release handoff (16 September 2026)

Signed version 3 is ready at `android/build/play-alpha/maginet-3.aab` (versionName 1.0). SHA-256: `f5866ed47d0c8f30148acd73f111a423c5149e1ee475690fcbb334b5f14eb4ac`. The user confirms version 2 was already uploaded; version 3 includes all final landscape and result-banner changes, including ‘Stalemate!’.

Completed in Console: reviewer access instructions, privacy policy, Strategy category and support contact, ads/advertising-ID/government/health/financial declarations, target audience ages 9–12 and older (no Expert Approved opt-in), and Data safety. The user changed the rating answers to violence against non-human creatures and submitted the questionnaire; preserve that declaration.

Data safety declares optional collection of app interactions, other user-generated content, other gameplay actions, and device/other identifiers. All are non-ephemeral and used for app functionality. Connection/identifier data also supports diagnostics and security. There is no advertising use or third-party sharing under the service-provider/user-initiated-transfer exemptions. The privacy URL provides deletion-request instructions. Final Console save was confirmed.

Alpha submission completed on 16 September 2026:

- The default store listing is saved and ready for review, including the uploaded icon, feature graphic and phone screenshots. All initial app setup tasks are complete.
- Version 3 is attached to `1.0 alpha (3)`. Final release validation had no blocking errors; the remaining warning concerns an optional deobfuscation file.
- Confirmed **Send changes for review** for all 15 changes. Publishing overview now shows **Changes in review**, including the Alpha full rollout, worldwide availability, tester lists, listing and declarations. Google's automated quick checks were still running at handoff; review proceeds automatically when they pass.
- Managed publishing is off: approved changes will publish automatically to the closed test. This is not a production release.

Remaining: await Google's checks/review, install through the closed-test opt-in link, and validate live billing with `cloud@evrim.zone` (already a license tester). End reviewer access before testing purchase, acknowledgement and restore. Actual Play approval and live billing validation are not yet confirmed.

Android emulator and physical Pixel 9a acceptance/reviewer-access checks passed. The iOS simulator suite and reviewer dialog UI check passed. See `../VERIFICATION.md` and `listing/reviewer-access.md`.
