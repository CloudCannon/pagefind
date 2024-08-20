import json
import logging
import os
import sys
from pathlib import Path
from typing import Any, Dict, List, Tuple, Union
from urllib.request import urlopen

from . import vendor_dir, upstream_version_file
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


def download(
    version: Union[str, None] = None, dry_run: bool = True
) -> Tuple[List[Path], str]:
    urls, files, tag_name = get_version_downloads(version or "latest")
    target_dir = vendor_dir / tag_name
    if dry_run:
        log.info(f"would download {len(urls)} assets to {target_dir}")
        for url in urls:
            log.info(f"  - {url}")
        return [], tag_name
    target_dir.mkdir(parents=True, exist_ok=True)
    log.info(f"downloading {len(urls)} assets to {target_dir}")
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
    with upstream_version_file.open("w") as f:
        f.write(version + "\n")
        # to avoid IDEs adding a trailing newline and causing a diff, we add one here.
    print(version)
