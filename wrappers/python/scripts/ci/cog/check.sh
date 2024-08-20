#!/usr/bin/env bash
set -eu
if [[ "${BASH_SOURCE[0]}" = */* ]]; then this_dir="${BASH_SOURCE[0]%/*}"; # bash
else this_dir=.;
fi
# shellcheck source=./files.sh
. "$this_dir"/files.sh
cog -PUe --check "${files_to_cog[@]}"

