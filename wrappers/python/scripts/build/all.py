import os
import tarfile
import tempfile
from pathlib import Path
from typing import List, Optional
from argparse import ArgumentParser

from . import dist_dir, setup_logging
from .binary_only_wheel import (
    LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS,
    write_pagefind_bin_only_wheel,
)
from .get_pagefind_release import download, find_bins

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
    unsupported = []
    for compressed_archive in certified:
        llvm_triple = get_llvm_triple(compressed_archive)
        platform = LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS.get(llvm_triple)
        if platform is None:
            unsupported.append(llvm_triple)
    if unsupported:
        err_message = "Unsupported platforms:\n" + "\n".join(sorted(unsupported))
        raise ValueError(err_message)


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("DIR", type=Path, default=None, nargs="?")
    args = parser.parse_args()
    dry_run: bool = args.dry_run
    bin_dir: Optional[Path] = args.DIR
    return dry_run, bin_dir


if __name__ == "__main__":
    dry_run, bin_dir = parse_args()
    setup_logging()
    if bin_dir is None:
        certified, tag_name = download("latest", dry_run=False)
    else:
        if (tag_name := os.environ.get("GIT_VERSION")) is None:
            raise KeyError("Missing DIR argument and GIT_VERSION environment variable")
        certified = find_bins(bin_dir)
    check_platforms(certified)

    if not dry_run:
        dist_dir.rmdir()
    dist_dir.mkdir(exist_ok=True)

    for tar_gz in certified:
        llvm_triple = get_llvm_triple(tar_gz)
        platform = LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS[llvm_triple]
        if platform is None:
            raise ValueError(f"Unsupported platform: {llvm_triple}")
        # TODO: avoid writing the extracted bin to disk
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
