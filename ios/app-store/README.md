# Maginet — App Store submission kit

English (U.S.), version 1.0. Open `index.html` for a screenshot gallery and copy buttons, or paste the individual files in `fields/`. `metadata.json` contains the same values in structured form. These files are prepared locally; they have not been entered into App Store Connect.

## Screenshots

Upload the seven PNGs in `screenshots/` to **iPhone → 6.5-inch Display**. Each is 2778 × 1284 pixels, landscape, with no alpha channel. These dimensions match [Apple's screenshot specifications](https://developer.apple.com/help/app-store-connect/reference/app-information/screenshot-specifications).

Suggested order: Main Menu, Shields II, Campaign Map, Patterns II, Beams II, Basics IV, Rite III. The first three introduce the game, show a battle, and show campaign progression.

Captured from the native iOS WKWebView on an iPhone 13 Pro Max simulator, using the exact bundled Web assets from the uploaded 1.0 archive. Full Game was unlocked through a local StoreKit test transaction; campaign progress was seeded on a dedicated simulator to reach the selected levels. No real purchase was made. Screenshots show the actual game rendering without promotional overlays. PNG export removes the unused alpha channel without changing the screenshot composition.

Shields II shows a missile in flight during move six; Beams II shows the impact during move six. Both were played through five preceding legal moves. The capture harness used deterministic engine-generated moves and validated the opponent moves against the running game’s legal moves. `alternatives/` includes the opening positions and settled boards after six moves.

## Fields on this page

- Promotional Text, Description, Keywords, Support URL, Marketing URL, Copyright: copy from `fields/`.
- Version: **1.0**.
- Routing App Coverage File: leave empty; the app does not provide navigation routes.
- App Clip and iMessage App: leave empty.
- Build: select the uploaded 1.0 build when processing completes. Resolve any export-compliance prompt using the app's actual encryption usage.
- Game Center: leave unchecked; the game uses its own multiplayer service.
- Sign-in required: leave unchecked; no app account is needed.
- Review contact: **Evrim / Öztamur / +31614297640 / maginet@evrim.zone**.
- Review Notes: copy `fields/review_notes.txt`.
- Review attachment: optional; leave empty unless Apple requests supporting material.
- Release: the current **Automatically release this version** setting publishes after approval. Select **Manually release this version** if you want control over launch day.

Name and subtitle are entered under **App Information**, not in the version description. Suggested subtitle: **A duel of mages and minds**.

## Support website

The supplied URLs are retained: `https://maginet.evrim.zone` for support and `https://evrim.zone` for marketing. Both respond. The game site's current HTML links to social accounts, but does not display `maginet@evrim.zone`. Add a visible support link there before review, for example:

> Need a hand? For help with Maginet, purchases, or restoring Full Game, email maginet@evrim.zone. Please include your iPhone model, iOS version, and a short description of the issue.

[Apple requires the Support URL to lead to actual contact information](https://developer.apple.com/help/app-store-connect/reference/app-information/platform-version-information). Your review phone number belongs in the private review contact field; the public support email is sufficient for the proposed page copy.

## Before App Review

The version page alone does not complete the first submission. Complete App Privacy and its privacy policy URL, age rating, pricing and availability, and the **Full Game** in-app purchase. Do not infer privacy answers from this marketing copy. The existing iOS verification notes also list physical-device and production purchase acceptance checks.

Full Game localization:

- Display name: **Full Game**
- Description: **Unlock the full campaign and every online mode.**
- Product ID: `zone.evrim.maginet.all`
- Type: **Non-Consumable**

Attach the first in-app purchase to the app version for review. Storefront prices, agreements, and the purchase's review screenshot must be completed separately.

## Copy checks

Name: 7/30 characters. Subtitle: 25/30. Promotional text: 162/170. Description: 1396/4000. Keywords: 95/100 ASCII bytes. In-app purchase description: 47/55. Description and text exports use plain text, with no Markdown formatting to leak into the listing.

## Latest upload (16 September 2026)

Version **1.0, build 7** contains the latest shared mobile changes and uploaded successfully. Apple processing starts after upload. This was **upload only** at the user's request: the existing version 1.0 review submission was not changed or resubmitted. Select build 7 later when ready to update the submission. Archive and receipt: `../build-final-update/`. Prepared update text: `fields/whats_new.txt`.

## Version 1.1 submission (17 September 2026)

Version **1.1, build 2** uploaded and submitted successfully. App Store Connect confirmed **Waiting for Review**. The existing automatic-release setting is retained. Archive, upload logs, test logs, and receipt are in `../build-ipad-release/`; the submitted archive is `Maginet-1.1-final.xcarchive`. Build 1 was superseded before submission.

This release enables native iPad support and fits the complete interface to tablet aspect ratios, alongside the latest campaign and visual updates. The iPad 13-inch listing includes `screenshots-ipad/01-main-menu.jpg` (2752 × 2064), captured from the native iPad Pro simulator after the layout correction. Release notes are in `fields/whats_new.txt`. Native rendering/worker tests and touch/purchase entry tests passed on iPad; canvas sizing tests passed with default and iOS features.
