#!/usr/bin/env bash

PKG_VERSION=$(awk -F= 'NR == 3 {print $2}' Cargo.toml | sed 's/^ *"\(.*\)" *$/\1/')

rm ../pagefind/vendor/*
wasm-pack build --release -t no-modules
mkdir -p ../pagefind/vendor
cp pkg/pagefind_web_bg.wasm ../pagefind/vendor/pagefind_web_bg.$PKG_VERSION.wasm
cp pkg/pagefind_web.js ../pagefind/vendor/pagefind_web.$PKG_VERSION.js

ls -lh ../pagefind/vendor/