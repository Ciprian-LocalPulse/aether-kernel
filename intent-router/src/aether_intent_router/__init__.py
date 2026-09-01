"""Aether Intent Router.

Translates high-level human intent into concrete, multi-device action
plans. See docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md §7.

Copyright 2026 Ciprian Ștefan Pleșca — Apache License 2.0
"""

from .executor import ExecutionResult, Executor
from .models import Action, AgentCapability, Intent, Plan
from .nlu import NluBackend, RuleBasedNlu
from .planner import GoapPlanner

__all__ = [
    "Action",
    "AgentCapability",
    "ExecutionResult",
    "Executor",
    "GoapPlanner",
    "Intent",
    "NluBackend",
    "Plan",
    "RuleBasedNlu",
]

__version__ = "0.1.0a0"
