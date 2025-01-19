#!/usr/bin/env bash
set -e

# This script runs Pagefind on the Pagefind documentation to generate
# an index that can be used to test the playground.

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

echo "Building Pagefind Hugo documentation site"
cd "$SCRIPT_DIR/../docs"
if ! command -v go &> /dev/null; then
    echo "Golang is not installed, please install Golang to proceed."
    exit 1
fi
if ! command -v hugo &> /dev/null; then
    echo "Hugo is not installed, please install Hugo to proceed."
    exit 1
fi

npm i
hugo

cd "$SCRIPT_DIR"

echo "Removing old index files"
echo "The following files/directories will be deleted:"
find output -mindepth 1 -maxdepth 1 ! -name 'playground' -print
echo "Do you want to proceed with deletion? (yes/no)"
read confirmation

if [ "$confirmation" = "yes" ]; then
    find output -mindepth 1 -maxdepth 1 ! -name 'playground' -execdir rm -rf {} +
else
    echo "Cancelling build script."
    exit 1
fi

echo "Running Pagefind"
../target/release/pagefind --site "../docs/public" --output-path "output"
