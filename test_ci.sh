#!/usr/bin/env bash
set -eu
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$SCRIPT_DIR"

PAGEFIND=$(realpath "$SCRIPT_DIR/target/$1/pagefind")
REPO_WD=$(realpath "$SCRIPT_DIR")

npx -y toolproof@0.4.1 --placeholders pagefind_exec_path="$PAGEFIND" repo_wd="$REPO_WD" -c 1
