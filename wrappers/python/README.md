# `pagefind`
An async python API for the [pagefind](https://pagefind.app) binary.

## Installation

```sh
python3 -m pip install 'pagefind[bin]'
python3 -m pagefind --help
```

## Usage
<!--[[[cog
  print("```py")
  print(open('./src/tests/integration.py').read())
  print("```")
]]] -->
```py
import asyncio
import json
import logging
import os
from pagefind.index import PagefindIndex, IndexConfig

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "INFO"))
log = logging.getLogger(__name__)
html_content = (
    "<html>"
    "  <body>"
    "    <main>"
    "      <h1>Example HTML</h1>"
    "      <p>This is an example HTML page.</p>"
    "    </main>"
    "  </body>"
    "</html>"
)


def prefix(pre: str, s: str) -> str:
    return pre + s.replace("\n", f"\n{pre}")


async def main():
    config = IndexConfig(
        root_selector="main", logfile="index.log", output_path="./output", verbose=True
    )
    async with PagefindIndex(config=config) as index:
        log.debug("opened index")
        new_file, new_record, new_dir = await asyncio.gather(
            index.add_html_file(
                content=html_content,
                url="https://example.com",
                source_path="other/example.html",
            ),
            index.add_custom_record(
                url="/elephants/",
                content="Some testing content regarding elephants",
                language="en",
                meta={"title": "Elephants"},
            ),
            index.add_directory("./public"),
        )
        print(prefix("new_file    ", json.dumps(new_file, indent=2)))
        print(prefix("new_record  ", json.dumps(new_record, indent=2)))
        print(prefix("new_dir     ", json.dumps(new_dir, indent=2)))

        files = await index.get_files()
        for file in files:
            print(prefix("files", f"{len(file['content']):10}B {file['path']}"))


if __name__ == "__main__":
    asyncio.run(main())

```
<!-- [[[end]]] -->
