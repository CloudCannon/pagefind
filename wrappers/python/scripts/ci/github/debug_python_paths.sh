#!/usr/bin/env bash
set -eu
cd wrappers/python
export VIRTUAL_ENV="$PWD/.venv"
export PATH="$VIRTUAL_ENV/bin:$PATH"
set -x
echo "$PATH" | tr ':' '\n'
command -v python
command -v python3
command -v poetry || echo "missing poetry"
stat ./.venv/bin/python
./.venv/bin/python --version
