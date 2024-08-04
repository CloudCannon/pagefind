# `pagefind_python`
An async python API for the [pagefind](https://pagefind.app) binary.

## Installation
<!-- eventual?
```sh
python3 -m pip install 'pagefind[bin]'
python3 -m pagefind --help
```
-->
## Usage
<!--[[[cog
  print("```py")
  print(open('./src/tests/integration.py').read())
  print("```")
]]] -->
```py
import asyncio
import logging
from pagefind_python.index import PagefindIndex, IndexConfig

logging.basicConfig(level=logging.DEBUG)
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
        print(f"new_file={new_file}")
        print(f"new_record={new_record}")
        print(f"new_dir={new_dir}")

        files = await index.get_files()
        for f in files:
            print(f"files= {len(f['content']):10}B {f['path']}")


if __name__ == "__main__":
    asyncio.run(main())

```
<!-- [[[end]]] -->
