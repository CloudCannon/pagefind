#!/usr/bin/env bash

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
    RUSTUP_TOOLCHAIN=nightly wasm-pack build --release -t no-modules --manifest-path ./Cargo.toml -Z build-std=panic_abort,std -Z build-std-features=panic_immediate_abort
fi
node build.js
mv pkg/pagefind_web.js "../pagefind/vendor/pagefind_web.$WASM_VERSION.js"
# Append pagefind_dcd to the decompressed wasm as a magic word read by the frontend
printf 'pagefind_dcd' > "../pagefind/vendor/wasm/pagefind_web_bg.unknown.$WASM_VERSION.wasm"
cat pkg/pagefind_web_bg.wasm >> "../pagefind/vendor/wasm/pagefind_web_bg.unknown.$WASM_VERSION.wasm"
gzip --best "../pagefind/vendor/wasm/pagefind_web_bg.unknown.$WASM_VERSION.wasm"

# Build the language-specific wasm files,
# naively grabbing all features from this crate's Cargo.toml
grep -e pagefind_stem/ Cargo.toml | while read line ; do
    lang="${line:0:2}"
    echo "Building pagefind wasm for language: $lang ($WASM_VERSION)"
    if [ "$1" = "debug" ]; then
        wasm-pack build --dev -t no-modules --features "$lang"
    else
        RUSTUP_TOOLCHAIN=nightly wasm-pack build --release -t no-modules --features "$lang" --manifest-path ./Cargo.toml -Z build-std=panic_abort,std -Z build-std-features=panic_immediate_abort
    fi

    # Append pagefind_dcd to the decompressed wasm as a magic word read by the frontend
    printf 'pagefind_dcd' > "../pagefind/vendor/wasm/pagefind_web_bg.$lang.$WASM_VERSION.wasm"
    cat pkg/pagefind_web_bg.wasm >> "../pagefind/vendor/wasm/pagefind_web_bg.$lang.$WASM_VERSION.wasm"
    gzip --best "../pagefind/vendor/wasm/pagefind_web_bg.$lang.$WASM_VERSION.wasm"
done

ls -lh ../pagefind/vendor/wasm/
