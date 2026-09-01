"""Natural Language Understanding: utterance -> Intent.

Blueprint §7.2 specifies LLM-backed NLU (GPT-4 / Claude / self-hosted
Llama 3 via vLLM) fine-tuned for spatial, multi-device commands. This
module defines the `NluBackend` protocol so that a real LLM backend can
be swapped in without touching the planner or executor, and ships a
dependency-free `RuleBasedNlu` fallback so the pipeline works offline.
"""

from __future__ import annotations

import re
from typing import ClassVar, Protocol

from .models import Intent


class NluBackend(Protocol):
    def parse(self, utterance: str) -> Intent: ...


class RuleBasedNlu:
    """Keyword/regex-based NLU. Deliberately simple: a safety net and a
    reference implementation of the `NluBackend` protocol, not a
    replacement for a real language model."""

    _PATTERNS: ClassVar[list[tuple[re.Pattern[str], str]]] = [
        (
            re.compile(r"\bbring me a?n? ?(?P<object>[\w ]+)", re.IGNORECASE),
            "fetch_object",
        ),
        (
            re.compile(r"\btake me to (?:the )?(?P<destination>[\w ]+)", re.IGNORECASE),
            "navigate_to",
        ),
        (
            re.compile(r"\bshow me how to (?P<task>[\w ]+)", re.IGNORECASE),
            "show_instructions",
        ),
    ]

    def parse(self, utterance: str) -> Intent:
        for pattern, intent_name in self._PATTERNS:
            match = pattern.search(utterance)
            if match:
                return Intent(
                    name=intent_name,
                    entities={k: v.strip() for k, v in match.groupdict().items() if v},
                    raw_text=utterance,
                    confidence=0.7,  # rule-based match, not a calibrated probability
                )
        return Intent(name="unknown", raw_text=utterance, confidence=0.0)


class LlmNluBackend:
    """Adapter shape for a real LLM-backed NLU. Not implemented here —
    wire this to the Anthropic/OpenAI SDK or a self-hosted vLLM endpoint
    and have it return a structured `Intent` (e.g. via function-calling /
    JSON mode), matching blueprint §7.2."""

    def __init__(self, model: str, system_prompt: str | None = None) -> None:
        self.model = model
        self.system_prompt = system_prompt or (
            "Extract a structured intent (name + entities) for a spatial "
            "computing assistant controlling AR devices, robots and vehicles."
        )

    def parse(self, utterance: str) -> Intent:  # pragma: no cover - stub
        raise NotImplementedError(
            "LlmNluBackend.parse is a Stage 5 roadmap item — wire up an "
            "LLM client here. See docs/roadmap/ROADMAP.md."
        )
