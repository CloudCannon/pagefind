import os

from pathlib import Path
import json
from contextlib import AbstractAsyncContextManager
from typing import Any, Dict, Optional, cast, TYPE_CHECKING
import asyncio
import base64
import logging
import shutil

from .types import (
    InternalNewIndexRequest,
    InternalNewIndexResponse,
    InternalRequestPayload,
    InternalResponsePayload,
    InternalResponseError,
    InternalServiceRequest,
    InternalServiceResponse,
    InternalResponseType,
)

if TYPE_CHECKING:
    from ..index import IndexConfig, PagefindIndex

log = logging.getLogger(__name__)


def _must_find_binary() -> Path:
    try:
        from pagefind_bin_extended import get_executable  # type: ignore

        executable: Path = get_executable()
        log.debug(f"using {executable}")
        return executable
    except ImportError:
        log.debug("unable to import pagefind_bin_extended")

    try:
        from pagefind_bin import get_executable  # type: ignore

        executable: Path = get_executable()
        log.debug(f"using {executable}")
        return executable
    except ImportError:
        log.debug("unable to import pagefind_bin")

    exe: Optional[str] = shutil.which("pagefind_extended") or shutil.which("pagefind")
    if exe is None:
        raise FileNotFoundError("Could not find pagefind binary")
    else:
        return Path(exe)


def _encode(req: InternalServiceRequest) -> bytes:
    return base64.b64encode(json.dumps(req).encode("utf-8"))


class PagefindService(AbstractAsyncContextManager["PagefindService"]):
    _bin: Path
    _backend: asyncio.subprocess.Process
    _message_id: int = 0
    _responses: Dict[int, asyncio.Future[InternalResponsePayload]]
    _loop: asyncio.AbstractEventLoop
    _poll_task: asyncio.Task[None]

    # _messages
    def __init__(self):
        self._loop = asyncio.get_event_loop()
        self._bin = _must_find_binary()
        self._responses = dict()

    async def launch(self) -> "PagefindService":
        log.debug(f"launching {self._bin}")
        # TODO: detach process on windows?
        # creation_flags: int = 0
        # if platform.system().lower() == "windows":
        #     creation_flags = subprocess.CREATE_NO_WINDOW | subprocess.CREATE_DETACHED
        self._backend = await asyncio.create_subprocess_exec(
            self._bin,
            "--service",
            "--verbose",
            cwd=os.getcwd(),
            stdin=asyncio.subprocess.PIPE,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.STDOUT,
            limit=2**21,  # <- 2MiB
            # anything less and the _wait_for_responses loop will hang
            # due to the stdout pipes deadlocking due to the buffer filling up
        )
        log.debug(f"launched {self._bin}: {self._backend}.")
        log.debug("polling for responses")
        self._poll_task = self._loop.create_task(self._wait_for_responses())
        log.debug(f"polling task created: {self._poll_task}")
        return self

    async def send(self, payload: InternalRequestPayload) -> InternalResponsePayload:
        self._message_id += 1
        message_id = self._message_id
        if (_ := self._responses.get(message_id)) is not None:
            raise KeyError(f"message_id {message_id} already in use")
        else:
            future: asyncio.Future[InternalResponsePayload] = self._loop.create_future()
            self._responses[message_id] = future
        # FIXME: check stdin not none?
        if self._backend.stdin is None:
            # restart the backend
            log.debug("restarting backend")
            await self.launch()
            log.debug("backend restarted")
        assert self._backend.stdin is not None
        req = InternalServiceRequest(message_id=message_id, payload=payload)
        log.debug(f"sending request: {req}")
        self._backend.stdin.write(_encode(req) + b",")
        # backend waits for a comma before responding
        await self._backend.stdin.drain()
        log.debug(f"request sent: {req}")
        result = await future
        log.debug(f"received response: {result}")
        return result

    async def _wait_for_responses(self) -> None:
        """
        Poll the subprocess's stdout for responses
        """
        while True:
            await asyncio.sleep(0.1)
            assert self._backend.stdout is not None
            log.debug("checking for data")
            output = await self._backend.stdout.readuntil(b",")
            if len(output) <= 100:
                log.debug(f"received data: {output}")
            else:
                log.debug(
                    f"received data: {output[:30]}...{len(output) - 40}B...{output[-10:]}"
                )
            if (resp := json.loads(base64.b64decode(output[:-1]))) is None:
                continue
            resp = cast(InternalServiceResponse, resp)
            if (message_id := resp.get("message_id")) is not None:
                log.debug(f"received response for message {message_id}")
                assert (
                    self._message_id >= message_id
                ), f"message_id out of order: incoming {message_id} > current: {self._message_id}"
                if (future := self._responses.get(message_id)) is not None:
                    log.debug(f"resolving future for message {message_id}")
                    payload = resp["payload"]
                    if payload["type"] == InternalResponseType.ERROR.value:
                        exc = cast(InternalResponseError, payload)
                        future.set_exception(
                            Exception(exc["message"], exc.get("original_message"))
                        )
                    else:
                        future.set_result(cast(InternalResponsePayload, payload))
                else:
                    log.debug(f"no receiving future for message {message_id}")
                    # FIXME: figure out how to surface the error
                    payload = cast(InternalResponseError, resp["payload"])
                    # assert (
                    #     payload["type"] == InternalResponseType.ERROR.value
                    # ), f"unexpected message type: {payload['type']}"

    async def close(self):
        # wait for all _responses to be resolved
        await asyncio.gather(*self._responses.values())  # IDEA: add timeout?
        self._poll_task.cancel()
        self._backend.terminate()
        await self._backend.wait()

    async def __aenter__(self) -> "PagefindService":
        return await self.launch()

    async def __aexit__(
        self,
        exc_type: Optional[Any],
        exc_value: Optional[Any],
        traceback: Optional[Any],
    ) -> None:
        await self.close()

    async def create_index(
        self, config: Optional["IndexConfig"] = None
    ) -> "PagefindIndex":
        from ..index import PagefindIndex

        _config: Optional["IndexConfig"] = None
        if config is not None:
            _config = {**config}
            _ = _config.pop("output_path", None)
        else:
            _config = None
        log.debug(f"creating index with config: {_config}")
        result = await self.send(
            InternalNewIndexRequest(type="NewIndex", config=_config)
        )
        log.debug(f"received response: {result}")
        assert result["type"] == "NewIndex"
        result = cast(InternalNewIndexResponse, result)
        return PagefindIndex(config=config, _service=self, _index_id=result["index_id"])