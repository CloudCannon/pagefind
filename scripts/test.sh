#!/usr/bin/env bash
set -eo pipefail

if [[ -z "${CHROME}" ]]; then
  CHROME="$(which google-chrome || which chromium || which chromium-browser || which chrome || true)"
else
  CHROME="$(which "$CHROME" || true)"
fi

if [[ -z "$CHROME" ]]; then
  echo "No Chrome browser found. Please install Chrome or Chromium."
  exit 1
fi

export CHROME

npx toolproof@latest