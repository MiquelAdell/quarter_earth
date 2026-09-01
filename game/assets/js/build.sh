#!/bin/bash
# npm install -d

node build.mjs
cp hector/hector.wasm dist/
sed -i.bak "s|hector.wasm|/hector/hector.wasm|g" dist/tgav.js
rm dist/tgav.js.bak
