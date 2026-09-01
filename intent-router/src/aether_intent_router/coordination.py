"""Multi-agent coordination: Contract-Net-style task allocation.

Blueprint §7.4: when a task spans multiple devices, agents declare their
capabilities and costs, and an arbiter (chosen dynamically) allocates
tasks. This module implements a minimal, synchronous version of that
negotiation for a single planning round.
"""

from __future__ import annotations

from typing import Dict, List, Optional

from .models import Action, AgentCapability


def allocate_actions(
    actions: List[Action], agents: List[AgentCapability]
) -> Dict[str, Optional[str]]:
    """For each action, pick the cheapest capable agent (lowest
    `cost_multiplier`). Returns a mapping of action name -> agent_id
    (or None if no agent can perform it).

    This models the "announce → bid → award" shape of the Contract Net
    Protocol as a single synchronous best-bid selection rather than a
    real multi-round negotiation — sufficient for a single planning
    round with all bids known up front.
    """
    assignment: Dict[str, Optional[str]] = {}
    for action in actions:
        candidates = [a for a in agents if action.name in a.supported_actions]
        if not candidates:
            assignment[action.name] = None
            continue
        best = min(candidates, key=lambda a: a.cost_multiplier)
        assignment[action.name] = best.agent_id
    return assignment
