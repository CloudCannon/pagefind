---
title: "Indexing content using the Python API"
nav_title: "Using the Python API"
nav_section: References
weight: 54 # slightly less weight than the node API
---

Pagefind provides an interface to the indexing binary as a Python package you can install and import.

There are situations where using this Python package is beneficial:
- Integrating Pagefind into an existing Python project, e.g. writing a plugin for a static site generator that can pass in-memory HTML files to Pagefind.
  Pagefind can also return the search index in-memory, to be hosted via the dev mode alongside the files.
- Users looking to index their site and augment that index with extra non-HTML pages can run a standard Pagefind crawl with [`add_directory`](#indexadd_directory) and augment it with [`add_custom_record`](#indexadd_custom_record).
- Users looking to use Pagefind's engine for searching miscellaneous content such as PDFs or subtitles, where [`add_custom_record`](#indexadd_custom_record) can be used to build the entire index from scratch.

## Installation

To install just the Python wrapper, and use a `pagefind` executable from your system:
```bash
python3 -m pip install 'pagefind'
```

To install the Python wrapper as well as the standard binary for your platform:
```bash
python3 -m pip install 'pagefind[bin]'
```

To install the Python wrapper as well as the extended binary for your platform:
```bash
python3 -m pip install 'pagefind[extended]'
```

## Example Usage

<!-- this example is copied verbatim from wrappers/python/src/tests/integration.py -->

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

All interactions with Pagefind are asynchronous, as they communicate with the native Pagefind binary in the background.

## PagefindIndex

`pagefind.index.PagefindIndex` manages a pagefind index.

`PagefindIndex` operates as an async contextmanager.
Entering the context starts a backing Pagefind service and creates an in-memory index in the backing service.
Exiting the context writes the in-memory index to disk and then shuts down the backing Pagefind service.

```py
from pagefind.index import PagefindIndex

async def main():
    async with PagefindIndex() as index: # open the index
        ... # update the index
    # the index is closed here and files are written to disk.
```

Each method of `PagefindIndex` that talks to the backing Pagefind service can raise errors.
If an error is is thrown inside `PagefindIndex`'s context, the context closes without writing the index files to disk.

```py
async def main():
    async with PagefindIndex() as index: # open the index
        await index.add_directory("./public")
        raise Exception("not today")
    # the index closes without writing anything to disk
```

`PagefindIndex` optionally takes a configuration dictionary that can apply parts of the [Pagefind CLI config](/docs/config-options/). The options available at this level are:

```py
from pagefind.index import PagefindIndex, IndexConfig
config = IndexConfig(
    root_selector="main",
    exclude_selectors="nav",
    force_language="en",
    verbose=True,
    logfile="index.log",
    keep_index_url=True,
    write_playground=True,
    output_path="./output",
)

async def main():
    async with PagefindIndex(config=config) as index:
        ...
```

See the relevant documentation for these configuration options in the [Configuring the Pagefind CLI](/docs/config-options/) documentation.

## index.add_directory

Indexes a directory from disk using the standard Pagefind indexing behaviour.
This is equivalent to running the Pagefind binary with `--site <dir>`.

```py
# Index all the HTML files in the public directory
indexed_dir = await index.add_directory("./public")
page_count: int = new_dir["page_count"]
```
If the `path` provided is relative, it will be relative to the current working directory of your Python process.

```py
# Index files in a directory matching a given glob pattern.
indexed_dir = await index.add_directory("./public", glob="**.{html}")
```

Optionally, a custom `glob` can be supplied which controls which files Pagefind will consume within the directory. The default is shown, and the `glob` option can be omitted entirely.
See [Wax patterns documentation](https://github.com/olson-sean-k/wax#patterns) for more details.

## index.add_html_file

Adds a virtual HTML file to the Pagefind index. Useful for files that don't exist on disk, for example a static site generator that is serving files from memory.

```py
html_content = (
    "<html lang='en'><body>"
    "  <h1>A Full HTML Document</h1>"
    "  <p> ... </p>"
    "</body></html>"
)

# Index a file as if Pagefind was indexing from disk
new_file = await index.add_html_file(
    content=html_content,
    source_path="other/example.html",
)

# Index HTML content, giving it a specific URL
new_file = await index.add_html_file(
    content=html_content,
    url="https://example.com",
)
```

The `source_path` should represent the path of this HTML file if it were to exist on disk. Pagefind will use this path to generate the URL. It should be relative, or absolute to a path within the current working directory.

Instead of `source_path`, a `url` may be supplied to explicitly set the URL of this search result.

The `content` should be the full HTML source, including the outer `<html> </html>` tags. This will be run through Pagefind's standard HTML indexing process, and should contain any required Pagefind attributes to control behaviour.

If successful, the `file` object is returned containing metadata about the completed indexing.

## index.add_custom_record
Adds a direct record to the Pagefind index.
Useful for adding non-HTML content to the search results.

```py
custom_record = await index.add_custom_record(
    url="/contact/",
    content=(
      "My raw content to be indexed for search. "
      "Will be lightly processed by Pagefind."
    ),
    language="en",
    meta={
        "title": "Contact",
        "category": "Landing Page"
    },
    filters={"tags": ["landing", "company"]},
    sort={"weight": "20"},
)

page_word_count: int = custom_record["page_word_count"]
page_url: str = custom_record["page_url"]
page_meta: dict[str, str] = custom_record["page_meta"]
```

The `url`, `content`, and `language` fields are all required. `language` should be an [ISO 639-1 code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).

`meta` is optional, and is strictly a flat object of keys to string values.
See the [Metadata documentation](https://pagefind.app/docs/metadata/) for semantics.

`filters` is optional, and is strictly a flat object of keys to arrays of string values.
See the [Filters documentation](https://pagefind.app/docs/filtering/) for semantics.

`sort` is optional, and is strictly a flat object of keys to string values.
See the [Sort documentation](https://pagefind.app/docs/sorts/) for semantics.
*When Pagefind is processing an index, number-like strings will be sorted numerically rather than alphabetically. As such, the value passed in should be `"20"` and not `20`*

If successful, the `file` object is returned containing metadata about the completed indexing.

## index.get_files

Get raw data of all files in the Pagefind index.
Useful for integrating a Pagefind index into the development mode of a static site generator and hosting these files yourself.

**WATCH OUT**: these files can be large enough to clog the pipe reading from the `pagefind` binary's subprocess, causing a deadlock.

```py
for file in (await index.get_files()):
    path: str = file["path"]
    content: str = file["content"]
    ...
```

## index.write_files

Calling `index.write_files()` writes the index files to disk, as they would be written when running the standard Pagefind binary directly.

Closing the `PagefindIndex`'s context automatically calls `index.write_files`, so calling this function is not necessary in normal operation.

Calling this function won't prevent files being written when the context closes, which may cause duplicate files to be written.
If calling this function manually, you probably want to also call `index.delete_index()`.

```py
config = IndexConfig(
    output_path="./public/pagefind",
)
async with PagefindIndex(config=config) as index:
    # ... add content to index

    # write files to the configured output path for the index:
    await index.write_files()

    # write files to a different output path:
    await index.write_files(output_path="./custom/pagefind")

    # prevent also writing files when closing the `PagefindIndex`:
    await index.delete_index()
```

The `output_path` option should contain the path to the desired Pagefind bundle directory. If relative, is relative to the current working directory of your Python process.

## index.delete_index

Deletes the data for the given index from its backing Pagefind service.
Doesn't affect any written files or data returned by `get_files()`.

```python
await index.delete_index()
```

Calling `index.get_files()` or `index.write_files()` doesn't consume the index, and further modifications can be made. In situations where many indexes are being created, the `delete_index` call helps clear out memory from a shared Pagefind binary service.

Reusing an `PagefindIndex` object after calling `index.delete_index()` will cause errors to be returned.

Not calling this method is fine — these indexes will be cleaned up when your `PagefindIndex`'s context closes, its backing Pagefind service closes, or your Python process exits.

## PagefindService

`PagefindService` manages a pagefind service running in a subprocess.

`PagefindService` operates as an async context manager: when the context is entered, the backing service starts, and when the context exits, the backing service shuts down.

```py
from pagefind.service import PagefindService

async def main():
    # or you can write
    service = await PagefindService().launch()
    ...
    await service.close()

    async with PagefindService() as service: # the service launches
        ...
    # the service closes
```

You should invoke `PagefindService` directly when you want to use the same backing service for many indexes:

```py
async with PagefindService() as service:
    default_index = await service.create_index()
    other_index = await service.create_index(
        config=IndexConfig(output_path="./search/nonstandard"),
    )
    await asyncio.gather(
        default_index.add_directory("./a"),
        other_index.add_directory("./b"),
    )
    await asyncio.gather(
        default_index.write_files(),
        other_index.write_files(),
    )
```
