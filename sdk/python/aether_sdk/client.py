from __future__ import annotations

from collections.abc import Callable
from dataclasses import dataclass


class SdkError(Exception):
    pass


@dataclass(frozen=True)
class ObjectId:
    value: int


@dataclass
class Transform:
    position: tuple[float, float, float] = (0.0, 0.0, 0.0)
    orientation: tuple[float, float, float, float] = (0.0, 0.0, 0.0, 1.0)


class AetherClient:
    """A connection to an Aether node.

    This is a scaffold: object storage and pub/sub are in-process only.
    A production client would talk to the sync-engine over QUIC (see
    ``sync-engine/src/network.rs``) and to the Intent Router over HTTP/gRPC.
    """

    def __init__(self, endpoint: str) -> None:
        self.endpoint = endpoint
        self._connected = True
        self._objects: dict[int, Transform] = {}
        self._next_id = 1
        self._subscribers: dict[str, list[Callable[[ObjectId, Transform], None]]] = {}

    @classmethod
    def connect(cls, endpoint: str) -> AetherClient:
        return cls(endpoint)

    def _require_connected(self) -> None:
        if not self._connected:
            raise SdkError("not connected")

    def create_object(self, transform: Transform | None = None) -> ObjectId:
        self._require_connected()
        oid = ObjectId(self._next_id)
        self._next_id += 1
        self._objects[oid.value] = transform or Transform()
        return oid

    def update_object(self, object_id: ObjectId, transform: Transform) -> None:
        self._require_connected()
        if object_id.value not in self._objects:
            raise SdkError(f"unknown object: {object_id}")
        self._objects[object_id.value] = transform
        for callback in self._subscribers.get("*", []):
            callback(object_id, transform)

    def subscribe(
        self, cell_id: str, callback: Callable[[ObjectId, Transform], None]
    ) -> None:
        self._require_connected()
        self._subscribers.setdefault(cell_id, []).append(callback)

    def send_intent(self, text: str, context: dict | None = None) -> None:
        """Forward a natural-language intent to the Intent Router.

        TODO(roadmap Stage 5): bridge to ``intent-router`` over HTTP/gRPC.
        """
        self._require_connected()
        _ = (text, context)

    def disconnect(self) -> None:
        self._connected = False
