#!/usr/bin/env python3
"""Regenerate the assembly parity fixtures (W1 of the assembly-workshop change).

Runs `game/examples/assembly_baseline.rs` several times per scenario in
separate processes, groups the runs by output hash, keeps one canonical run
per native climate variant, diffs the variants, optionally replays the
recorded emissions through the browser Hector build (Node), and writes
`references/assembly/fixtures/reproducibility.json`.

Usage (repo root):
    export CXXFLAGS="-I/opt/homebrew/include"
    cargo build -p hes-game --example assembly_baseline
    python3 references/assembly/tools/assembly_baseline.py [--runs 5] [--profile debug|release] [--no-web]

Stdlib only. Read-only with respect to game logic.
"""

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / "references" / "assembly" / "fixtures"
SCENARIOS = FIXTURES / "scenarios.json"
WEB_REPLAY = ROOT / "references" / "assembly" / "tools" / "assembly_web_replay.mjs"


def variant_name(nox_lengths: str) -> str:
    # "OH:287,ozone:257" -> appended into OH; the other section is padded.
    parts = dict(p.split(":") for p in nox_lengths.split(","))
    return "oh_appended" if int(parts["OH"]) > int(parts["ozone"]) else "ozone_appended"


def flatten(prefix, value, out):
    if isinstance(value, dict):
        for k, v in value.items():
            flatten(f"{prefix}.{k}" if prefix else k, v, out)
    elif isinstance(value, list) and value and isinstance(value[0], dict):
        for i, v in enumerate(value):
            flatten(f"{prefix}[{i}]", v, out)
    else:
        out[prefix] = value


def diff_runs(a, b):
    """Max abs diff per numeric field and list of discrete mismatches."""
    rows_b = {r["year"]: r for r in b["rows"]}
    numeric, discrete = {}, []
    for ra in a["rows"]:
        fa, fb = {}, {}
        flatten("", ra, fa)
        flatten("", rows_b[ra["year"]], fb)
        for key, va in fa.items():
            vb = fb.get(key)
            is_num = isinstance(va, (int, float)) and not isinstance(va, bool)
            if is_num and key.startswith(("numeric", "regions")):
                field = key.split("[")[0] + (key.split("]", 1)[1] if "]" in key else "")
                d = abs(va - vb)
                if d > numeric.get(field, {"max_abs": -1})["max_abs"]:
                    numeric[field] = {"max_abs": d, "year": ra["year"]}
            elif va != vb:
                discrete.append({"year": ra["year"], "field": key, "a": va, "b": vb})
    return numeric, discrete


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--runs", type=int, default=5)
    ap.add_argument("--profile", default="debug", choices=["debug", "release"])
    ap.add_argument("--no-web", action="store_true")
    args = ap.parse_args()

    binary = ROOT / "target" / args.profile / "examples" / "assembly_baseline"
    if not binary.exists():
        sys.exit(f"missing {binary}; build it first (see docstring)")
    spec = json.loads(SCENARIOS.read_text())
    tmp = Path(tempfile.mkdtemp(prefix="assembly_baseline_"))
    report = {
        "profile": args.profile,
        "runs_per_scenario": args.runs,
        "scenarios": {},
    }

    for scenario in spec["scenarios"]:
        sid = scenario["id"]

        def run(i):
            out_json = tmp / f"{sid}.run{i}.json"
            out_csv = tmp / f"{sid}.run{i}.csv"
            subprocess.run(
                [str(binary), str(SCENARIOS), sid, str(out_json), str(out_csv)],
                cwd=ROOT,
                check=True,
            )
            return i, out_json, out_csv

        with ThreadPoolExecutor(max_workers=args.runs) as pool:
            results = list(pool.map(run, range(1, args.runs + 1)))

        # The native climate variant is chosen per process by HashMap order.
        # Require at least two runs of each variant to prove bit-identity.
        def variant_counts(res):
            counts = {"oh_appended": 0, "ozone_appended": 0}
            for _, j, _ in res:
                counts[variant_name(json.loads(j.read_text())["meta"]["nox_append_lengths"])] += 1
            return counts

        while min(variant_counts(results).values()) < 2 and len(results) < 30:
            results.append(run(len(results) + 1))

        runs, by_variant = [], {}
        for i, out_json, out_csv in results:
            data = json.loads(out_json.read_text())
            # Hash the run without the variant marker so equal physics => equal hash.
            digest = hashlib.sha256(json.dumps(data["rows"], sort_keys=True).encode()).hexdigest()
            variant = variant_name(data["meta"]["nox_append_lengths"])
            runs.append({"run": i, "variant": variant, "rows_sha256": digest})
            by_variant.setdefault(variant, []).append((digest, out_json, out_csv, data))

        dest = FIXTURES / sid
        dest.mkdir(parents=True, exist_ok=True)
        variants = {}
        for variant, items in sorted(by_variant.items()):
            hashes = sorted({h for h, *_ in items})
            variants[variant] = {
                "runs": len(items),
                "distinct_outputs": len(hashes),
                "bit_identical": len(hashes) == 1,
            }
            _, out_json, out_csv, _ = items[0]
            shutil.copy(out_json, dest / f"{variant}.json")
            shutil.copy(out_csv, dest / f"{variant}.csv")

        entry = {"runs": runs, "variants": variants}
        if len(by_variant) == 2:
            a = by_variant["oh_appended"][0][3]
            b = by_variant["ozone_appended"][0][3]
            numeric, discrete = diff_runs(a, b)
            entry["cross_variant"] = {
                "discrete_mismatches": discrete,
                "numeric_max_abs": {k: v for k, v in sorted(numeric.items()) if v["max_abs"] > 0},
            }

        if not args.no_web:
            web = {}
            for variant in by_variant:
                fixture = dest / f"{variant}.json"
                proc = subprocess.run(
                    ["node", str(WEB_REPLAY), str(fixture)],
                    cwd=ROOT,
                    check=True,
                    capture_output=True,
                    text=True,
                )
                series = json.loads(proc.stdout)
                deltas = [r["web_tgav"] - r["native_tgav"] for r in series]
                web[variant] = {
                    "open_loop_tgav_minus_native_min_c": min(deltas),
                    "open_loop_tgav_minus_native_max_c": max(deltas),
                    "open_loop_tgav_minus_native_2052_c": deltas[-1],
                }
            entry["web_replay"] = web

        report["scenarios"][sid] = entry
        print(sid, json.dumps(variants))

    (FIXTURES / "reproducibility.json").write_text(json.dumps(report, indent=2) + "\n")
    shutil.rmtree(tmp)


if __name__ == "__main__":
    main()
