from aether_intent_router.executor import Executor
from aether_intent_router.nlu import RuleBasedNlu
from aether_intent_router.planner import GoapPlanner
from aether_intent_router.coordination import allocate_actions
from aether_intent_router.models import AgentCapability


def test_rule_based_nlu_extracts_fetch_object_intent():
    nlu = RuleBasedNlu()
    intent = nlu.parse("bring me a glass of water")
    assert intent.name == "fetch_object"
    assert "object" in intent.entities


def test_rule_based_nlu_returns_unknown_for_unmatched_text():
    nlu = RuleBasedNlu()
    intent = nlu.parse("the weather is nice today")
    assert intent.name == "unknown"
    assert intent.confidence == 0.0


def test_planner_finds_full_fetch_object_plan():
    nlu = RuleBasedNlu()
    intent = nlu.parse("bring me a glass of water")
    planner = GoapPlanner()
    plan = planner.plan(
        intent,
        world_state={"robot_at": "living_room", "has_glass": False, "glass_filled": False},
    )
    action_names = [a.name for a in plan.actions]
    assert action_names == [
        "navigate_to_kitchen",
        "pick_up_glass",
        "fill_glass",
        "deliver_glass",
    ]


def test_executor_runs_full_plan_with_dry_run_handler():
    nlu = RuleBasedNlu()
    intent = nlu.parse("bring me a glass of water")
    planner = GoapPlanner()
    plan = planner.plan(
        intent,
        world_state={"robot_at": "living_room", "has_glass": False, "glass_filled": False},
    )
    executor = Executor()
    results = list(executor.run(plan))
    assert len(results) == len(plan.actions)
    assert all(r.succeeded for r in results)


def test_coordination_allocates_cheapest_capable_agent():
    from aether_intent_router.models import Action

    actions = [Action(name="pick_up_glass")]
    agents = [
        AgentCapability(agent_id="robot-a", supported_actions=["pick_up_glass"], cost_multiplier=2.0),
        AgentCapability(agent_id="robot-b", supported_actions=["pick_up_glass"], cost_multiplier=1.0),
    ]
    assignment = allocate_actions(actions, agents)
    assert assignment["pick_up_glass"] == "robot-b"
