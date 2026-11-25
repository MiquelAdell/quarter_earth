#!/usr/bin/env python3
"""Regenerate the assembly parity fixtures (W1 of the assembly-workshop change).

Runs `game/examples/assembly_baseline.rs` in several separate processes per
scenario, for every requested build profile. It checks that all runs are
bit-identical within each profile (cross-profile last-bit f32 differences
are recorded, discrete differences fail), writes one canonical fixture per
scenario from the first profile (`fixtures/<scenario>.json` and `.csv`), optionally replays the recorded
emissions through the browser Hector build (Node), and writes
`references/assembly/fixtures/reproducibility.json`.

Usage (repo root):
    export CXXFLAGS="-I/opt/homebrew/include"
    cargo build -p hes-game --example assembly_baseline
    cargo build -p hes-game --example assembly_baseline --release   # if using release
    python3 references/assembly/tools/assembly_baseline.py \
        [--runs 3] [--profiles debug,release] [--no-web]

Exits non-zero if two runs of one profile differ or discrete state differs. Stdlib only. Read-only
with respect to game logic.
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


def rows_digest(data) -> str:
    return hashlib.sha256(json.dumps(data["rows"], sort_keys=True).encode()).hexdigest()


def nox_lengths_equal(nox_lengths: str) -> bool:
    # "OH:287,ozone:287": both Hector sections received every appended year.
    parts = dict(p.split(":") for p in nox_lengths.split(","))
    return parts["OH"] == parts["ozone"]


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
    """Max abs diff per numeric field (non-zero only) and discrete mismatches."""
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
                if d > numeric.get(field, {"max_abs": 0})["max_abs"]:
                    numeric[field] = {"max_abs": d, "year": ra["year"]}
            elif va != vb:
                discrete.append({"year": ra["year"], "field": key, "a": va, "b": vb})
    return dict(sorted(numeric.items())), discrete


def web_replay(fixture: Path):
    proc = subprocess.run(
        ["node", str(WEB_REPLAY), str(fixture)],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    series = json.loads(proc.stdout)
    deltas = [r["web_tgav"] - r["native_tgav"] for r in series]
    worst = max(series, key=lambda r: abs(r["web_tgav"] - r["native_tgav"]))
    return {
        "open_loop_tgav_minus_native_min_c": min(deltas),
        "open_loop_tgav_minus_native_max_c": max(deltas),
        "open_loop_tgav_minus_native_max_abs_year": worst["year"],
        "open_loop_tgav_minus_native_2052_c": deltas[-1],
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--runs", type=int, default=3, help="processes per scenario and profile")
    ap.add_argument("--profiles", default="debug,release")
    ap.add_argument("--no-web", action="store_true")
    args = ap.parse_args()

    profiles = [p for p in args.profiles.split(",") if p]
    binaries = {p: ROOT / "target" / p / "examples" / "assembly_baseline" for p in profiles}
    missing = [str(b) for b in binaries.values() if not b.exists()]
    if missing:
        sys.exit(f"missing {missing}; build first (see docstring)")

    spec = json.loads(SCENARIOS.read_text())
    tmp = Path(tempfile.mkdtemp(prefix="assembly_baseline_"))
    report = {"profiles": profiles, "runs_per_profile": args.runs, "scenarios": {}}
    failed = False

    for scenario in spec["scenarios"]:
        sid = scenario["id"]
        jobs = [(p, i) for p in profiles for i in range(1, args.runs + 1)]

        def run(job):
            profile, i = job
            out_json = tmp / f"{sid}.{profile}.{i}.json"
            out_csv = tmp / f"{sid}.{profile}.{i}.csv"
            subprocess.run(
                [str(binaries[profile]), str(SCENARIOS), sid, str(out_json), str(out_csv)],
                cwd=ROOT,
                check=True,
            )
            data = json.loads(out_json.read_text())
            return {
                "profile": profile,
                "run": i,
                "rows_sha256": rows_digest(data),
                "nox_append_lengths": data["meta"]["nox_append_lengths"],
                "json": out_json,
                "csv": out_csv,
            }

        with ThreadPoolExecutor(max_workers=len(jobs)) as pool:
            runs = list(pool.map(run, jobs))

        # Bit-identity is required between processes of the same build
        # profile. Across profiles, f32 results may differ in the last bit
        # (different LLVM optimisation); record those differences instead.
        by_profile = {p: {r["rows_sha256"] for r in runs if r["profile"] == p} for p in profiles}
        digests = {r["rows_sha256"] for r in runs}
        nox_ok = all(nox_lengths_equal(r["nox_append_lengths"]) for r in runs)
        identical = all(len(d) == 1 for d in by_profile.values())
        failed |= not (identical and nox_ok)
        cross_profile = {}
        if len(digests) > 1 and identical:
            first = {p: next(r for r in runs if r["profile"] == p) for p in profiles}
            base = json.loads(first[profiles[0]]["json"].read_text())
            for p in profiles[1:]:
                numeric, discrete = diff_runs(base, json.loads(first[p]["json"].read_text()))
                failed |= bool(discrete)
                cross_profile[f"{profiles[0]}_vs_{p}"] = {
                    "discrete_mismatches": discrete,
                    "numeric_max_abs": numeric,
                }

        shutil.copy(runs[0]["json"], FIXTURES / f"{sid}.json")
        shutil.copy(runs[0]["csv"], FIXTURES / f"{sid}.csv")

        entry = {
            "runs": [
                {k: r[k] for k in ("profile", "run", "rows_sha256", "nox_append_lengths")}
                for r in runs
            ],
            "distinct_outputs": len(digests),
            "bit_identical_within_profile": identical,
            "bit_identical_across_profiles": len(digests) == 1,
            "cross_profile": cross_profile,
            "nox_sections_equal": nox_ok,
        }
        if not args.no_web:
            entry["web_replay"] = web_replay(FIXTURES / f"{sid}.json")
        report["scenarios"][sid] = entry
        print(sid, f"runs={len(runs)} within_profile={identical} across={len(digests) == 1} nox_ok={nox_ok}")

    (FIXTURES / "reproducibility.json").write_text(json.dumps(report, indent=2) + "\n")
    shutil.rmtree(tmp)
    if failed:
        sys.exit("non-reproducible scenario(s); see reproducibility.json")


if __name__ == "__main__":
    main()
