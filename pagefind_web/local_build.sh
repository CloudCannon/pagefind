#!/usr/bin/env bash

rm ../pagefind/vendor/*
if [ $1 = "debug" ]; then
wasm-pack build --debug -t no-modules
else
wasm-pack build --release -t no-modules
fi
mkdir -p ../pagefind/vendor
cp pkg/pagefind_web_bg.wasm ../pagefind/vendor/pagefind_web_bg.0.0.0.wasm
cp pkg/pagefind_web.js ../pagefind/vendor/pagefind_web.0.0.0.js

ls -lh ../pagefind/vendor/