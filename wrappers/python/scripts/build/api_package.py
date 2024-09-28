# HACK: This script is a hack to build the API package without using poetry to lock the
# optional dependencies. It might be preferable to use setuptools directly rather than
# work around poetry.

import logging
import subprocess
import re
from argparse import ArgumentParser

from . import python_root, setup_logging

pyproject_toml = python_root / "pyproject.toml"

cli = ArgumentParser()
cli.add_argument("--dry-run", action="store_true")
cli.add_argument("--tag", required=True, help="The version to build.")
log = logging.getLogger(__name__)


def process_tag(tag: str) -> str:
    """Convert a git tag to a version string compliant with PEP 440.
    See https://peps.python.org/pep-0440/#public-version-identifiers
    """
    pattern = (
        # note that this pattern accepts a superset of the tagging pattern used
        # in this repository.
        r"^v(?P<major>\d+)"
        r"\.(?P<minor>\d+)"
        r"\.(?P<patch>\d+)"
        r"(-"
        r"(?P<prerelease_kind>alpha|beta|rc)"
        r"\.?(?P<prerelease_number>\d+)"
        ")?"
    )
    parts = re.match(pattern, tag)
    if parts is None:
        raise ValueError(f"Invalid tag: `{tag}` does not match pattern `{pattern}`")
    major = int(parts["major"])
    minor = int(parts["minor"])
    patch = int(parts["patch"])
    suffix = ""

    if (prerelease_kind := parts["prerelease_kind"]) is not None:
        if prerelease_kind == "rc":
            suffix = "rc"
        elif prerelease_kind.startswith("alpha"):
            suffix = "a"
        elif prerelease_kind.startswith("beta"):
            suffix = "b"
    if (prerelease_number := parts["prerelease_number"]) is not None:
        suffix += str(int(prerelease_number))

    return f"{major}.{minor}.{patch}{suffix}"


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
