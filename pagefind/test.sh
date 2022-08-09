#!/usr/bin/env bash

cargo build --release
TEST_BINARY=../target/release/pagefind npx humane@latest
