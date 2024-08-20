#!/usr/bin/env bash
# shellcheck disable=SC2296
if [[ "${BASH_SOURCE[0]}" = */* ]]; then this_dir="${BASH_SOURCE[0]%/*}"; # bash
else this_dir=.;
fi
# shellcheck source=./files.sh
. "$this_dir"/files.sh

cog -PUre "${files_to_cog[@]}"
