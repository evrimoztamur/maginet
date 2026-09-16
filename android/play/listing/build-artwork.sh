#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../../.."
mkdir -p android/play/listing/screenshots
# Preserve every source pixel; add the game's background to meet Play's 16:9 format.
for source in ios/app-store/screenshots/*.png; do
  magick "$source" -background '#002a2a' -gravity center -extent 2800x1575 -alpha off "PNG24:android/play/listing/screenshots/${source##*/}"
done
cp static/png/appicon.png android/play/listing/app-icon.png
magick -size 1024x500 canvas:'#002a2a' \
  \( static/png/appicon.png -filter point -resize 360x360 \) -gravity northwest -geometry +50+70 -composite \
  \( static/png/logo.png -filter point -resize 480x120 \) -geometry +485+170 -composite \
  -alpha off PNG24:android/play/listing/feature-graphic.png
