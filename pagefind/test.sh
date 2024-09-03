#!/usr/bin/env bash

cargo build --release --features extended
if [ -z "$1" ]; then
    TEST_BINARY=../target/release/pagefind npx -y humane@0.9.0
else
    TEST_BINARY=../target/release/pagefind npx -y humane@0.9.0 --name "$1"
fi
