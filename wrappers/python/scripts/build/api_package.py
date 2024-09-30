# HACK: This script is a hack to build the API package without using poetry to lock the
# optional dependencies. It might be preferable to use setuptools directly rather than
# work around poetry.

import logging
import subprocess
import re
from argparse import ArgumentParser

from . import python_root, setup_logging
from .versioning import process_tag

pyproject_toml = python_root / "pyproject.toml"

cli = ArgumentParser()
cli.add_argument("--dry-run", action="store_true")
cli.add_argument("--tag", required=True, help="The version to build.")
log = logging.getLogger(__name__)

def main() -> None:
    setup_logging()
    args = cli.parse_args()
    tag: str = args.tag
    dry_run: bool = args.dry_run
    log.debug("args: dry_run=%s; tag=%s", dry_run, tag)
    version = process_tag(tag)

    log.info("Building version %s", version)
    # create a pyproject.toml with updated versions
    original = pyproject_toml.read_text()
    temp = ""
    for line in original.splitlines():
        if "0.0.0a0" in line:
            line = line.replace("0.0.0a0", version)
            log.debug("patching: %s", line)
        elif line.endswith("#!!opt"):
            line = line.removeprefix("# ").removesuffix("#!!opt")
            line = re.sub(r'version = "[^"]+"', f'version = "~={version}"', line)
            log.debug("patching: %s", line)
        temp += line + "\n"
    log.debug("patched pyproject.toml", extra={"updated": temp})

    if dry_run:
        return

    with pyproject_toml.open("w") as f:
        f.write(temp)
        log.debug("wrote patched pyproject.toml")

    log.info("Building API package")
    subprocess.run(["poetry", "build"], check=True)
    with pyproject_toml.open("w") as f:  # restore the original
        f.write(original)
        log.debug("restored original pyproject.toml")


if __name__ == "__main__":
    setup_logging()
    main()
