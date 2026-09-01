"""Goal-Oriented Action Planning (GOAP) — blueprint §7.2 planner stage.

A minimal, dependency-free GOAP implementation: given a world state (a
dict of facts) and a goal implied by the `Intent`, search backward/forward
over a small fixed action library to find a sequence of actions whose
preconditions are satisfiable and whose effects reach the goal.

This is intentionally simple (breadth-first over a tiny action set) —
production planning should use PDDL or Behavior Trees per the blueprint;
this module exists to make the pipeline runnable end-to-end today.
"""

from __future__ import annotations

from collections import deque
from typing import Any

from .models import Action, Intent, Plan

# A tiny built-in action library. Real systems would derive this from
# each connected agent's declared `AgentCapability` (see coordination.py)
# rather than hardcoding it.
_ACTION_LIBRARY: list[Action] = [
    Action(
        name="navigate_to_kitchen",
        preconditions={"robot_at": "!kitchen"},
        effects={"robot_at": "kitchen"},
        cost=2.0,
    ),
    Action(
        name="pick_up_glass",
        preconditions={"robot_at": "kitchen", "has_glass": False},
        effects={"has_glass": True},
        cost=1.0,
    ),
    Action(
        name="fill_glass",
        preconditions={"has_glass": True, "glass_filled": False},
        effects={"glass_filled": True},
        cost=1.0,
    ),
    Action(
        name="deliver_glass",
        preconditions={"has_glass": True, "glass_filled": True},
        effects={"delivered": True},
        cost=2.0,
    ),
]

_INTENT_GOALS: dict[str, dict[str, Any]] = {
    "fetch_object": {"delivered": True},
}


def _preconditions_met(state: dict[str, Any], preconditions: dict[str, Any]) -> bool:
    for key, expected in preconditions.items():
        if isinstance(expected, str) and expected.startswith("!"):
            if state.get(key) == expected[1:]:
                return False
        elif state.get(key) != expected:
            return False
    return True


def _apply(state: dict[str, Any], effects: dict[str, Any]) -> dict[str, Any]:
    new_state = dict(state)
    new_state.update(effects)
    return new_state


def _goal_met(state: dict[str, Any], goal: dict[str, Any]) -> bool:
    return all(state.get(k) == v for k, v in goal.items())


class GoapPlanner:
    """Breadth-first GOAP search over `_ACTION_LIBRARY`."""

    def plan(self, intent: Intent, world_state: dict[str, Any]) -> Plan:
        goal = _INTENT_GOALS.get(intent.name)
        if goal is None:
            return Plan(intent=intent, actions=[])

        # BFS over (state, path) — fine for a handful of actions; swap for
        # A*/PDDL when the action library grows (roadmap Stage 5).
        start_state = dict(world_state)
        queue: deque[tuple[dict[str, Any], list[Action]]] = deque()
        queue.append((start_state, []))
        visited = {tuple(sorted(start_state.items()))}

        max_iterations = 10_000
        for _ in range(max_iterations):
            if not queue:
                break
            state, path = queue.popleft()
            if _goal_met(state, goal):
                return Plan(intent=intent, actions=path)

            for action in _ACTION_LIBRARY:
                if _preconditions_met(state, action.preconditions):
                    next_state = _apply(state, action.effects)
                    key = tuple(sorted(next_state.items()))
                    if key not in visited:
                        visited.add(key)
                        queue.append((next_state, path + [action]))

        # No plan found — return an empty plan; caller decides how to
        # surface that (blueprint's Executor would report failure).
        return Plan(intent=intent, actions=[])
