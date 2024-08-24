#!/usr/bin/env bash
set -eu
python3 -m pip install poetry

# not using pipx since this is a CI environment that will be reset -- 
# there's not much risk of poetry's dependencies conflicting with ours

# python3 -m pip install pipx
# python3 -m pipx install poetry
