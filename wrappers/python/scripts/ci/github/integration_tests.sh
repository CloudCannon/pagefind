#!/usr/bin/env bash
set -eu
# ensure pagefind is not installed
if command -v pagefind; then
  exit 1
fi
# ensure pagefind_python is not installed in the current python environment
if python3 -c "import pagefind_python"; then
  echo "dirty python environment: unexpectedly found pagefind_python"
  exit 1
fi

# use pagefind installed from the officially-maintained node.js channel
pagefind_version="$(cat ./pagefind_version.txt)" #~
npm i "pagefind@$pagefind_version"
_prev_path="$PATH"
export PATH="$PWD/node_modules/.bin:$PATH"


# remove_src_from_pythonpath="
# import os
# import sys
# from pathlib import Path
# repo_root = Path(os.getcwd())
# src = repo_root / 'src'
# sys.path.remove(str(src))
# "
_get_executable='
import logging
import os
from pagefind_python.service import get_executable

logging.basicConfig(level=os.environ.get("PAGEFIND_PYTHON_LOG_LEVEL", "INFO")) 
print(get_executable())
'
export PAGEFIND_PYTHON_LOG_LEVEL=DEBUG


python3 -m pip install         \
  --no-index --find-links=dist \
  --only-binary :all:          \
  pagefind_python

python3 -c "$_get_executable"
python3 -m pagefind_python --help
echo "starting integration tests using system pagefind"
python3 src/tests/integration.py

# remove the externally installed pagefind binary
rm -rf node_modules output
export PATH="$_prev_path"
if command -v pagefind; then
  echo "dirty PATH: unexpectedly found pagefind"
  exit 1
fi

python3 -m pip install         \
  --no-index --find-links=dist \
  --only-binary :all:          \
  'pagefind_python[bin]'

python3 -c "$_get_executable"
python3 -m pagefind_python --help
echo "starting integration tests using pagefind_bin python module"
python3 src/tests/integration.py

