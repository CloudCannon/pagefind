import json
import logging
import os
import sys
from pathlib import Path
from typing import Any, Dict, List, Tuple, Union
from urllib.request import urlopen

from . import vendor_dir
from .download_verification import verify_hashes

log = logging.getLogger(__name__)
logging.basicConfig(level=os.environ.get("PAGEFIND_PYTHON_LOG_LEVEL") or logging.INFO)

if sys.argv[1:]:
    version = sys.argv[1]
elif "PAGEFIND_VERSION" in os.environ:
    version = os.environ["PAGEFIND_VERSION"]
else:
    version = "latest"


def get_version_downloads(
    version: str,
) -> tuple[
    List[str],  # urls
    Dict[str, str],  # file: hash_file mapping
    str,  # tag_name
]:
    url = f"https://api.github.com/repos/CloudCannon/pagefind/releases/{version}"
    response = urlopen(url)
    data = json.loads(response.read())
    all_assets: Dict[str, Dict[str, Any]] = dict()
    for asset in data["assets"]:
        all_assets[asset["name"]] = asset
    tag_name = data["tag_name"]

    files: Dict[str, str] = dict()
    urls = []
    for name in all_assets:
        if name.endswith(".sha256"):
            name = name
            file_name = name.removesuffix(".sha256")
            files[file_name] = name
            urls.append(all_assets[name]["browser_download_url"])
            urls.append(all_assets[file_name]["browser_download_url"])

    return urls, files, tag_name


def find_bins(target_dir: Path) -> List[Path]:
    assert target_dir.is_dir()
    name_to_hash = {}
    for hash_file in vendor_dir.glob("*.sha256"):
        if (file := vendor_dir / hash_file.name.removesuffix(".sha256")).exists():
            name_to_hash[file.name] = hash_file.name
    return verify_hashes(target_dir, name_to_hash)


def download(
    version: Union[str, None] = None, dry_run: bool = True
) -> Tuple[List[Path], str]:
    urls, files, tag_name = get_version_downloads(version or "latest")
    target_dir = vendor_dir / tag_name  # TODO: rm -rf this to ensure it's clean
    if dry_run:
        log.info(f"would download {len(urls)} assets to {target_dir}")
        for url in urls:
            log.info(f"  - {url}")
        return [], tag_name
    target_dir.mkdir(parents=True, exist_ok=True)
    log.info(f"downloading {len(urls)} assets to {target_dir}")
    # TODO: parallelize downloads
    for i, url in enumerate(urls):
        name = url.split("/")[-1]
        with urlopen(url) as response:
            target_file = target_dir / name
            with target_file.open("wb") as local_artifact:
                local_artifact.write(response.read())
        log.info(f"{i}/{len(urls)} downloaded {name} to {target_file}")
    log.info(f"downloaded {len(urls)} assets to {target_dir}")
    with (target_dir / "files.json").open("w") as files_json:
        json.dump(files, files_json)
    certified = verify_hashes(target_dir, files)
    return certified, tag_name


if __name__ == "__main__":
    _urls, _files, tag_name = get_version_downloads("latest")
    version = tag_name.removeprefix("v")
    print(version)
