#!/usr/bin/env bash
# fetch the current version of the pagefind executable
# see https://simonwillison.net/2020/Oct/9/git-scraping/
set -eu
export PATH="$PWD/.venv/bin:$PATH"
file="pagefind_version.txt"

python3 -m scripts.build.get_pagefind_release
pagefind_version=""; pagefind_version=$(cat ./"$file")

if ! git --no-pager diff --exit-code -- "$file"; then # there's a new version
  ./scripts/ci/cog/update.sh # note that $PWD is the repo root
  git add -u
  git config user.name "Automated"
  git config user.email "actions@users.noreply.github.com"
  git commit -m "chore: update pagefind binary to $pagefind_version"
  git tag "bin/$pagefind_version"
  git push
  git push --tags --follow-tags
fi
