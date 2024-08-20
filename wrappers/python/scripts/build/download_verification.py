import hashlib
import logging
from pathlib import Path
from typing import Dict, List

log = logging.getLogger(__name__)


def verify_hashes(version_vendor_dir: Path, name_to_hash: Dict[str, str]) -> List[Path]:
    verified = []
    assert (
        version_vendor_dir.is_dir()
    ), f"{version_vendor_dir} is not a directory; pwd={Path.cwd()}"
    for name, hash_name in name_to_hash.items():
        to_verify = version_vendor_dir / name
        hash_file = version_vendor_dir / hash_name

        assert hash_name.endswith(".sha256"), f"{hash_name} does not end with .sha256"
        assert to_verify.is_file(), f"{to_verify} is not a file"
        assert hash_file.is_file(), f"{hash_file} is not a file"

        with hash_file.open() as f:
            expected_hash, expected_name = f.read().strip().split()
            expected_name = expected_name.removeprefix("*")
        with to_verify.open("rb") as f:
            actual_hash = hashlib.sha256(f.read()).hexdigest()
        if name != expected_name:
            raise ValueError(
                f"name mismatch: actual {to_verify.name} != expected {expected_name}"
            )
        if actual_hash != expected_hash:
            raise ValueError(
                f"hash mismatch: actual {actual_hash} != expected {expected_hash}"
            )
        else:
            verified.append(to_verify)
            log.info(f"hash {actual_hash} verified for {name}")
    return verified
