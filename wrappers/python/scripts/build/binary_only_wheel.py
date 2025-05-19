#!/usr/bin/env python3
# Adapted from https://github.com/ziglang/zig-pypi/blob/a0ca0d8b2d5104498f4eececff09ed2b1ede2d0b/make_wheels.py
# See also https://simonwillison.net/2022/May/23/bundling-binary-tools-in-python-wheels/
#
# Note that this script assumes that the relevant files are on disk and either
# the files hashes have been verified or we trust the files.
import argparse
import logging
from email.message import EmailMessage
from pathlib import Path
from typing import Any, Dict, List, Mapping, Optional, Tuple, Union
from zipfile import ZIP_DEFLATED, ZipInfo

import wheel  # type: ignore
import wheel.wheelfile  # type: ignore

from . import python_root, setup_logging

log = logging.getLogger(__name__)
# constants
HOMEPAGE = "https://pagefind.app"
REPO = "https://github.com/CloudCannon/pagefind/"
REQUIRED_PYTHON_VERSION = "~=3.9"


src_dir = python_root / "src" / "pagefind_python_bin"
assert src_dir.is_dir(), f"{src_dir} is not a directory"


# as of the time of writing, these are the supported platforms:
# See https://doc.rust-lang.org/nightly/rustc/platform-support.html
# wheel name format: {dist}-{version}(-{build})?-{python}-{abi}-{platform}.whl
# this dict helps look up the last part of the wheel name:      ^^^^^^^^^^
LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS = {
    # LLVM triple: Python platform
    # only the LLVM triples that are produced in CI are listed here; see
    # https://github.com/CloudCannon/pagefind/releases/latest
    # the python platform mapping is copied from zig-pypi's script.
    # See also: https://github.com/PyO3/maturin/blob/main/src/auditwheel/manylinux-policy.json
    # See also: https://github.com/PyO3/maturin/blob/main/src/auditwheel/musllinux-policy.json
    # TODO: check the python platforms are correct.
    "aarch64-apple-darwin": "macosx_12_0_arm64",
    "aarch64-unknown-linux-musl": "manylinux_2_17_aarch64.manylinux2014_aarch64.musllinux_1_1_aarch64",
    "x86_64-apple-darwin": "macosx_12_0_arm64",
    "x86_64-pc-windows-msvc": "win_amd64",
    "x86_64-unknown-freebsd": "freebsd_11_4_amd64",
    "x86_64-unknown-linux-musl": "manylinux_2_12_x86_64.manylinux2010_x86_64.musllinux_1_1_x86_64",
}


def as_zip_info(file: Path, *, alias: str) -> Tuple[ZipInfo, bytes]:
    zip_info = ZipInfo(alias or file.name, (1980, 1, 1, 0, 0, 0))
    zip_info.external_attr = file.stat().st_mode << 16
    with file.open("rb") as f:
        data = f.read()
    zip_info.file_size = len(data)
    return zip_info, data


class ReproducibleWheelFile(wheel.wheelfile.WheelFile):  # type: ignore
    def writestr(
        self,
        zip_info_or_arc_name: Union[ZipInfo, str],
        data: Any,
        *args: Any,
        **kwargs: Any,
    ) -> None:
        if isinstance(zip_info_or_arc_name, ZipInfo):
            zip_info = zip_info_or_arc_name
        else:
            assert isinstance(zip_info_or_arc_name, str)
            zip_info = ZipInfo(zip_info_or_arc_name)
            zip_info.file_size = len(data)
            zip_info.external_attr = 0o0644 << 16
            if zip_info_or_arc_name.endswith(".dist-info/RECORD"):
                zip_info.external_attr = 0o0664 << 16

        zip_info.compress_type = ZIP_DEFLATED
        zip_info.date_time = (1980, 1, 1, 0, 0, 0)
        zip_info.create_system = 3
        wheel.wheelfile.WheelFile.writestr(self, zip_info, data, *args, **kwargs)


def make_message(
    headers: Dict[str, Union[str, List[str]]],
    payload: Optional[Union[str, bytes]] = None,
) -> EmailMessage:
    msg = EmailMessage()
    for name, value in headers.items():
        if isinstance(value, list):
            for value_part in value:
                msg[name] = value_part
        else:
            msg[name] = value
    if payload:
        msg.set_payload(payload)
    return msg


def write_wheel_file(
    filename: Path,
    contents: Mapping[
        Union[str, ZipInfo], Union[str, bytes, EmailMessage, ZipInfo, Path]
    ],
) -> Path:
    with ReproducibleWheelFile(filename, "w") as wheel:
        for member_info, member_source in contents.items():
            if isinstance(member_source, str):
                data = member_source.encode("utf-8")
            elif isinstance(member_source, bytes):
                data = member_source
            elif isinstance(member_source, EmailMessage):
                data = member_source.as_bytes(
                    policy=member_source.policy.clone(linesep="\n"), unixfrom=False
                )
            elif isinstance(member_source, Path):
                assert type(member_info) is str
                member_info, data = as_zip_info(member_source, alias=member_info)
            else:
                raise ValueError(f"unexpected content: {type(member_source)}")
            wheel.writestr(member_info, data)
    return filename


def write_wheel(
    out_dir: Path,
    *,
    name: str,
    version: str,
    tag: str,
    metadata: Dict[str, Any],
    description: str,
    contents: Mapping[
        Union[str, ZipInfo], Union[str, bytes, EmailMessage, ZipInfo, Path]
    ],
) -> Path:
    wheel_name = f"{name}-{version}-{tag}.whl"
    dist_info = f"{name}-{version}.dist-info"
    return write_wheel_file(
        (out_dir / wheel_name),
        {
            **contents,
            f"{dist_info}/METADATA": make_message(
                {
                    # see https://packaging.python.org/en/latest/specifications/core-metadata/
                    "Metadata-Version": "2.1",
                    "Name": name,
                    "Version": version,
                    **metadata,
                },
                description,
            ),
            f"{dist_info}/WHEEL": make_message(
                {
                    "Wheel-Version": "1.0",
                    "Generator": "scripts/build/binary_only_wheel.py",
                    "Root-Is-Purelib": "false",  # see https://packaging.python.org/en/latest/specifications/binary-distribution-format/#what-s-the-deal-with-purelib-vs-platlib
                    "Tag": tag,
                }
            ),
        },
    )


def write_pagefind_bin_only_wheel(
    *,
    executable: Path,
    output_dir: Path,
    version: str,
    platform: str,
) -> Path:
    # FIXME: update when package support is stabilized
    name = "pagefind_bin"
    if "extended" in executable.name:
        name += "_extended"
    contents: Mapping[Union[str, ZipInfo], Path] = {
        f"{name}/__init__.py": (src_dir / "__init__.py"),
        f"{name}/__main__.py": (src_dir / "__main__.py"),
        f"{name}/{executable.name}": executable,
    }

    # Load in static files
    with (src_dir / "README.md").open() as f:
        description = f.read().replace("pagefind_bin", name)

    return write_wheel(
        output_dir,
        name=name,
        version=version,
        tag=f"py3-none-{platform}",
        metadata={
            "Summary": "Pagefind is a library for performant, low-bandwidth, fully static search.",
            "Description-Content-Type": "text/markdown",
            "License": "MIT",
            "Author": "CloudCannon",
            "Classifier": [
                "License :: OSI Approved :: MIT License",
                "Development Status :: 3 - Alpha",  # FIXME: update when package name stabilized
                "Intended Audience :: Developers",
            ],
            "Project-URL": [
                f"Homepage, {HOMEPAGE}",
                f"Source Code, {REPO}",
                f"Bug Tracker, {REPO}/issues",
            ],
            "Requires-Python": REQUIRED_PYTHON_VERSION,
        },
        description=description,
        contents=contents,
    )


def get_arg_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog=__file__, description="Repackage Pagefind binaries as Python wheels"
    )
    parser.add_argument(
        "--version",
        default=None,
        help="version to package",
    )
    parser.add_argument("--suffix", default="", help="wheel version suffix")
    parser.add_argument("--bin-path", help="path to the binary to embed", required=True)
    parser.add_argument(
        "--output-dir",
        default="dist/",
        help="Output directory in which to place the built wheel",
    )
    parser.add_argument(
        "--llvm-triple",
        required=True,
        choices=list(LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS.keys()),
        help="platform to build for",
    )
    return parser


def main() -> None:
    setup_logging()
    args = get_arg_parser().parse_args()
    platform = LLVM_TRIPLES_TO_PYTHON_WHEEL_PLATFORMS.get(args.llvm_triple)
    if platform is None:
        raise ValueError(f"Unsupported platform: {args.llvm_triple}")

    logging.getLogger(wheel.__name__).setLevel(logging.WARNING)
    write_pagefind_bin_only_wheel(
        output_dir=Path(args.output_dir),
        executable=Path(args.bin_path),
        version=args.version,
        platform=platform,
    )


if __name__ == "__main__":
    main()
