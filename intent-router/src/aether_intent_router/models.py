"""Shared data models for the Intent Router pipeline."""

from __future__ import annotations

from typing import Any, Dict, List, Optional

from pydantic import BaseModel, Field


class Intent(BaseModel):
    """A structured human intent extracted from an utterance.

    Mirrors blueprint §7.2: NLU turns raw input into "intenție + entități".
    """

    name: str = Field(..., description="Canonical intent name, e.g. 'fetch_object'")
    entities: Dict[str, str] = Field(default_factory=dict)
    raw_text: Optional[str] = None
    confidence: float = 1.0


class Action(BaseModel):
    """A single primitive action in a plan: preconditions, effects, cost."""

    name: str
    preconditions: Dict[str, Any] = Field(default_factory=dict)
    effects: Dict[str, Any] = Field(default_factory=dict)
    cost: float = 1.0
    assigned_agent: Optional[str] = None


class Plan(BaseModel):
    """An ordered sequence of actions produced by the planner."""

    intent: Intent
    actions: List[Action] = Field(default_factory=list)

    @property
    def total_cost(self) -> float:
        return sum(a.cost for a in self.actions)


class AgentCapability(BaseModel):
    """What a given agent (robot, vehicle, device) claims it can do —
    used by the Contract-Net-style coordinator (blueprint §7.4)."""

    agent_id: str
    supported_actions: List[str]
    cost_multiplier: float = 1.0
    location: Optional[str] = None
