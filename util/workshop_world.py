#!/usr/bin/env python3
"""Build the M1 workshop world from engine/assets/DEFAULT.world.

Produces references/simplification/worlds/workshop-m1.world with exactly
16 playable policy cards (13 unlocked at start, 3 prerequisite-gated),
everything else locked, all events removed, fixed rebalanced costs.

Reproducible: python3 util/workshop_world.py
Stdlib only.
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "engine" / "assets" / "DEFAULT.world"
OUT = ROOT / "references" / "simplification" / "worlds" / "workshop-m1.world"

# Card name (as it appears in DEFAULT.world) -> target fixed cost in PC.
CARDS = {
    "Mass Electrification": 15,
    "Energy Quotas": 10,
    "Crack Down on Crypto-Mining": 5,
    "Vegetarian Mandate": 15,
    "Meatless Mondays": 5,
    "Cellular Meat": 10,
    "Regenerative Agriculture": 10,
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

# Prerequisite-gated cards: card -> the card whose passing unlocks it.
# These start locked; the prerequisite gets an UnlocksProject effect.
PREREQS = {
    "Cellular Meat": "Regenerative Agriculture",
    "Solar Radiation Management (SRM)": "Remediate and Protect Ecosystems",
    # M1 placeholder: the real gate is a clean-electricity card (M2).
    "Mass Electrification": "Energy Quotas",
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


def main():
    world = json.loads(SRC.read_text())

    by_name = {p["name"]: p for p in world["projects"]}
    missing = [name for name in CARDS if name not in by_name]
    if missing:
        sys.exit(f"FATAL: cards not found in DEFAULT.world: {missing}")

    kept_ids = {by_name[name]["id"]: name for name in CARDS}

    # 5. Delete all events (M1 also runs with SKIP_EVENTS; belt-and-braces).
    world["events"] = []

    for project in world["projects"]:
        name = project["name"]
        # Event-referencing effects would dangle/panic with events gone;
        # strip from every project (locked ones are still validated/applied).
        strip_effects(project, EVENT_EFFECTS)
        if name in CARDS:
            # 1. Re-kind everything to Policy (spec lever B2).
            project["kind"] = "Policy"
            # 2. Flatten cost to the fixed PC value.
            project["base_cost"] = {"Fixed": CARDS[name]}
            project["cost"] = CARDS[name]
            project["cost_modifier"] = 1.0
            # 4. Prereq-gated cards start locked; the rest start unlocked.
            project["locked"] = name in PREREQS
            # Playing a card must never unlock/lock non-workshop projects.
            strip_effects(project, PROJECT_LOCK_EFFECTS)
        else:
            # 3. Lock everything else (kept, not deleted, so ids referenced
            # by project_lockers/effects stay valid).
            project["locked"] = True

    # 4. Wire prerequisites via the existing UnlocksProject effect.
    for card, prereq in PREREQS.items():
        by_name[prereq]["effects"].append({"UnlocksProject": by_name[card]["id"]})

    # --- Acceptance checks -------------------------------------------------
    errors = validate(world)
    assert not errors, "\n".join(errors)

    kept = [p for p in world["projects"] if p["name"] in CARDS]
    others = [p for p in world["projects"] if p["name"] not in CARDS]
    assert len(kept) == 16
    assert all(p["kind"] == "Policy" for p in kept)
    assert all(p["base_cost"] == {"Fixed": CARDS[p["name"]]} for p in kept)
    unlocked = sorted(p["name"] for p in kept if not p["locked"])
    gated = sorted(p["name"] for p in kept if p["locked"])
    assert len(unlocked) == 13 and gated == sorted(PREREQS), (unlocked, gated)
    assert all(p["locked"] for p in others)
    assert world["events"] == []
    # No event-referencing effects remain anywhere in the world.
    leftovers = []
    collect_effects(world, leftovers)
    assert not any(effect_key(e) in EVENT_EFFECTS for e in leftovers)

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(world, indent=2, ensure_ascii=False) + "\n")

    print(f"Wrote {OUT.relative_to(ROOT)}")
    print(f"  projects: {len(world['projects'])} "
          f"(16 workshop cards, {len(others)} locked)")
    print(f"  unlocked at start (13): {', '.join(unlocked)}")
    print(f"  prerequisite-gated (3): "
          + "; ".join(f"{c} <- {p}" for c, p in sorted(PREREQS.items())))
    print(f"  events: {len(world['events'])}")
    print("  referential integrity: OK")


if __name__ == "__main__":
    main()
