"""Aether SDK (Python).

Client API mirroring blueprint §9.1: connect, create_object,
update_object, subscribe, send_intent.

Copyright 2026 Ciprian Ștefan Pleșca — Apache License 2.0
"""

from .client import AetherClient, ObjectId, SdkError, Transform

__all__ = ["AetherClient", "ObjectId", "SdkError", "Transform"]
__version__ = "0.1.0a0"
