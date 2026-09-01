"""Shared data models for the Intent Router pipeline."""

from __future__ import annotations

from typing import Any

from pydantic import BaseModel, Field


class Intent(BaseModel):
    """A structured human intent extracted from an utterance.

    Mirrors blueprint §7.2: NLU turns raw input into "intenție + entități".
    """

    name: str = Field(..., description="Canonical intent name, e.g. 'fetch_object'")
    entities: dict[str, str] = Field(default_factory=dict)
    raw_text: str | None = None
    confidence: float = 1.0


class Action(BaseModel):
    """A single primitive action in a plan: preconditions, effects, cost."""

    name: str
    preconditions: dict[str, Any] = Field(default_factory=dict)
    effects: dict[str, Any] = Field(default_factory=dict)
    cost: float = 1.0
    assigned_agent: str | None = None


class Plan(BaseModel):
    """An ordered sequence of actions produced by the planner."""

    intent: Intent
    actions: list[Action] = Field(default_factory=list)

    @property
    def total_cost(self) -> float:
        return sum(a.cost for a in self.actions)


class AgentCapability(BaseModel):
    """What a given agent (robot, vehicle, device) claims it can do —
    used by the Contract-Net-style coordinator (blueprint §7.4)."""

    agent_id: str
    supported_actions: list[str]
    cost_multiplier: float = 1.0
    location: str | None = None
