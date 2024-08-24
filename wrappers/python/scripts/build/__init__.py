import logging
import os
from pathlib import Path

this_file = Path(__file__)
this_dir = Path(__file__).parent
python_root = this_dir.parent.parent.resolve().absolute()
dist_dir = python_root / "dist"
vendor_dir = python_root / "vendor"


def setup_logging() -> None:
    logging.basicConfig(
        level=os.environ.get("PAGEFIND_PYTHON_LOG_LEVEL") or logging.INFO
    )
