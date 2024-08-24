#!/usr/bin/env bash
set -eu
export PATH="$PWD/target/release:$PATH"
export PYTHONPATH="$PWD/wrappers/python/src:$PYTHONPATH"
export PAGEFIND_PYTHON_LOG_LEVEL=DEBUG

cd wrappers/python
python3 -c 'import sys; print("pythonpath"\n" + "\n".join(sys.path))'
python3 -c '
import logging
import os
from pagefind.service import get_executable

logging.basicConfig(level=os.environ.get("PAGEFIND_PYTHON_LOG_LEVEL", "INFO")) 
print(get_executable())
'

python3 -m pagefind --help
python3 src/tests/integration.py
