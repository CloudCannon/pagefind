import logging
import base64
from typing import Any, Dict, List, Optional, Sequence, TypedDict, cast

from ..service import PagefindService
from ..service.types import (
    InternalAddDirRequest,
    InternalAddFileRequest,
    InternalAddRecordRequest,
    InternalDeleteIndexRequest,
    InternalGetFilesRequest,
    InternalGetFilesResponse,
    InternalIndexedDirResponse,
    InternalIndexedFileResponse,
    InternalDecodedFile,
    InternalWriteFilesRequest,
)

log = logging.getLogger(__name__)


class IndexConfig(TypedDict, total=False):
    root_selector: Optional[str]
    """
    The root selector to use for the index.
    If not supplied, Pagefind will use the ``<html>`` tag.
    """
    exclude_selectors: Optional[Sequence[str]]
    """Extra element selectors that Pagefind should ignore when indexing."""
    force_language: Optional[str]
    """
    Ignores any detected languages and creates a single index for the entire site as the
    provided language. Expects an ISO 639-1 code, such as ``en`` or ``pt``.
    """
    verbose: Optional[bool]
    """
    Prints extra logging while indexing the site. Only affects the CLI, does not impact
    web-facing search.
    """
    logfile: Optional[str]
    """
    A path to a file to log indexing output to in addition to stdout.
    The file will be created if it doesn't exist and overwritten on each run.
    """
    keep_index_url: Optional[bool]
    """Whether to keep ``index.html`` at the end of search result paths.

    By default, a file at ``animals/cat/index.html`` will be given the URL
    ``/animals/cat/``. Setting this option to ``true`` will result in the URL
    ``/animals/cat/index.html``.
    """
    write_playground: Optional[bool]
    """When writing or outputting files, also write the Pagefind playground to /pagefind/playground/.

    Defaults to false, ensuring the playground isn't available on a live site.
    """
    output_path: Optional[str]
    """
    The folder to output the search bundle into, relative to the processed site.
    Defaults to ``pagefind``.
    """


class PagefindIndex:
    """Manages a Pagefind index.

    ``PagefindIndex`` operates as an async contextmanager.
    Entering the context starts a backing Pagefind service and creates an in-memory index in the backing service.
    Exiting the context writes the in-memory index to disk and then shuts down the backing Pagefind service.

    Each method of ``PagefindIndex`` that talks to the backing Pagefind service can raise errors.
    If an exception is is rased inside ``PagefindIndex``'s context, the context closes without writing the index files to disk.

    ``PagefindIndex`` optionally takes a configuration dictionary that can apply parts of the [Pagefind CLI config](/docs/config-options/). The options available at this level are:

    See the relevant documentation for these configuration options in the
    `Configuring the Pagefind CLI <https://pagefind.app/docs/config-options/>` documentation.
    """

    _service: Optional["PagefindService"] = None
    _index_id: Optional[int] = None
    _config: Optional[IndexConfig] = None
    """Note that config should be immutable."""

    def __init__(
        self,
        config: Optional[IndexConfig] = None,
        *,
        _service: Optional["PagefindService"] = None,
        _index_id: Optional[int] = None,
    ):
        self._service = _service
        self._index_id = _index_id
        self._config = config

    async def _start(self) -> "PagefindIndex":
        """Start the backing Pagefind service and create an in-memory index."""
        assert self._index_id is None
        assert self._service is None
        self._service = await PagefindService().launch()
        _index = await self._service.create_index(self._config)
        self._index_id = _index._index_id
        return self

    async def add_html_file(
        self,
        *,
        content: str,
        source_path: Optional[str] = None,
        url: Optional[str] = None,
    ) -> InternalIndexedFileResponse:
        """Add an HTML file to the index.

        :param content: The source HTML content of the file to be parsed.
        :param source_path: The source path of the HTML file would have on disk. \
            Must be a relative path, or an absolute path within the current working directory. \
            Pagefind will compute the result URL from this path.
        :param url: an explicit URL to use, instead of having Pagefind compute the \
            URL based on the source_path. If not supplied, source_path must be supplied.
        """
        assert self._service is not None
        assert self._index_id is not None
        result = await self._service.send(
            InternalAddFileRequest(
                type="AddFile",
                index_id=self._index_id,
                url=url,
                file_contents=content,
                file_path=source_path,
            )
        )
        assert result["type"] == "IndexedFile"
        return cast(InternalIndexedFileResponse, result)

    async def add_directory(
        self, path: str, *, glob: Optional[str] = None
    ) -> InternalIndexedDirResponse:
        """Indexes a directory from disk using the standard Pagefind indexing behaviour.

        This is equivalent to running the Pagefind binary with ``--site <dir>``.

        :param path: the path to the directory to index. If the `path` provided is relative, \
                it will be relative to the current working directory of your Python process.
        :param glob: a glob pattern to filter files in the directory. If not provided, all \
            files matching ``**.{html}`` are indexed. For more information on glob patterns, \
            see the `Wax patterns documentation <https://github.com/olson-sean-k/wax#patterns>`.
        """
        assert self._service is not None
        assert self._index_id is not None
        result = await self._service.send(
            InternalAddDirRequest(
                type="AddDir",
                index_id=self._index_id,
                path=path,
                glob=glob,
            )
        )
        assert result["type"] == "IndexedDir"
        return cast(InternalIndexedDirResponse, result)

    async def get_files(self) -> List[InternalDecodedFile]:
        """Get raw data of all files in the Pagefind index.

        WATCH OUT: this method emits all files. This can be a lot of data, and
        this amount of data can cause reading from the subprocess pipes to deadlock.

        STRICTLY PREFER calling ``self.write_files()``.
        """
        assert self._service is not None
        assert self._index_id is not None

        response = await self._service.send(
            InternalGetFilesRequest(type="GetFiles", index_id=self._index_id)
        )
        assert response["type"] == "GetFiles"
        files = cast(InternalGetFilesResponse, response)["files"]

        decoded_files = [
            {"path": file["path"], "content": base64.b64decode(file["content"])}
            for file in files
        ]

        return cast(List[InternalDecodedFile], decoded_files)

    async def delete_index(self) -> None:
        """
        Deletes the data for the given index from its backing Pagefind service.
        Doesn't affect any written files or data returned by ``get_files()``.
        """
        assert self._service is not None
        assert self._index_id is not None
        result = await self._service.send(
            InternalDeleteIndexRequest(type="DeleteIndex", index_id=self._index_id)
        )
        assert result["type"] == "DeleteIndex"
        self._index_id = None
        self._service = None

    async def add_custom_record(
        self,
        *,
        url: str,
        content: str,
        language: str,
        meta: Optional[Dict[str, str]] = None,
        filters: Optional[Dict[str, List[str]]] = None,
        sort: Optional[Dict[str, str]] = None,
    ) -> InternalIndexedFileResponse:
        """Add a direct record to the Pagefind index.

        This method is useful for adding non-HTML content to the search results.

        :param content: the raw content of this record.
        :param url: the output URL of this record. Pagefind will not alter this.
        :param language: ISO 639-1 code of the language this record is written in.
        :param meta: the metadata to attach to this record. Supplying a ``title`` is highly recommended.
        :param filters: the filters to attach to this record. Filters are used to group records together.
        :param sort: the sort keys to attach to this record.
        """
        assert self._service is not None
        assert self._index_id is not None
        result = await self._service.send(
            InternalAddRecordRequest(
                type="AddRecord",
                index_id=self._index_id,
                url=url,
                content=content,
                language=language,
                meta=meta,
                filters=filters,
                sort=sort,
            )
        )
        assert result["type"] == "IndexedFile"
        return cast(InternalIndexedFileResponse, result)

    async def write_files(self, output_path: Optional[str] = None) -> None:
        """Write the index files to disk.

        If you're using PagefindIndex as a context manager, there's no need to call this method:
        if no error occurred, closing the context automatically writes the index files to disk.

        :param output_path: a path to override the configured output path for the index.
        """
        assert self._service is not None
        assert self._index_id is not None
        if not output_path:
            if not self._config:
                output_path = None
            else:
                output_path = self._config.get("output_path")

        result = await self._service.send(
            InternalWriteFilesRequest(
                type="WriteFiles",
                index_id=self._index_id,
                output_path=output_path,
            )
        )
        assert result["type"] == "WriteFiles"

    async def __aenter__(self) -> "PagefindIndex":
        assert self._service is None
        assert self._index_id is None
        return await self._start()

    async def __aexit__(
        self,
        exc_type: Optional[Any],
        exc_value: Optional[Any],
        traceback: Optional[Any],
    ) -> None:
        if self._service is None:
            return
        if self._index_id is None:
            return
        if exc_type is None:
            await self.write_files()
        await self._service.close()
