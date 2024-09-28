#!/usr/bin/env bash
set -eu
cd wrappers/python

echo "VIRTUAL_ENV=$VIRTUAL_ENV"

# shellcheck disable=SC2016
echo '$PATH:'
echo "$PATH" | tr ':' '\n' | sed 's/^/  - /g'

echo
echo " python ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ "
echo

python --version
command -v python
command -v python3
stat ./.venv/bin/python \
  || stat ./.venv/Scripts/python.exe \
  || echo "missing .venv/bin/python{.exe}"

echo
echo " poetry ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ "
echo

command -v poetry || echo "missing poetry"

echo
echo " mypy ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ "
echo

if ! command -v mypy; then 
  if command -v mypy.exe; then
    echo "missing mypy, but found mypy.exe"
  else
    echo "missing mypy{.exe}"
  fi
fi

