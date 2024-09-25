#!/usr/bin/env bash
set -eu
python3 -m mypy src scripts
python3 -m ruff check
python3 -m ruff format --check
