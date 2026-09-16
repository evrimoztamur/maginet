# Google Play listing material

Reuse the approved-style iOS copy and artwork. The full description is copied verbatim from `ios/app-store/fields/description.txt`; the short description is adapted to Play's 80-character limit.

Phone screenshot source: `ios/app-store/screenshots/`, suggested order: `01-main-menu.png`, `05-shields-ii.png`, `07-campaign-map.png`, `03-patterns-ii.png`, `04-beams-ii.png`, `02-basics-iv.png`, `06-rite-iii.png`. These are actual iOS simulator captures of the shared game, not Android-device verification. They have no promotional overlays. Check the composition against Android before publishing; some show paid campaign content.

App icon source: `static/png/appicon.png` (512 × 512). `feature-graphic.png` is a 1024 × 500 composition using the existing app icon and logo on the game background. `screenshots/` preserves the seven original captures pixel-for-pixel, centered on a 2800 × 1575 (16:9) game-colored canvas to meet Play aspect-ratio rules. All screenshots and the feature graphic are opaque RGB PNGs. Rebuild with `./android/play/listing/build-artwork.sh` (ImageMagick required).

Privacy URL: https://maginet.evrim.zone/privacy. The policy has been extended to Android/Google Play and deployed; the public HTTPS response was verified against the local file on 16 September 2026. Public support: maginet@evrim.zone.

References: https://support.google.com/googleplay/android-developer/answer/9866151

These files are preparation artifacts; their presence does not mean that the listing or declarations have been saved in Play Console.
