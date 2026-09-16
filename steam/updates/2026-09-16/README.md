# Steam update kit

Open [index.html](index.html) for the visual preview. The announcement is 143 words, with four gameplay GIFs. [update.md](update.md) is the readable draft; [steam-bbcode.txt](steam-bbcode.txt) is the Steam body. Title and summary have separate text files.

The narrator speaks for House Ruby against the Azur Clan, following the mobile promo copy and the supplied Zugzwang lore. The late-1800s adventure tone includes brief modern asides; it does not announce unimplemented story events.

The copy deliberately uses **out now** for iOS and Android, as requested for launch day. Both store URLs remain placeholders.

## Publish

1. Copy `title.txt`, `summary.txt`, and `steam-bbcode.txt` into the corresponding Steam announcement fields.
2. Use `assets/cover.png` for the event cover.
   Use `assets/header.png` for the optional 1920 × 622 event header.
3. Upload the four GIFs below and replace their `*_IMAGE_URL_PENDING` tokens with the uploaded image URLs (or insert the images with Steam's editor).
4. Replace `IOS_STORE_URL_PENDING` and `ANDROID_STORE_URL_PENDING` with the final store links.

| Body token | Asset | Content |
| --- | --- | --- |
| `ANIMATIONS_IMAGE_URL_PENDING` | `assets/01-animations.gif` | Three legal moves in Patterns I with movement, casting, and impact animations |
| `CAMPAIGN_IMAGE_URL_PENDING` | `assets/02-campaign.gif` | Pan from Patterns III through Diagonals I and Beams I to Shields I |
| `DEADLOCK_IMAGE_URL_PENDING` | `assets/04-deadlock.gif` | A Beam is consumed, the Deadlock! banner appears, then Blue and Red move diagonally |
| `MOBILE_CONTROLS_IMAGE_URL_PENDING` | `assets/03-mobile-controls.gif` | Touch direction pad: preview, confirm, and collect the Diagon Rune |

Each GIF also has a PNG still with the same basename. The GIFs total less than 4 MB. The cover is 800 × 450. `cover.html` is its editable layout, using the atlas logo, glyphs, exact game kerning and action-button trim, and a captured battle frame. The logo is 336 pixels wide (40% larger than the first cover); all lettering comes from the atlas. The subtitle “New campaign and juice” is drawn at 80% alpha. A campaign star and its golden sparkle sprites sit on the left of the mobile button.

Steam supports PNGs and animated GIFs in announcements; its event cover specification is 800 × 450. Sources: [Steam event editor](https://partner.steamgames.com/doc/marketing/event_tools), [event graphical assets](https://partner.steamgames.com/doc/store/assets/eventassets).

The header combines four actual battle captures under a dark overlay with the centered atlas logo. `header.html` is the editable source. The logo uses 10× nearest-neighbour scaling: its 960 × 240 sprite frame occupies exactly 50% of the header width. The official `event_header.psd` was downloaded and visually checked; the logo fits inside its safe area and artwork extends through the surrounding bleed. The two additional battle views come from the existing native iOS store screenshots; `assets/header-deadlock-source.png` is a frame from the newly captured deadlock sequence.

## Capture and scope notes

Reviewed changes after `e5406405ea9ad61a04157c8cfe691d560258e94a` through `b214965`. The short announcement covers animation, campaign and teaching revisions, mobile versions and controls, AI difficulties, team selection, attack previews, deadlock/inactivity changes, and interface polish. Build tooling and assessment reports are omitted from the player-facing copy.

Fresh captures use the real Rust/Wasm renderer built from the working tree, with an isolated browser profile and staged campaign progress. No personal saves were touched. Frames were sampled at 30 fps with the presentation clock advanced at its normal rate. The battle's scripted AI reply was checked against the game's legal moves; other AI replies were held for framing. The mobile capture runs the actual shared `mobile` build in Chromium with synthetic touch input and a local ownership bridge. It is a mobile UI capture, **not a physical iPhone/Android screenshot**. No purchases, online matches, or external publishing were performed.

The deadlock GIF uses a real editor-created 5×5 Diamond/Knight position, level code `j0100024j414809004`. Consuming its last Beam triggers the real deadlock detector; both surviving mages receive diagonal runes. The recording verifies the banner and three subsequent diagonal moves (Blue, Red, Blue), ending with a spell attack. It runs at normal speed.

The animation GIF is cropped and enlarged with nearest-neighbour scaling. The deadlock GIF is cropped around the board and banner. The campaign and mobile GIFs retain their captured canvas sizes. Transparent canvas backgrounds are composited over the game's own #002a2a background. Screenshots and GIF frames are gameplay captures, with no generated artwork.

Validation: desktop and mobile Wasm builds succeeded; all four capture runs had no browser errors; all GIFs load in the preview. The gameplay frames, cover, and full article preview were visually inspected. Store links and uploaded Steam image URLs are the only pending publication inputs.
