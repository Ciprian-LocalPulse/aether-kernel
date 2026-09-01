# `intent-router/` — Intent Router (Python)

Translates high-level human intent (voice/text/gesture) into concrete
multi-device action plans. Blueprint reference:
[docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md §7](../docs/whitepaper/AETHER_KERNEL_BLUEPRINT.md#7-intent-router).

## Pipeline

```
Utterance → NLU (intent + entities) → Planner (GOAP/PDDL) → Executor → Aether SDK calls
```

## Modules

| File | Responsibility |
|---|---|
| `nlu.py` | Natural-language understanding: text → `Intent` + entities |
| `planner.py` | Goal-Oriented Action Planning (GOAP) over a simple world-state model |
| `executor.py` | Executes a `Plan` step by step, reporting progress/failures |
| `coordination.py` | Contract-Net-style multi-agent task allocation (§7.4) |
| `models.py` | Shared `pydantic` data models (`Intent`, `Action`, `Plan`, `AgentCapability`) |

## Status

`nlu.py` ships a rule-based/keyword NLU backend by default (no API key
required) and defines an `LlmNluBackend` protocol so a real LLM (blueprint:
GPT-4, Claude, self-hosted Llama 3 via vLLM) can be dropped in without
touching the planner or executor.

## Install & test

```bash
pip install -e ".[dev]"
pytest
```

## Example

```python
from aether_intent_router.nlu import RuleBasedNlu
from aether_intent_router.planner import GoapPlanner
from aether_intent_router.executor import Executor

nlu = RuleBasedNlu()
intent = nlu.parse("bring me a glass of water")

planner = GoapPlanner()
plan = planner.plan(intent, world_state={"robot_at": "kitchen", "has_glass": False})

executor = Executor()
for result in executor.run(plan):
    print(result)
```
