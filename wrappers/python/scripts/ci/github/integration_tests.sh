#!/usr/bin/env bash
set -eu

# starting in repo root
cd wrappers/python

echo "PATH: ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
echo "$PATH" | tr ':' '\n' | sed 's/^/  - /g'

if ! command -v pagefind; then
  echo "pagefind not found in PATH"
  exit 1
fi

# check that PYTHONPATH is set correctly
echo
echo "PYTHONPATH: ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
echo

python -c 'import sys;print("  - " +  "\n  - ".join(sys.path))'
# ^ wrappers/python/src should be at the front of the path

echo
echo "testing import ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
echo

export PAGEFIND_PYTHON_LOG_LEVEL=DEBUG
python -c '
import logging
import os
from pagefind.service import get_executable

logging.basicConfig(level=os.environ.get("PAGEFIND_PYTHON_LOG_LEVEL", "INFO")) 
print(f"exe={get_executable()}")
'
echo
echo "python -m pagefind --help ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
echo

python -m pagefind --help

echo
echo "running integration tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
echo

python src/tests/integration.py
