#!/usr/bin/env bash
set -e

##
#
# TODO: Run the language files through binary diffing (i.e: divvun/bidiff) based on generic
# so that the main Pagefind CLI doesn't need to include each language wasm in full.
#
##

if [[ -z "${GIT_VERSION}" ]]; then
  WASM_VERSION="0.0.0"
else
  WASM_VERSION="${GIT_VERSION}"
fi

rm ../pagefind/vendor/wasm/* || true
mkdir -p ../pagefind/vendor/wasm

# Build the generic wasm file
if [ "$1" = "debug" ]; then
    wasm-pack build --dev -t no-modules
else
    wasm-pack build --release -t no-modules 
fi
mv pkg/pagefind_web.js ../pagefind/vendor/pagefind_web.$WASM_VERSION.js
# Append pagefind_dcd to the decompressed wasm as a magic word read by the frontend
printf 'pagefind_dcd' > ../pagefind/vendor/wasm/pagefind_web_bg.unknown.$WASM_VERSION.wasm
cat pkg/pagefind_web_bg.wasm >> ../pagefind/vendor/wasm/pagefind_web_bg.unknown.$WASM_VERSION.wasm
gzip --best ../pagefind/vendor/wasm/pagefind_web_bg.unknown.$WASM_VERSION.wasm

# Build only the en wasm file in quick mode

if [ "$1" = "debug" ]; then
    wasm-pack build --dev -t no-modules --features en
else
    wasm-pack build --release -t no-modules --features en
fi

# Append pagefind_dcd to the decompressed wasm as a magic word read by the frontend
printf 'pagefind_dcd' > ../pagefind/vendor/wasm/pagefind_web_bg.en.$WASM_VERSION.wasm
cat pkg/pagefind_web_bg.wasm >> ../pagefind/vendor/wasm/pagefind_web_bg.en.$WASM_VERSION.wasm
gzip --best ../pagefind/vendor/wasm/pagefind_web_bg.en.$WASM_VERSION.wasm


ls -lh ../pagefind/vendor/wasm/
