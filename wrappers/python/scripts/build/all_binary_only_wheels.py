"""A script that builds all the pagefind binary-only wheels."""

import logging
import re
import tarfile
import tempfile
from pathlib import Path
from typing import List, NamedTuple, Optional
from argparse import ArgumentParser

from . import dist_dir, setup_logging
from .binary_only_wheel import (
    LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS,
    write_pagefind_bin_only_wheel,
)
from .get_pagefind_release import download, find_bins
from .versioning import process_tag

__candidates = (
    "pagefind",
    "pagefind.exe",
    "pagefind_extended",
    "pagefind_extended.exe",
)

log = logging.getLogger(__name__)


def find_bin(dir: Path) -> Path:
    for file in dir.iterdir():
        log.debug("Checking for executable @ %s", (dir / file).absolute())
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
    log.debug(f"derived llvm_triple {llvm_triple} from {tar_gz.name}")
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


class Args(NamedTuple):
    dry_run: bool
    bin_dir: Optional[Path]
    tag: Optional[str]


def parse_args() -> Args:
    parser = ArgumentParser()
    parser.add_argument("--tag", type=str, default=None)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--bin-dir", type=Path, default=None)
    args = parser.parse_args()
    dry_run: bool = args.dry_run
    bin_dir: Optional[Path] = args.bin_dir
    tag: Optional[str] = args.tag
    return Args(dry_run=dry_run, bin_dir=bin_dir, tag=tag)


if __name__ == "__main__":
    dry_run, bin_dir, tag_name = parse_args()
    log.debug("args: dry_run=%s; bin_dir=%s; tag_name=%s", dry_run, bin_dir, tag_name)
    setup_logging()
    if bin_dir is None:
        log.debug("no bin_dir specified, downloading latest release")
        assert tag_name is None, f"--tag={tag_name} conflicts with downloading"
        certified, tag_name = download("latest", dry_run=False)
    else:
        certified = find_bins(bin_dir)
    if tag_name is None:
        raise ValueError("tag_name is None")
    assert re.match(
        r"^v\d+\.\d+\.\d+(-\w+\.?\d*)?", tag_name
    ), f"Invalid tag_name: {tag_name}"
    check_platforms(certified)

    if not dry_run:
        if dist_dir.exists():
            dist_dir.rmdir()
    dist_dir.mkdir(exist_ok=True)

    version = process_tag(tag_name)

    for tar_gz in certified:
        log.info("Processing %s", tar_gz)
        llvm_triple = get_llvm_triple(tar_gz)
        log.debug("llvm_triple=%s", llvm_triple)
        platform = LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS[llvm_triple]
        log.debug("platform=%s", platform)
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
                version=version,
                platform=platform,
            )
