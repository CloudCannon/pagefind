#!/usr/bin/env bash
set -e

##
#
# TODO: Run the language files through binary diffing (i.e: divvun/bidiff) based on generic
# so that the main Pagefind CLI doesn't need to include each language wasm in full.
#
##

rm ../pagefind/vendor/wasm/* || true
mkdir -p ../pagefind/vendor/wasm

# Build the generic wasm file
if [ "$1" = "debug" ]; then
    wasm-pack build --dev -t no-modules
else
    wasm-pack build --release -t no-modules 
fi
mv pkg/pagefind_web.js ../pagefind/vendor/pagefind_web.0.0.0.js
# Append pagefind_dcd to the decompressed wasm as a magic word read by the frontend
printf 'pagefind_dcd' > ../pagefind/vendor/wasm/pagefind_web_bg.unknown.0.0.0.wasm
cat pkg/pagefind_web_bg.wasm >> ../pagefind/vendor/wasm/pagefind_web_bg.unknown.0.0.0.wasm
gzip --best ../pagefind/vendor/wasm/pagefind_web_bg.unknown.0.0.0.wasm

# Build the language-specific wasm files,
# naively grabbing all features from this crate's Cargo.toml
grep -e pagefind_stem/ Cargo.toml | while read line ; do
    if [ "$1" = "debug" ]; then
        wasm-pack build --dev -t no-modules --features ${line:0:2}
    else
        wasm-pack build --release -t no-modules --features ${line:0:2}
    fi

    # Append pagefind_dcd to the decompressed wasm as a magic word read by the frontend
    printf 'pagefind_dcd' > ../pagefind/vendor/wasm/pagefind_web_bg.${line:0:2}.0.0.0.wasm
    cat pkg/pagefind_web_bg.wasm >> ../pagefind/vendor/wasm/pagefind_web_bg.${line:0:2}.0.0.0.wasm
    gzip --best ../pagefind/vendor/wasm/pagefind_web_bg.${line:0:2}.0.0.0.wasm
done

ls -lh ../pagefind/vendor/wasm/
