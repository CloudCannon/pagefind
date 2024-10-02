import logging
import os
import platform
import sys
from pathlib import Path
from typing import List

__all__ = ["get_executable", "cli"]


this_dir = Path(__file__).parent
log = logging.getLogger(__name__)


def get_candidate_paths() -> List[Path]:
    names = ["pagefind_extended", "pagefind"]
    extensions = [""]
    if platform.system().lower() == "windows":
        extensions.append(".exe")
    bin_names = [n + ext for n in names for ext in extensions]
    paths = [this_dir / bin for bin in bin_names]
    return paths


def get_executable() -> Path:
    candidates = get_candidate_paths()
    for candidate in candidates:
        if candidate.exists():
            log.debug(f"{candidate} found")
            if candidate.is_file():
                return candidate
            else:
                raise FileNotFoundError(f"{candidate} is not a file")
        else:
            log.debug(f"{candidate} not found")
    raise FileNotFoundError(f"Could not find any of {candidates}")


def cli() -> None:
    bin = str(get_executable().absolute())
    argv = [bin, *sys.argv[1:]]
    if os.name == "posix":
        os.execv(bin, argv)
    else:
        import subprocess

        sys.exit(subprocess.call(argv))
