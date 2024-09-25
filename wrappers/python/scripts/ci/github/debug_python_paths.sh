#!/usr/bin/env bash
set -eu
cd wrappers/python
export VIRTUAL_ENV="$PWD/.venv"
export PATH="$VIRTUAL_ENV/bin:$PATH"
# shellcheck disable=SC2016
echo '$PATH:'
echo "$PATH" | tr ':' '\n  - '

command -v python
command -v python3
command -v poetry || echo "missing poetry"
if ! command -v mypy; then 
  if command -v mypy.exe; then
    echo "missing mypy, but found mypy.exe"
  else
    echo "missing mypy{.exe}"
  fi
fi
stat ./.venv/bin/python || stat ./.venv/bin/python.exe || echo "missing .venv/bin/python{.exe}"
python --version
