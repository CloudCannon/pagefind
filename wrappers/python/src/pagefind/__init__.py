#!/usr/bin/env python3
# assume the python version is >= 3.9, which is the oldest LTS version with
# more 2 months of life as of the time of writing, 2024-08-18


# https://docs.python.org/3/reference/datamodel.html#async-context-managers
# https://docs.python.org/3/library/contextlib.html#contextlib.asynccontextmanager

# [[[cog
# import tomllib # ok since the development environment must be python >= 3.11
# from pathlib import Path
# pyproject = Path("pyproject.toml") # note the CWD is the project root
# assert pyproject.is_file(), f"expected {pyproject.absolute()} to be a file"
# version = tomllib.load(pyproject.open("rb"))["tool"]["poetry"]["version"]
# print(f'__version__ = "{version}"')
# ]]]
__version__ = "0.0.0a0"
# [[[end]]]
