"""Aether Intent Router.

Translates high-level human intent into concrete, multi-device action
plans. See docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md §7.

Copyright 2026 Ciprian Ștefan Pleșca — Apache License 2.0
"""

from .models import Action, AgentCapability, Intent, Plan
from .nlu import NluBackend, RuleBasedNlu
from .planner import GoapPlanner
from .executor import Executor, ExecutionResult

__all__ = [
    "Action",
    "AgentCapability",
    "Intent",
    "Plan",
    "NluBackend",
    "RuleBasedNlu",
    "GoapPlanner",
    "Executor",
    "ExecutionResult",
]

__version__ = "0.1.0a0"
