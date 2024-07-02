from typing import Dict, List, Optional, NamedTuple, cast

from ..service.types import (
    InternalAddFileRequest,
    InternalAddRecordRequest,
    InternalDeleteIndexRequest,
    InternalGetFilesRequest,
    InternalIndexedFileResponse,
    InternalAddDirRequest,
    InternalIndexedDirResponse,
)
from ..service import Service


class HtmlFile(NamedTuple):
    content: str
    """The source HTML content of the file to be parsed."""

    source_path: Optional[str] = None
    """
    The source path of the HTML file if it were to exist on disk.
    Must be a relative path, or an absolute path within the current working directory.
    Pagefind will compute the result URL from this path.

    If not supplied, url must be supplied.

    @example "about/index.html"
    @example "/Users/user/Documents/site/about/index.html"
    """

    url: Optional[str] = None
    """
    An explicit URL to use, instead of having Pagefind
    compute the URL based on the sourcePath.

    If not supplied, source_path must be supplied.

    @example "/about/"
    """


class CustomRecord(NamedTuple):
    """
    The data required for Pagefind to index a custom record that isn't backed by an HTML file
    """

    url: str
    """The output URL of this record. Pagefind will not alter this."""

    content: str
    """The raw content of this record"""

    language: str
    """What language is this record written in. Multiple languages will be split into separate indexes. Expects an ISO 639-1 code."""

    meta: Optional[Dict[str, str]] = None
    """The metadata to attach to this record. Supplying a `title` is highly recommended."""

    filters: Optional[Dict[str, List[str]]] = None
    """The filters to attach to this record. Filters are used to group records together."""

    sort: Optional[Dict[str, str]] = None
    """The sort keys to attach to this record."""


class SiteDirectory(NamedTuple):
    path: str
    """The path to the directory to index. If relative, it's relative to the current working directory."""
    glob: Optional[str] = None
    """Optionally, a custom glob to evaluate for finding files. Default to all HTML files."""


class PagefindIndex:
    def __init__(self, service: Service, index_id: int):
        self._service = service
        self.index_id = index_id

    async def add_html_file(self, html_file: HtmlFile) -> InternalIndexedFileResponse:
        result = await self._service.send(
            InternalAddFileRequest(
                type="AddFile",
                index_id=self.index_id,
                url=html_file.url,
                file_contents=html_file.content,
                file_path=html_file.source_path,
            )
        )
        assert result["type"] == "IndexedFile"
        return cast(InternalIndexedFileResponse, result)

    async def add_directory(
        self, directory: SiteDirectory
    ) -> InternalIndexedDirResponse:
        result = await self._service.send(
            InternalAddDirRequest(
                type="AddDir",
                index_id=self.index_id,
                path=directory.path,
                glob=directory.glob,
            )
        )
        assert result["type"] == "IndexedDir"
        return cast(InternalIndexedDirResponse, result)

    async def get_files(self):
        result = await self._service.send(
            InternalGetFilesRequest(type="GetFiles", index_id=self.index_id)
        )
        assert result["type"] == "GetFiles"
        return cast(List[InternalIndexedFileResponse], result)

    async def delete_index(self):
        result = await self._service.send(
            InternalDeleteIndexRequest(type="DeleteIndex", index_id=self.index_id)
        )
        assert result["type"] == "DeletedIndex"

    async def add_custom_record(
        self, record: CustomRecord
    ) -> InternalIndexedFileResponse:
        result = await self._service.send(
            InternalAddRecordRequest(
                type="AddRecord",
                index_id=self.index_id,
                url=record.url,
                content=record.content,
                language=record.language,
                meta=record.meta,
                filters=record.filters,
                sort=record.sort,
            )
        )
        assert result["type"] == "IndexedFile"
        return cast(InternalIndexedFileResponse, result)
