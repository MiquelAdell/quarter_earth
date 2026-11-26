#!/usr/bin/env python3
"""Simulated workshop round: cumulative card decisions -> PLANET + REPORT text.

Runs the real engine runner (assembly_baseline) against the embedded WORKSHOP.world.

Usage:
  sim_engine.py --decisions d.json --round N [--out-dir DIR] [--planet-only]

d.json: {"2022": ["Card Name", ...], "2027": [...]}  (list order = application order)
Round N plans in year 2022 + 5*(N-1) and its report covers planning year -> +5.
Only decisions for planning years <= the round's planning year are sent to the engine.
"""
import argparse
import csv
import json
import os
import subprocess
import sys

REPO = os.path.expanduser("~/code/quarter_earth")
RUNNER = os.path.join(REPO, "target/release/examples/assembly_baseline")
SCENARIOS = os.path.join(REPO, "references/assembly/fixtures/scenarios.json")
HERE = os.path.dirname(os.path.abspath(__file__))
FIRST_YEAR = 2022
ROUND_YEARS = 5

# (csv column, display name, unit, formatter)
INDICATORS = (
    ("emissions_gtco2eq", "Emissions", "Gt CO2-eq/yr", lambda v: f"{v:.2f}"),
    ("temperature_c", "Temperature", "°C warming", lambda v: f"{v:.2f}"),
    ("extinction_rate", "Extinction pressure", "index, lower is better", lambda v: f"{v:.1f}"),
    ("outlook", "Well-being (contentedness)", "index", lambda v: f"{v:.1f}"),
    ("land_use_fraction", "Land use", "% of habitable land", lambda v: f"{v * 100:.1f}%"),
)


def planning_year(round_no):
    return FIRST_YEAR + ROUND_YEARS * (round_no - 1)


def load_fixture():
    with open(SCENARIOS) as f:
        return json.load(f)


def build_scenario(fixture, decisions, up_to_year):
    by_name = {c["name"].lower(): c for c in fixture["opening_cards"]}
    unknown = [n for names in decisions.values() for n in names if n.lower() not in by_name]
    if unknown:
        sys.exit(f"Unknown card name(s): {', '.join(unknown)}. Known: "
                 + ", ".join(c["name"] for c in fixture["opening_cards"]))
    years = {y: names for y, names in decisions.items() if int(y) <= up_to_year and names}
    scenario = {
        "id": "sim",
        "title": "sim",
        "decisions": {
            y: [{"id": by_name[n.lower()]["id"], "name": by_name[n.lower()]["name"]} for n in names]
            for y, names in sorted(years.items())
        },
    }
    return {**fixture, "scenarios": [scenario]}


def run_engine(scenario_file, out_dir, tag):
    out_json = os.path.join(out_dir, f"{tag}.out.json")
    out_csv = os.path.join(out_dir, f"{tag}.csv")
    proc = subprocess.run([RUNNER, scenario_file, "sim", out_json, out_csv],
                          cwd=REPO, capture_output=True, text=True)
    if proc.returncode != 0:
        sys.exit(f"Engine runner failed ({proc.returncode}):\n{proc.stderr}")
    with open(out_csv) as f:
        rows = {int(r["year"]): r for r in csv.DictReader(f)}
    return rows, out_csv


def cards(field):
    return [c for c in field.split("|") if c] if field else []


def fmt_cards(field):
    names = cards(field)
    return ", ".join(names) if names else "(none)"


def planet_text(row, year):
    lines = [f"PLANET {year}"]
    lines += [f"  {name:<28} {fmt(float(row[col])):>8}  ({unit})" for col, name, unit, fmt in INDICATORS]
    lines.append(f"  Cards in force:  {fmt_cards(row['online_cards'])}")
    lines.append(f"  Cards building:  {fmt_cards(row['building_cards'])}")
    return "\n".join(lines)


def signed(col, delta):
    if col == "land_use_fraction":
        return f"{delta * 100:+.1f} pts"
    return f"{delta:+.2f}"


def report_text(rows, round_no):
    start, end = planning_year(round_no), planning_year(round_no) + ROUND_YEARS
    years = [y for y in range(start, end + 1) if y in rows]
    lines = [f"REPORT round {round_no}: {start} -> {end}"]
    for col, name, unit, fmt in INDICATORS:
        a, b = float(rows[start][col]), float(rows[end][col])
        lines.append(f"  {name:<28} {fmt(a):>8} -> {fmt(b):>8}  ({signed(col, b - a)})  [{unit}]")
    lines.append("  Year by year:")
    header = "    " + f"{'year':<6}" + "".join(f"{name.split(' ')[0][:11]:>12}" for _, name, _, _ in INDICATORS)
    lines.append(header)
    lines += ["    " + f"{y:<6}" + "".join(f"{fmt(float(rows[y][col])):>12}" for col, _, _, fmt in INDICATORS)
              for y in years]
    lines.append(f"  Cards in force at {end}:  {fmt_cards(rows[end]['online_cards'])}")
    lines.append(f"  Cards building at {end}:  {fmt_cards(rows[end]['building_cards'])}")
    return "\n".join(lines)


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--decisions", required=True, help='JSON file {"2022": ["Card Name", ...], ...}')
    p.add_argument("--round", type=int, required=True, help="round number (1 = 2022, 2 = 2027, ...)")
    p.add_argument("--out-dir", default=os.path.join(HERE, "runs"))
    p.add_argument("--planet-only", action="store_true")
    args = p.parse_args(argv)

    if args.round < 1:
        sys.exit("--round must be >= 1")
    with open(args.decisions) as f:
        decisions = json.load(f)
    os.makedirs(args.out_dir, exist_ok=True)

    fixture = load_fixture()
    year = planning_year(args.round)
    if year + ROUND_YEARS > fixture["end_year"]:
        sys.exit(f"Round {args.round} ends after end_year {fixture['end_year']}")

    tag = f"round{args.round}"
    scen_file = os.path.join(args.out_dir, f"{tag}.scenario.json")
    with open(scen_file, "w") as f:
        json.dump(build_scenario(fixture, decisions, year), f, indent=2)
    rows, out_csv = run_engine(scen_file, args.out_dir, tag)

    print(planet_text(rows[year], year))
    if not args.planet_only:
        print()
        print(report_text(rows, args.round))
    print(f"\n(engine CSV: {out_csv})")


if __name__ == "__main__":
    main()
