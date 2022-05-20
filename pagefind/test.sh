#!/usr/bin/env bash

if [ $# -eq 0 ]
  then
    TEST_BINARY=../target/release/pagefind cargo test --release --test cucumber -- -c 16 --tags "not @skip"
  else
    TEST_BINARY=../target/release/pagefind cargo test --release --test cucumber -- -c 16 --name "$1"
fi
