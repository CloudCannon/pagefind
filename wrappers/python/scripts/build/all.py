import tarfile
import tempfile
from pathlib import Path
from typing import List

from . import dist_dir, setup_logging
from .binary_only_wheel import (
    LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS,
    write_pagefind_bin_only_wheel,
)
from .get_pagefind_release import download

__candidates = (
    "pagefind",
    "pagefind.exe",
    "pagefind_extended",
    "pagefind_extended.exe",
)


def find_bin(dir: Path) -> Path:
    for file in dir.iterdir():
        if file.is_file() and file.name in __candidates:
            return file
    raise FileNotFoundError(f"Could not find any of {__candidates} in {dir}")


def get_llvm_triple(tar_gz: Path) -> str:
    assert tar_gz.name.endswith(".tar.gz")
    # parse the llvm triple from the archive name
    llvm_triple = tar_gz.name
    llvm_triple = llvm_triple.removesuffix(".tar.gz")
    llvm_triple = llvm_triple.removeprefix(f"pagefind-{tag_name}-")
    llvm_triple = llvm_triple.removeprefix(f"pagefind_extended-{tag_name}-")
    return llvm_triple


def check_platforms(certified: List[Path]) -> None:
    for compressed_archive in certified:
        llvm_triple = get_llvm_triple(compressed_archive)
        platform = LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS.get(llvm_triple)
        if platform is None:
            raise ValueError(f"Unsupported platform: {llvm_triple}")


if __name__ == "__main__":
    setup_logging()
    certified, tag_name = download("latest", dry_run=False)
    # create a temp directory to hold the extracted binaries
    check_platforms(certified)
    dist_dir.mkdir(exist_ok=True)
    for tar_gz in certified:
        llvm_triple = get_llvm_triple(tar_gz)
        platform = LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS.get(llvm_triple)
        if platform is None:
            raise ValueError(f"Unsupported platform: {llvm_triple}")

        # FIXME: avoid writing the extracted bin to disk
        # unpack the tar.gz archive
        name = tar_gz.name.removesuffix(".tar.gz")
        with tempfile.TemporaryDirectory(prefix=name + "~") as _temp_dir:
            temp_dir = Path(_temp_dir)
            with tarfile.open(tar_gz, "r:gz") as tar:
                tar.extractall(_temp_dir)
            write_pagefind_bin_only_wheel(
                executable=find_bin(temp_dir),
                output_dir=dist_dir,
                version=tag_name.removeprefix("v"),
                platform=platform,
            )
