import asyncio
import base64
import json
import logging
import os
import shutil
from contextlib import AbstractAsyncContextManager
from pathlib import Path
from typing import TYPE_CHECKING, Any, Dict, List, Optional, cast

from .types import (
    InternalNewIndexRequest,
    InternalNewIndexResponse,
    InternalRequestPayload,
    InternalResponseError,
    InternalResponsePayload,
    InternalResponseType,
    InternalServiceRequest,
    InternalServiceResponse,
    InternalSyntheticFile,
)

if TYPE_CHECKING:
    from ..index import IndexConfig, PagefindIndex

log = logging.getLogger(__name__)


__all__ = ["PagefindService", "get_executable"]


def get_executable() -> Optional[Path]:
    env_bin_path = os.getenv("PAGEFIND_BINARY_PATH")
    if env_bin_path is not None:
        log.debug(f"using {env_bin_path}")
        return Path(env_bin_path)

    try:
        from pagefind_bin_extended import get_executable  # type: ignore

        extended: Path = get_executable()
        log.debug(f"using {extended}")
        return extended
    except ImportError:
        log.debug("unable to import pagefind_bin_extended")

    try:
        from pagefind_bin import get_executable  # type: ignore

        bin: Path = get_executable()
        log.debug(f"using {bin}")
        return bin
    except ImportError:
        log.debug("unable to import pagefind_bin")

    external: Optional[str] = shutil.which("pagefind_extended")
    external = external or shutil.which("pagefind")
    if external is None:
        log.debug("Could not find externally-installed pagefind binary")
        return None
    else:
        log.debug(f"using {external}")
        return Path(external)


def _must_get_executable() -> Path:
    if (bin := get_executable()) is None:
        raise FileNotFoundError("Could not find pagefind binary")
    return bin


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
    def __init__(self) -> None:
        self._loop = asyncio.get_event_loop()
        self._bin = _must_get_executable()
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
            # "--verbose", # <- verbose emits debug logs to stdout, which is also used for IPC
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
        if result["type"] == InternalResponseType.GET_FILES.value:  # these are HUGE
            if (files := result.get("files")) is not None:
                files = cast(List[InternalSyntheticFile], files)
                base64_ch = sum(len(file["content"]) for file in files)
                log.debug(f"received response: <{len(files)} files, {base64_ch} chars>")
        else:
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
            if len(output) <= 200:
                log.debug(f"received data: {output!r}")
            else:
                log.debug(
                    f"received data: {output[:30]!r}...{len(output) - 40}B...{output[-10:]!r}"
                )
            if (resp := json.loads(base64.b64decode(output[:-1]))) is None:
                continue
            resp = cast(InternalServiceResponse, resp)
            message_id = resp.get("message_id")
            if message_id is None:
                # If the backend service failed to parse the message, it won't return the ID
                # However it does return the message itself, so we can retrieve the ID we sent
                if (orginal := resp["payload"].get("original_message")) is not None:
                    if (sent := json.loads(orginal)) is not None:
                        message_id = sent.get("message_id")
            if message_id is not None:
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

    async def close(self) -> None:
        # wait for all _responses to be resolved
        log.debug("waiting for all responses to be resolved")
        try:
            # wait at most 5s for all responses to be resolved
            async with asyncio.timeout(5):
                await asyncio.gather(*self._responses.values())
                log.debug("all responses resolved")
        except asyncio.TimeoutError:
            log.error("timed out waiting for responses to be resolved")
        self._poll_task.cancel()
        self._backend.terminate()
        await self._backend.wait()
        log.debug("backend terminated")

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
            _config = {**config}  # clone the config to avoid modifying the original
            _config.pop("output_path", None)

        log.debug(f"creating index with config: {_config}")
        result = await self.send(
            InternalNewIndexRequest(type="NewIndex", config=_config)
        )
        log.debug(f"received response: {result}")
        assert result["type"] == "NewIndex"
        result = cast(InternalNewIndexResponse, result)
        return PagefindIndex(config=config, _service=self, _index_id=result["index_id"])
