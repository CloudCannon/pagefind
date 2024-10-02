#!/usr/bin/env bash
set -eu

cd wrappers/python

VIRTUAL_ENV="$PWD/.venv"
echo "VIRTUAL_ENV=$VIRTUAL_ENV" >> "$GITHUB_ENV"

if ! [ -d "$VIRTUAL_ENV" ]; then
  echo "No virtualenv found at $VIRTUAL_ENV"
  exit 127
fi

# Ensure binaries from the virtualenv are available at the start of $PATH
# see https://docs.python.org/3/library/venv.html#creating-virtual-environments
if [ -d "$VIRTUAL_ENV/bin" ]; then
  # on unix systems, virtualenv puts executables in .venv/bin
  venv_bin_path="$VIRTUAL_ENV/bin"
elif [ -d "$VIRTUAL_ENV/Scripts" ]; then
  # on windows, virtualenv places executables in .venv/Scripts
  venv_bin_path="$VIRTUAL_ENV/Scripts"
fi

echo "$venv_bin_path" >> "$GITHUB_PATH"
# see https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions#adding-a-system-path
