"""Executes a `Plan` step by step against connected Aether devices.

Blueprint §7.2: "Executor: Traduce planul în comenzi concrete ... [și]
monitorizează progresul și face ajustări în timp real." This scaffold
models execution against a pluggable `ActionHandler` so it can run in a
simulator, against the SDK, or against nothing at all (dry run / tests).
"""

from __future__ import annotations

from collections.abc import Callable, Iterator
from dataclasses import dataclass

from .models import Action, Plan

ActionHandler = Callable[[Action], bool]  # returns True on success


@dataclass
class ExecutionResult:
    action: Action
    succeeded: bool
    message: str = ""


def _default_handler(action: Action) -> bool:
    """Dry-run handler: pretends every action succeeds. Replace with a
    handler that calls into the Aether SDK (`sdk/python`) to actually
    drive a device."""
    return True


class Executor:
    def __init__(self, handler: ActionHandler | None = None) -> None:
        self._handler = handler or _default_handler

    def run(self, plan: Plan) -> Iterator[ExecutionResult]:
        for action in plan.actions:
            try:
                ok = self._handler(action)
                yield ExecutionResult(
                    action=action,
                    succeeded=ok,
                    message="ok" if ok else "handler reported failure",
                )
                if not ok:
                    # Blueprint calls for real-time adjustment on failure;
                    # this scaffold just stops the plan. A production
                    # executor would re-plan or retry here.
                    return
            except Exception as exc:  # noqa: BLE001 - surface any handler error
                yield ExecutionResult(action=action, succeeded=False, message=str(exc))
                return
