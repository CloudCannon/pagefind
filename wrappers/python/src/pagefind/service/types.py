from enum import Enum
from typing import Dict, List, Literal, Optional, Sequence, TypedDict, Union


class InternalRequestType(Enum):
    NEW_INDEX = "NewIndex"
    ADD_FILE = "AddFile"
    ADD_RECORD = "AddRecord"
    ADD_DIR = "AddDir"
    WRITE_FILES = "WriteFiles"
    GET_FILES = "GetFiles"
    DELETE_INDEX = "DeleteIndex"


class InternalPagefindServiceConfig(TypedDict, total=False):
    # FIXME: document
    root_selector: Optional[str]
    exclude_selectors: Optional[Sequence[str]]
    force_language: Optional[str]
    verbose: Optional[bool]
    logfile: Optional[str]
    keep_index_url: Optional[bool]
    write_playground: Optional[bool]


class InternalNewIndexRequest(TypedDict):
    type: Literal["NewIndex"]
    config: Optional[InternalPagefindServiceConfig]


class InternalAddFileRequest(TypedDict):
    type: Literal["AddFile"]
    index_id: int
    """index_id must be positive."""
    file_path: Optional[str]
    url: Optional[str]
    file_contents: str


class InternalAddRecordRequest(TypedDict):
    type: Literal["AddRecord"]
    index_id: int
    """index_id must be positive."""
    url: str
    content: str
    language: str
    meta: Optional[Dict[str, str]]
    filters: Optional[Dict[str, List[str]]]
    sort: Optional[Dict[str, str]]


class InternalAddDirRequest(TypedDict, total=False):
    type: Literal["AddDir"]
    index_id: int
    path: str  # TODO: support Path
    glob: Optional[str]


class InternalWriteFilesRequest(TypedDict, total=False):
    type: Literal["WriteFiles"]
    index_id: int
    """index_id must be positive."""
    output_path: Optional[str]


class InternalGetFilesRequest(TypedDict):
    type: Literal["GetFiles"]
    index_id: int
    """index_id must be positive."""


class InternalDeleteIndexRequest(TypedDict):
    type: Literal["DeleteIndex"]
    index_id: int
    """index_id must be positive."""


InternalRequestPayload = Union[
    InternalNewIndexRequest,
    InternalAddFileRequest,
    InternalAddRecordRequest,
    InternalAddDirRequest,
    InternalWriteFilesRequest,
    InternalGetFilesRequest,
    InternalDeleteIndexRequest,
]


class InternalServiceRequest(TypedDict):
    message_id: Optional[int]
    payload: InternalRequestPayload


class InternalResponseType(Enum):
    NEW_INDEX = "NewIndex"
    INDEXED_FILE = "IndexedFile"
    INDEXED_DIR = "IndexedDir"
    WRITE_FILES = "WriteFiles"
    GET_FILES = "GetFiles"
    DELETE_INDEX = "DeleteIndex"
    ERROR = "Error"


class InternalResponseError(TypedDict):
    type: Literal["Error"]
    message: str
    original_message: Optional[str]


class InternalNewIndexResponse(TypedDict):
    type: Literal["NewIndex"]
    index_id: int


class InternalIndexedFileResponse(TypedDict):
    type: Literal["IndexedFile"]
    page_word_count: int
    page_url: str
    page_meta: Dict[str, str]


class InternalIndexedDirResponse(TypedDict):
    type: str
    page_count: int


class InternalWriteFilesResponse(TypedDict):
    type: Literal["IndexedFile"]
    output_path: str


class InternalSyntheticFile(TypedDict):
    path: str
    content: str


class InternalDecodedFile(TypedDict):
    path: str
    content: bytes


class InternalGetFilesResponse(TypedDict):
    type: Literal["GetFiles"]
    files: List[InternalSyntheticFile]


class InternalDeleteIndexResponse(TypedDict):
    type: Literal["DeleteIndex"]


InternalResponsePayload = Union[
    InternalNewIndexResponse,
    InternalIndexedFileResponse,
    InternalIndexedDirResponse,
    InternalWriteFilesResponse,
    InternalGetFilesResponse,
    InternalDeleteIndexResponse,
]


class InternalServiceResponse(TypedDict):
    message_id: Optional[int]
    payload: Union[InternalResponsePayload, InternalResponseError]


class InternalResponseCallback(TypedDict, total=False):
    exception: Optional[Exception]
    err: Optional[InternalResponseError]
    result: Optional[InternalResponsePayload]
