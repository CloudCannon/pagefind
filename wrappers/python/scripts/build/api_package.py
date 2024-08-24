# HACK: This script is a hack to build the API package without using poetry to lock the
# optional dependencies. It might be preferable to use setuptools directly rather than
# work around poetry.

from . import python_root, setup_logging
import subprocess
import re
import os

pyproject_toml = python_root / "pyproject.toml"


def main() -> None:
    version = os.environ.get("PAGEFIND_VERSION")
    if version is None:
        version = "1"
    original = pyproject_toml.read_text()
    temp = ""
    for line in original.splitlines():
        if line.endswith("#!!opt"):
            line = line.removeprefix("# ") + "\n"
            line = re.sub(r'version = "[^"]+"', f'version = "~={version}"', line)
        temp += line + "\n"
    with pyproject_toml.open("w") as f:
        f.write(temp)
    subprocess.run(["poetry", "build"], check=True)
    with pyproject_toml.open("w") as f:
        f.write(original)


if __name__ == "__main__":
    setup_logging()
    main()
