#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $SCRIPT_DIR

cd pagefind
cargo build --release --features extended
cd $SCRIPT_DIR

PAGEFIND=$(realpath "$SCRIPT_DIR/target/release/pagefind")
REPO_WD=$(realpath "$SCRIPT_DIR")

npx -y toolproof@0.3.0 --placeholders pagefind_exec_path="$PAGEFIND" repo_wd="$REPO_WD" -c 4 -i
