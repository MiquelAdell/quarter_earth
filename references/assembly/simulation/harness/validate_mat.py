#!/usr/bin/env python3
"""Operator check for a coalition mat (run-sheet "Operator check" table, deck A).

Usage:
  validate_mat.py mat.json [--history history.json] [--other-mat other.json] [--json-only]

mat.json:     {"round": 1, "cards": ["Solar Push", ...], "sponsors": ["EI", "LS"]}
history.json: ["Card Name", ...]  cards passed in earlier rounds
Prints a JSON verdict {"valid", "rejections", "warnings"} followed by human text.
Exit code 0 when valid, 1 when rejected.
"""
import argparse
import json
import os
import sys

SCENARIOS = os.path.expanduser("~/code/quarter_earth/references/assembly/fixtures/scenarios.json")
BUDGET = 30
MAX_CARDS = 3
COAL_PAIR = ("Phase Out Coal", "Solar Push")
COAL_WARNING = ("Coal power starts too low for both; whichever applies second does nothing, "
                "but both are charged.")


def load_cards():
    with open(SCENARIOS) as f:
        return {c["name"].lower(): c for c in json.load(f)["opening_cards"]}


def canonical(name, catalogue):
    card = catalogue.get(name.strip().lower())
    return card["name"] if card else name.strip()


def check(mat, history=(), other_mat=None, catalogue=None):
    catalogue = catalogue or load_cards()
    names = [canonical(n, catalogue) for n in mat.get("cards", [])]
    passed = {canonical(n, catalogue) for n in history}
    sponsors = [s.strip().upper() for s in mat.get("sponsors", [])]
    known = [n for n in names if n.lower() in catalogue]

    rejections = []
    unknown = [n for n in names if n.lower() not in catalogue]
    if unknown:
        rejections.append(f"Unknown card: {', '.join(unknown)}.")
    if not names:
        rejections.append("No cards: a package needs 1-3 cards.")
    elif len(names) > MAX_CARDS:
        rejections.append(f"Too many cards ({len(names)}): a package holds at most {MAX_CARDS}.")
    dupes = sorted({n for n in names if names.count(n) > 1})
    if dupes:
        rejections.append(f"Repeated card: {', '.join(dupes)}.")
    total = sum(catalogue[n.lower()]["cost"] for n in known)
    if total > BUDGET:
        rejections.append(f"Over budget: total {total} > {BUDGET} points.")
    already = [n for n in dict.fromkeys(names) if n in passed]
    if already:
        rejections.append(f"Card already passed in an earlier round: {', '.join(already)}.")
    if len(set(sponsors)) < 2:
        rejections.append(f"Fewer than two distinct sponsoring tables ({', '.join(sponsors) or 'none'}).")
    if other_mat is not None:
        other = {s.strip().upper() for s in other_mat.get("sponsors", [])}
        reused = sorted(set(sponsors) & other)
        if reused:
            rejections.append(f"Sponsor reused on both mats: {', '.join(reused)}. "
                              "Each table signs one mat per round.")

    warnings = []
    pool = set(names) | passed
    if all(c in pool for c in COAL_PAIR) and any(c in names for c in COAL_PAIR):
        warnings.append(f"Coal conflict ({' + '.join(COAL_PAIR)}): {COAL_WARNING}")

    return {"valid": not rejections, "rejections": rejections, "warnings": warnings,
            "total_cost": total}


def human(result, mat):
    lines = [f"Mat (round {mat.get('round', '?')}): {', '.join(mat.get('cards', [])) or '(empty)'} "
             f"| sponsors {', '.join(mat.get('sponsors', [])) or '(none)'} "
             f"| total {result['total_cost']}/{BUDGET}",
             "VALID" if result["valid"] else "REJECTED"]
    lines += [f"  - REJECT: {r}" for r in result["rejections"]]
    lines += [f"  - WARNING (read aloud): {w}" for w in result["warnings"]]
    return "\n".join(lines)


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("mat")
    p.add_argument("--history", help="JSON list of card names passed in earlier rounds")
    p.add_argument("--other-mat", help="the other mat this round, to check sponsor reuse")
    p.add_argument("--json-only", action="store_true")
    args = p.parse_args(argv)

    def load(path):
        with open(path) as f:
            return json.load(f)

    mat = load(args.mat)
    history = load(args.history) if args.history else []
    other = load(args.other_mat) if args.other_mat else None
    result = check(mat, history, other)
    print(json.dumps({k: result[k] for k in ("valid", "rejections", "warnings")}, indent=2))
    if not args.json_only:
        print()
        print(human(result, mat))
    sys.exit(0 if result["valid"] else 1)


if __name__ == "__main__":
    main()
