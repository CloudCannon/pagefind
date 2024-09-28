#!/usr/bin/env bash
set -eu
export PATH="$PWD/target/release:$PATH"
export PYTHONPATH="$PWD/wrappers/python/src:${PYTHONPATH:-}"
# ^ ensure `import pagefind` imports wrappers/python/src/pagefind/__init__.py
export PAGEFIND_PYTHON_LOG_LEVEL=DEBUG

cd wrappers/python
python -c 'import sys; print("pythonpath\n  - " + "\n  - ".join(sys.path))'
python -c '
import logging
import os
from pagefind.service import get_executable

logging.basicConfig(level=os.environ.get("PAGEFIND_PYTHON_LOG_LEVEL", "INFO")) 
print(get_executable())
'

python -m pagefind --help
python src/tests/integration.py
