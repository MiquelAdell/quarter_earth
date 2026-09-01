#!/usr/bin/env python3
"""Build the release workshop world from engine/assets/DEFAULT.world.

Produces engine/assets/WORKSHOP.world with exactly 21 playable policy cards
(18 unlocked at start, 3 prerequisite-gated), five atomic mix-transfer cards,
everything else locked, no events, and fixed workshop costs.

Reproducible: python3 util/workshop_world.py
Stdlib only.
"""

import json
import sys
import uuid
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "engine" / "assets" / "DEFAULT.world"
OUT = ROOT / "engine" / "assets" / "WORKSHOP.world"

# Card name (as it appears in DEFAULT.world) -> target fixed cost in PC.
CARDS = {
    "Solar Push": 15,
    "Wind Push": 15,
    "Nuclear Expansion": 15,
    "Phase Out Coal": 10,
    "Mass Electrification": 15,
    "Energy Quotas": 10,
    "Crack Down on Crypto-Mining": 5,
    "Vegetarian Mandate": 15,
    "Meatless Mondays": 5,
    "Cellular Meat": 10,
    "Regenerative Agriculture": 10,
    "Organic Transition": 10,
    "Expand Nature Preserves": 15,
    "Remediate and Protect Ecosystems": 10,
    "Ban Outdoor Cats": 5,
    "Solar Radiation Management (SRM)": 10,
    "Expand Public Transit": 10,
    "Ban Cars": 15,
    "Restrict Air Travel": 5,
    "Degrowth in Developed Regions": 15,
    "Luxury for All": 15,
}

CARD_DESCRIPTIONS = {
    "Solar Push": "Move 20% of electricity generation from coal to solar photovoltaics.",
    "Wind Push": "Move 20% of electricity generation from gas to terrestrial wind power.",
    "Nuclear Expansion": "Move 15% of electricity generation from coal to nuclear power.",
    "Phase Out Coal": "Retire 25% of coal generation. Retired capacity no longer supplies electricity.",
    "Organic Transition": "Move 20% of crop agriculture from industrial to organic production.",
}

# Each tuple is card name, source process name, target process name, and
# amount in five-percent units. Retired coal is a zero-capacity sink, keeping
# phase-out reversible while intentionally reducing available electricity.
MIX_TRANSFERS = (
    ("Solar Push", "Coal Power Generation", "Solar PV", 4),
    ("Wind Push", "Natural Gas Power Gen", "Terrestrial Wind Power", 4),
    ("Nuclear Expansion", "Coal Power Generation", "Nuclear Power", 3),
    ("Phase Out Coal", "Coal Power Generation", "Retired Coal Capacity", 5),
    ("Organic Transition", "Industrial Crop Ag", "Organic Crop Ag", 4),
)

RETIRED_COAL_CAPACITY = "Retired Coal Capacity"

# Prerequisite-gated cards: card -> the card whose passing unlocks it.
# These start locked; the prerequisite gets an UnlocksProject effect.
PREREQS = {
    "Cellular Meat": "Regenerative Agriculture",
    "Solar Radiation Management (SRM)": "Remediate and Protect Ecosystems",
    "Mass Electrification": ("Solar Push", "Wind Push", "Nuclear Expansion"),
}

# Effect variants that reference event ids (see Effect::event_id() in
# engine/src/events/effects.rs). With all events deleted these would be
# dangling references (and runtime panics), so they are stripped everywhere.
EVENT_EFFECTS = {"AddEvent", "TriggerEvent", "ModifyEventProbability"}

# Effect variants that unlock/lock projects. Stripped from kept cards so
# that playing a card can never surface a non-workshop project; the three
# prerequisite unlocks are then added back explicitly.
PROJECT_LOCK_EFFECTS = {"UnlocksProject", "LocksProject"}


def effect_key(effect):
    """JSON effect is either a bare string ("Migration") or {Variant: data}."""
    if isinstance(effect, dict) and len(effect) == 1:
        return next(iter(effect))
    return effect if isinstance(effect, str) else None


def strip_effects(node, drop_keys):
    """Recursively remove effects whose variant is in drop_keys from every
    "effects" list nested anywhere in node (project effects, outcomes,
    upgrades, flavor dialogue responses)."""
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "effects" and isinstance(value, list):
                node[key] = [e for e in value if effect_key(e) not in drop_keys]
                for e in node[key]:
                    strip_effects(e, drop_keys)
            else:
                strip_effects(value, drop_keys)
    elif isinstance(node, list):
        for item in node:
            strip_effects(item, drop_keys)


def collect_effects(node, out):
    """Recursively collect all effects from nested "effects" lists."""
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "effects" and isinstance(value, list):
                out.extend(value)
            collect_effects(value, out)
    elif isinstance(node, list):
        for item in node:
            collect_effects(item, out)


def collect_conditions(node, out):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "conditions" and isinstance(value, list):
                out.extend(value)
            collect_conditions(value, out)
    elif isinstance(node, list):
        for item in node:
            collect_conditions(item, out)


# Which condition variants reference which entity, per Condition::project_id /
# process_id in the engine (validate.rs only checks projects and processes).
CONDITION_PROJECT = {"ProjectStatus", "ActiveProjectUpgrades"}
CONDITION_PROCESS = {"ProcessOutput", "ProcessMixShare"}

# Effect variant -> entity kind, per engine/src/events/effects.rs.
EFFECT_REFS = {
    "LocksProject": "projects",
    "UnlocksProject": "projects",
    "ProjectRequest": "projects",
    "ProjectCostModifier": "projects",
    "OutputForProcess": "processes",
    "ProcessLimit": "processes",
    "UnlocksProcess": "processes",
    "TransferMixShare": "processes",
    "ProcessRequest": "processes",
    "ModifyProcessByproducts": "processes",
    "ModifyIndustryByproducts": "industries",
    "ModifyIndustryResources": "industries",
    "ModifyIndustryResourcesAmount": "industries",
    "ModifyIndustryDemand": "industries",
    "AddEvent": "events",
    "TriggerEvent": "events",
    "ModifyEventProbability": "events",
}


def referenced_id(effect):
    """Return (entity_kind, id) for effects that reference an entity."""
    key = effect_key(effect)
    kind = EFFECT_REFS.get(key)
    if kind is None:
        return None
    data = effect[key]
    # Id is either the payload itself or the first tuple element.
    eid = data[0] if isinstance(data, list) else data
    return (kind, eid)


def workshop_id(name):
    return str(uuid.uuid5(uuid.NAMESPACE_URL, f"quarter-earth/workshop-v2/{name}"))


def transfer_effect(source_id, target_id, amount):
    return {"TransferMixShare": [source_id, target_id, amount]}


def validate(world):
    """Replicate editor/src/validate.rs referential-integrity checks."""
    ids = {
        "projects": {p["id"] for p in world["projects"]},
        "processes": {p["id"] for p in world["processes"]},
        "industries": {p["id"] for p in world["industries"]},
        "events": {e["id"] for e in world["events"]},
    }
    errors = []
    for item in world["projects"] + world["events"]:
        effects, conditions = [], []
        collect_effects(item, effects)
        collect_conditions(item, conditions)
        for effect in effects:
            ref = referenced_id(effect)
            if ref and ref[1] not in ids[ref[0]]:
                errors.append(f"{item['name']}: effect {effect_key(effect)} "
                              f"refers to missing {ref[0][:-1]} {ref[1]}")
            if effect_key(effect) == "TransferMixShare":
                source, target, _ = effect["TransferMixShare"]
                for process_id in (source, target):
                    if process_id not in ids["processes"]:
                        errors.append(f"{item['name']}: transfer refers to missing "
                                      f"process {process_id}")
        for cond in conditions:
            key = effect_key(cond)
            if key in CONDITION_PROJECT or key in CONDITION_PROCESS:
                kind = "projects" if key in CONDITION_PROJECT else "processes"
                data = cond[key]
                cid = data[0] if isinstance(data, list) else data
                if cid not in ids[kind]:
                    errors.append(f"{item['name']}: condition {key} refers "
                                  f"to missing {kind[:-1]} {cid}")
    return errors


def validate_workshop(world):
    by_name = {project["name"]: project for project in world["projects"]}
    assert world["year"] == 2022
    assert world["lifespan"] == 30
    assert world["planning_anchor"] == 2022
    assert world["events"] == []
    assert world["project_lockers"] == {}
    assert sum(CARDS.values()) == 235
    assert set(CARDS) <= set(by_name)

    cards = [project for project in world["projects"] if project["name"] in CARDS]
    other_projects = [project for project in world["projects"] if project["name"] not in CARDS]
    assert len(cards) == 21
    assert all(project["kind"] == "Policy" for project in cards)
    assert all(project["base_cost"] == {"Fixed": CARDS[project["name"]]} for project in cards)
    assert all(project["cost"] == CARDS[project["name"]] for project in cards)
    assert all(project["locked"] for project in other_projects)

    unlocked = {project["name"] for project in cards if not project["locked"]}
    assert unlocked == set(CARDS) - set(PREREQS)
    unlocks = {
        project["name"]: [effect["UnlocksProject"] for effect in project["effects"] if effect_key(effect) == "UnlocksProject"]
        for project in cards
    }
    prerequisite_targets = {by_name[card]["id"] for card in PREREQS}
    for card, prereqs in PREREQS.items():
        expected = {by_name[card]["id"]}
        for prereq in (prereqs,) if isinstance(prereqs, str) else prereqs:
            assert set(unlocks[prereq]) == expected
    assert {
        target
        for targets in unlocks.values()
        for target in targets
    } == prerequisite_targets

    processes = {process["name"]: process for process in world["processes"]}
    for card, source, target, amount in MIX_TRANSFERS:
        assert transfer_effect(processes[source]["id"], processes[target]["id"], amount) in by_name[card]["effects"]
    assert processes[RETIRED_COAL_CAPACITY]["mix_share"] == 0
    assert processes[RETIRED_COAL_CAPACITY]["limit"] == 0
    assert all(effect_key(effect) not in EVENT_EFFECTS for effect in _effects(world))


def _effects(node):
    effects = []
    collect_effects(node, effects)
    return effects


def main():
    world = json.loads(SRC.read_text())

    by_name = {p["name"]: p for p in world["projects"]}
    generated_cards = {name for name, _, _, _ in MIX_TRANSFERS}
    missing = [name for name in CARDS if name not in generated_cards and name not in by_name]
    if missing:
        sys.exit(f"FATAL: cards not found in DEFAULT.world: {missing}")

    mix_template = by_name["Mass Electrification"]
    for name, _, _, _ in MIX_TRANSFERS:
        project = json.loads(json.dumps(mix_template))
        project.update({
            "id": workshop_id(name),
            "name": name,
            "kind": "Policy",
            "group": "Energy" if name != "Organic Transition" else "Food",
            "ongoing": False,
            "gradual": False,
            "locked": False,
            "cost": CARDS[name],
            "base_cost": {"Fixed": CARDS[name]},
            "cost_modifier": 1.0,
            "progress": 0.0,
            "points": 0,
            "estimate": 0,
            "status": "Inactive",
            "level": 0,
            "completed_at": 0,
            "required_majority": 0.0,
            "effects": [],
            "outcomes": [{"effects": [], "probability": {"likelihood": "Guaranteed", "conditions": []}}],
            "upgrades": [],
            "active_outcome": None,
            "supporters": [],
            "opposers": [],
            "notes": "Workshop v2 policy card.",
        })
        project["flavor"]["description"] = CARD_DESCRIPTIONS[name]
        project["flavor"]["outcomes"] = []
        world["projects"].append(project)

    by_name = {p["name"]: p for p in world["projects"]}
    coal_sink = json.loads(json.dumps(next(process for process in world["processes"] if process["name"] == "Coal Power Generation")))
    coal_sink.update({
        "id": workshop_id(RETIRED_COAL_CAPACITY),
        "name": RETIRED_COAL_CAPACITY,
        "mix_share": 0,
        "limit": 0,
        "locked": False,
        "supporters": [],
        "opposers": [],
        "notes": "Workshop-only zero-capacity sink for retired coal generation.",
    })
    coal_sink["flavor"]["description"] = "Retired coal capacity does not produce electricity."
    world["processes"].append(coal_sink)
    world["lifespan"] = 30
    world["planning_anchor"] = 2022

    # 5. Delete all events (M1 also runs with SKIP_EVENTS; belt-and-braces).
    world["events"] = []

    for project in world["projects"]:
        name = project["name"]
        # Event-referencing effects would dangle/panic with events gone;
        # strip from every project (locked ones are still validated/applied).
        strip_effects(project, EVENT_EFFECTS)
        strip_effects(project, PROJECT_LOCK_EFFECTS)
        if name in CARDS:
            # 1. Re-kind everything to Policy (spec lever B2).
            project["kind"] = "Policy"
            # 2. Flatten cost to the fixed PC value.
            project["base_cost"] = {"Fixed": CARDS[name]}
            project["cost"] = CARDS[name]
            project["cost_modifier"] = 1.0
            # 4. Prereq-gated cards start locked; the rest start unlocked.
            project["locked"] = name in PREREQS
        else:
            # 3. Lock everything else (kept, not deleted, so ids referenced
            # by project_lockers/effects stay valid).
            project["locked"] = True

    processes = {process["name"]: process for process in world["processes"]}
    for card, source, target, amount in MIX_TRANSFERS:
        by_name[card]["effects"] = [
            transfer_effect(processes[source]["id"], processes[target]["id"], amount)
        ]

    # 4. Wire prerequisites via the existing UnlocksProject effect after
    # installing transfer effects, so clean-electricity cards retain both.
    for card, prereqs in PREREQS.items():
        for prereq in (prereqs,) if isinstance(prereqs, str) else prereqs:
            by_name[prereq]["effects"].append({"UnlocksProject": by_name[card]["id"]})

    world["project_lockers"] = {}

    # --- Acceptance checks -------------------------------------------------
    errors = validate(world)
    assert not errors, "\n".join(errors)

    validate_workshop(world)

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(world, indent=2, ensure_ascii=False) + "\n")

    print(f"Wrote {OUT.relative_to(ROOT)}")
    kept = [project for project in world["projects"] if project["name"] in CARDS]
    others = [project for project in world["projects"] if project["name"] not in CARDS]
    unlocked = sorted(project["name"] for project in kept if not project["locked"])
    print(f"  projects: {len(world['projects'])} "
          f"(21 workshop cards, {len(others)} locked)")
    print(f"  unlocked at start (18): {', '.join(unlocked)}")
    print(f"  prerequisite-gated (3): "
          + "; ".join(f"{c} <- {p}" for c, p in sorted(PREREQS.items())))
    print(f"  events: {len(world['events'])}")
    print("  referential integrity: OK")


if __name__ == "__main__":
    main()
