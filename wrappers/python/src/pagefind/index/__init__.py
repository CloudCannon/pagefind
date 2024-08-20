import logging
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
    InternalSyntheticFile,
    InternalWriteFilesRequest,
)

log = logging.getLogger(__name__)


class IndexConfig(TypedDict, total=False):
    root_selector: Optional[str]
    exclude_selectors: Optional[Sequence[str]]
    force_language: Optional[str]
    verbose: Optional[bool]
    logfile: Optional[str]
    keep_index_url: Optional[bool]
    output_path: Optional[str]


class PagefindIndex:
    _service: Optional["PagefindService"] = None
    _index_id: Optional[int] = None
    config: Optional[IndexConfig] = None
    """Note that config is immutable after initialization."""

    def __init__(
        self,
        config: Optional[IndexConfig] = None,
        *,
        _service: Optional["PagefindService"] = None,
        _index_id: Optional[int] = None,
        # TODO: cache config
    ):
        self._service = _service
        self._index_id = _index_id
        self.config = config

    async def _start(self) -> "PagefindIndex":
        assert self._index_id is None
        assert self._service is None
        self._service = await PagefindService().launch()
        _index = await self._service.create_index(self.config)
        self._index_id = _index._index_id
        return self

    async def add_html_file(
        self,
        *,
        content: str,
        source_path: Optional[str] = None,
        url: Optional[str] = None,
    ) -> InternalIndexedFileResponse:
        """
        ARGS:
        content: The source HTML content of the file to be parsed.
        source_path: The source path of the HTML file if it were to exist on disk. \
            Must be a relative path, or an absolute path within the current working directory. \
            Pagefind will compute the result URL from this path.
        url: an explicit URL to use, instead of having Pagefind compute the URL \
            based on the source_path. If not supplied, source_path must be supplied.
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

    async def get_files(self) -> List[InternalSyntheticFile]:
        """
        WATCH OUT: this method emits all files. This can be a lot of data, and
        this amount of data can cause reading from the subprocess pipes to deadlock.

        STRICTLY PREFER calling `self.write_files()`.
        """
        assert self._service is not None
        assert self._index_id is not None

        response = await self._service.send(
            InternalGetFilesRequest(type="GetFiles", index_id=self._index_id)
        )
        assert response["type"] == "GetFiles"
        result = cast(InternalGetFilesResponse, response)["files"]
        return result

    async def delete_index(self) -> None:
        assert self._service is not None
        assert self._index_id is not None
        result = await self._service.send(
            InternalDeleteIndexRequest(type="DeleteIndex", index_id=self._index_id)
        )
        assert result["type"] == "DeletedIndex"
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
        """
        ARGS:
        content: the raw content of this record.
        url: the output URL of this record. Pagefind will not alter this.
        language: ISO 639-1 code of the language this record is written in.
        meta: the metadata to attach to this record. Supplying a `title` is highly recommended.
        filters: the filters to attach to this record. Filters are used to group records together.
        sort: the sort keys to attach to this record.
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

    async def write_files(self) -> None:
        assert self._service is not None
        assert self._index_id is not None
        if not self.config:
            output_path = None
        else:
            output_path = self.config.get("output_path")

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
        assert self._service is not None
        assert self._index_id is not None
        if exc_type is None:
            await self.write_files()
        await self._service.close()
