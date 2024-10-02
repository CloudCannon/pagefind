#!/usr/bin/env bash
set -eu
mypy src scripts
ruff check
ruff format --check
