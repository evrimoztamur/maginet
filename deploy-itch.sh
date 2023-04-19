#!/usr/bin/env sh

mkdir -p deploy/static/js/pkg

cp html/itch.html deploy/index.html
cp -r static deploy

wasm-pack build --target web --out-dir deploy/static/js/pkg --out-name maginet_8369734 -- --features deploy

zip itch.zip deploy -r

butler push itch.zip evrimzone/maginet:itch-web