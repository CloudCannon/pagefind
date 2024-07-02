import os
import platform

from pathlib import Path
import json
from contextlib import AbstractAsyncContextManager
from typing import Any, Dict, Optional, Union, cast

from .types import (
    InternalRequestPayload,
    InternalResponsePayload,
    InternalResponseError,
    InternalServiceRequest,
    InternalServiceResponse,
    InternalResponseType,
)
import asyncio
import base64


# _bin: Optional[Path] = None


def _find_binary() -> Union[Path, None]:
    # TODO: verify this is the correct path
    this_dir = Path(__file__).parent
    names = ["pagefind_extended", "pagefind"]
    extensions = [""]
    if platform.system().lower() == "Windows":
        extensions.append(".exe")
    result = None
    for name in [n + ext for n in names for ext in extensions]:
        if (bin := this_dir / name).exists():
            if not bin.is_file():
                raise FileNotFoundError(f"{bin} is not a file")
            result = bin
            break
    return result


def _encode(req: InternalServiceRequest) -> bytes:
    return base64.b64encode(json.dumps(req).encode("utf-8"))


class Service(AbstractAsyncContextManager["Service"]):
    _bin: Path
    _backend: asyncio.subprocess.Process
    _message_id: int = 0
    _responses: Dict[int, asyncio.Future[InternalResponsePayload]]
    _loop: asyncio.AbstractEventLoop
    _poll_task: asyncio.Task[None]

    # _messages
    def __init__(self):
        self._loop = asyncio.get_event_loop()
        _bin = _find_binary()
        if _bin is None:
            raise FileNotFoundError(
                "Could not find `pagefind` or `pagefind_extended` binary"
            )
        self._bin = _bin

    async def launch(self) -> "Service":
        # TODO: detach process on windows?
        # creation_flags: int = 0
        # if platform.system().lower() == "windows":
        #     creation_flags = subprocess.CREATE_NO_WINDOW | subprocess.CREATE_DETACHED
        self._backend = await asyncio.create_subprocess_exec(
            self._bin,
            "--service",
            cwd=os.getcwd(),
            stdin=None,
            stdout=None,
            # creationflags=creation_flags,
        )
        self._poll_task = self._loop.create_task(self.wait_for_responses())

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
            ...  # restart the backend
            await self.launch()
        assert self._backend.stdin is not None

        req = InternalServiceRequest(message_id=message_id, payload=payload)
        self._backend.stdin.write(_encode(req))
        await self._backend.stdin.drain()

        return await future

    async def wait_for_responses(self) -> None:
        """
        Poll the subprocess's stdout for responses
        """
        while True:
            assert self._backend.stdout is not None
            output = await self._backend.stdout.readuntil(b",")
            if (resp := json.loads(base64.b64decode(output[:-1]))) is None:
                continue
            resp = cast(InternalServiceResponse, resp)
            if (message_id := resp.get("message_id")) is not None:
                assert self._message_id <= message_id, "message_id out of order"
                if (future := self._responses.get(message_id)) is not None:
                    payload = resp["payload"]
                    if payload["type"] == InternalResponseType.ERROR.value:
                        exc = cast(InternalResponseError, payload)
                        future.set_exception(
                            Exception(exc["message"], exc.get("original_message"))
                        )
                    else:
                        future.set_result(cast(InternalResponsePayload, payload))
                else:
                    payload = cast(InternalResponseError, resp["payload"])
                    # assert (
                    #     payload["type"] == InternalResponseType.ERROR.value
                    # ), f"unexpected message type: {payload['type']}"
                    # FIXME: figure out how to surface the error

    async def close(self):
        # wait for all _responses to be resolved
        await asyncio.gather(*self._responses.values())  # IDEA: add timeout?
        self._poll_task.cancel()
        self._backend.terminate()
        await self._backend.wait()

    async def __aenter__(self) -> "Service":
        return await self.launch()

    async def __aexit__(
        self,
        exc_type: Optional[Any],
        exc_value: Optional[Any],
        traceback: Optional[Any],
    ) -> None:
        await self.close()
