#!/usr/bin/env python3
"""Check the assembly content catalog (strings.csv) against W1 data, W2 templates and the run sheet.

Errors (exit 1):
  * missing/empty en, es or ca; duplicate or malformed keys
  * placeholder sets differ between languages, or differ from the declared params
  * a card/metric/mandate ID that is not in the W1 fixtures; an English card name that differs
    from the engine name; a template card cost that differs from scenarios.json
  * a derived param that cannot be computed from the fixtures/baseline
  * numbers that disagree between en, es and ca once rendered (decimal comma normalized)
  * a card name quoted inside another string that does not use that locale's card name
  * a user-visible string in a W2 template data block with no catalog key, a stale source,
    or a catalog `en` that no longer matches the template text
  * an English string over its max_chars, or a card effect over 160 English characters
  * the es/ca run sheets drift structurally from the English one (headings, table rows,
    checklist items, code spans, numbers)

Flags (reported, exit 0): es/ca strings longer than their max_chars (the 35% headroom W2 proved),
listed separately for short labels where the ratio means little; catalog rows that share an
English text but translate it differently (they collide in game/src/locales.rs, which looks
strings up by a hash of the English text).

Usage:
  python3 references/assembly/content/check.py                 # run all checks
  python3 references/assembly/content/check.py --write-gaps    # also rewrite ingame-gaps.csv
  python3 references/assembly/content/check.py --render DIR    # also write es/ca/en copies of the
                                                               # W2 templates to DIR/<lang>/ (proof only)
Needs Python 3.9+ and Node (for evaluating the template data blocks).
"""
import argparse, collections, csv, json, math, pathlib, re, subprocess, sys

HERE = pathlib.Path(__file__).resolve().parent
ASSEMBLY = HERE.parent
REPO = ASSEMBLY.parent.parent
PRINT = ASSEMBLY / "print"
FIX = ASSEMBLY / "fixtures"
LANGS = ("en", "es", "ca")
NBSP = " "

TEMPLATES = {
    "deck-a": ["sheet-01-04-deck-a.html"], "deck-b": ["optional-deck-b.html"],
    "decks": ["sheet-01-04-deck-a.html", "optional-deck-b.html"],
    "mandate-fl": ["sheet-05-mandate-food-land.html"], "mandate-ei": ["sheet-06-mandate-energy-industry.html"],
    "mandate-ls": ["sheet-07-mandate-living-standards.html"], "mandate-lw": ["sheet-08-mandate-living-world.html"],
    "mandates": ["sheet-05-mandate-food-land.html", "sheet-06-mandate-energy-industry.html",
                 "sheet-07-mandate-living-standards.html", "sheet-08-mandate-living-world.html"],
    "mats": ["sheet-09-10-coalition-mats.html"], "slips": ["extra-package-slips.html"],
    "predictions": ["extra-prediction-slips.html"], "votes": ["extra-voting-cards.html"],
    "wall": ["extra-wall-timeline.html"],
}
DOC_TITLE = {"sheet-01-04-deck-a.html": "deck_a", "optional-deck-b.html": "deck_b",
             "sheet-05-mandate-food-land.html": "mandate", "sheet-06-mandate-energy-industry.html": "mandate",
             "sheet-07-mandate-living-standards.html": "mandate", "sheet-08-mandate-living-world.html": "mandate",
             "sheet-09-10-coalition-mats.html": "mats", "extra-package-slips.html": "slips",
             "extra-prediction-slips.html": "predictions", "extra-voting-cards.html": "votes",
             "extra-wall-timeline.html": "wall"}
SKIP_FIELDS = {"id", "theme", "cls", "glyph", "no", "cost", "shape"}
MANDATE_ORDER = ["food_land", "energy_industry", "living_standards", "living_world"]
SHORT_LABEL = 15          # en strings shorter than this get their own flag list
EFFECT_MAX_EN = 160       # print/README.md: card effect text ≤ ~160 English characters
GAP_SOURCES = ["views/report.rs", "workshop.rs", "views/session/plan.rs", "views/session/mod.rs",
               "views/world.rs", "views/ending.rs", "splash/menu.rs"]

NODE_JS = r"""
const fs = require('fs'), vm = require('vm');
const html = fs.readFileSync(process.argv[process.argv.length - 1], 'utf8');
const m = html.match(/<script>([\s\S]*?)\/\* ===== renderer/);
if (!m) { process.stdout.write('null'); process.exit(0); }
const pick = n => `typeof ${n} !== 'undefined' ? ${n} : null`;
const code = m[1] + `\n;({STRINGS: ${pick('STRINGS')}, TABLE: ${pick('TABLE')}, CARDS: ${pick('CARDS')}, MATS: ${pick('MATS')}})`;
process.stdout.write(JSON.stringify(vm.runInNewContext(code, {})));
"""

# ------------------------------------------------------------------ catalog
def load_catalog():
    rows = list(csv.DictReader(open(HERE / "strings.csv", encoding="utf-8", newline="")))
    for r in rows:
        for lang in LANGS:
            r[lang] = (r.get(lang) or "").replace("\\n", "\n")
    return rows

def parse_params(s):
    out = {}
    for part in filter(None, (p.strip() for p in (s or "").split(";"))):
        name, _, spec = part.partition("=")
        out[name.strip()] = spec.strip()
    return out

PH = re.compile(r"%\{([a-z_][a-z0-9_]*)\}")

# ------------------------------------------------------------------ W1 data
def load_w1():
    sc = json.load(open(FIX / "scenarios.json"))
    a = json.load(open(FIX / "a_no_action" / "ozone_appended.json"))
    base = (ASSEMBLY / "model-baseline.md").read_text()
    r0, r1 = a["rows"][0], a["rows"][1]
    mix = r0["discrete"]["mix_share_5pct_units"]
    def grab(rx):
        m = re.search(rx, base)
        if not m:
            raise ValueError(f"model-baseline.md no longer matches /{rx}/")
        return float(m.group(1))
    solar = grab(r"TransferMixShare\(Coal Power → Solar PV, (\d+)\)")
    phase = grab(r"TransferMixShare\(Coal Power → Retired Coal Capacity, (\d+)\)")
    organic = grab(r"TransferMixShare\(Industrial Crop Ag → Organic Crop Ag, (\d+)\)")
    protect = grab(r"ProtectLand \+([\d.]+)")
    lux = grab(r"Outlook \+(\d+)")
    cats = grab(r"WorldVariable ExtinctionRate (-?[\d.]+)")
    n0, n1 = r0["numeric"], r1["numeric"]
    coal, solar0 = mix["electricity"]["Coal Power Generation"], mix["electricity"]["Solar PV"]
    ind, org = mix["plant_calories"]["Industrial Crop Ag"], mix["plant_calories"]["Organic Crop Ag"]
    d = n0["demand"]
    derived = {
        "coal_power_from_pct": coal * 5, "coal_power_after_solar_pct": (coal - solar) * 5,
        "coal_power_after_phaseout_pct": (coal - phase) * 5,
        "solar_from_pct": solar0 * 5, "solar_to_pct": (solar0 + solar) * 5,
        "industrial_crop_from_pct": ind * 5, "industrial_crop_to_pct": (ind - organic) * 5,
        "organic_crop_from_pct": org * 5, "organic_crop_to_pct": (org + organic) * 5,
        "organic_shift_points": organic * 5,
        "protect_added_pct": protect * 100, "protected_from_pct": n0["protected_land_fraction"] * 100,
        "protected_to_pct": (n0["protected_land_fraction"] + protect) * 100,
        "luxury_outlook_gain": lux, "cats_extinction_drop": abs(cats),
        "extinction_2022": n0["extinction_rate"], "emissions_2022": n0["emissions_gtco2eq"],
        "temperature_2022": n0["temperature_c"], "temperature_2023": n1["temperature_c"],
        "outlook_2022": n0["outlook"], "outlook_2023": n1["outlook"],
        "electricity_2022_pwh": n0["produced"]["electricity_kwh"] * 1e-12,
        "population_2022_billion": n0["population"] / 1e9,
        "land_use_2022_pct": n0["land_use_fraction"] * 100,
        "animal_calorie_share_2022_pct": 100 * d["animal_calories_kcal"] / (d["animal_calories_kcal"] + d["plant_calories_kcal"]),
    }
    cards = r0["discrete"]["cards"]            # all 21 workshop cards: id -> {name, ...}
    metrics = set(a["meta"]["units"]) | set(n0) | {f"produced.{k}" for k in n0["produced"]}
    mandates = {m for c in sc["opening_cards"] for m in c["mandates"]}
    costs = {c["id"]: c["cost"] for c in sc["opening_cards"]}
    return dict(derived=derived, cards=cards, metrics=metrics, mandates=mandates, costs=costs,
                veg75="75% of animal-calorie demand" in base)

# ------------------------------------------------------------------ formatting
def fmt(value, kind, lang):
    if kind == "pct":
        s = str(int(round(value)))
        return s + ("%" if lang == "en" else NBSP + "%")
    if kind == "int":
        return str(int(round(value)))
    if kind in ("dec1", "dec2"):
        s = f"{value:.{kind[-1]}f}"
        return s if lang == "en" else s.replace(".", ",")
    raise ValueError(f"unknown format {kind}")

def render(row, lang, derived, runtime="⟨{}⟩"):
    params = parse_params(row["params"])
    def sub(m):
        spec = params.get(m.group(1), "")
        if spec == "runtime" or not spec:
            return runtime.format(m.group(1))
        src, _, kind = spec.partition(":")
        return fmt(derived[src], kind, lang)
    return PH.sub(sub, row[lang])

NUM = re.compile(r"\d+(?:[.,]\d+)?")
def numbers(s):
    return sorted(n.replace(",", ".") for n in NUM.findall(s))

# ------------------------------------------------------------------ templates
def template_data(path):
    out = subprocess.run(["node", "-e", NODE_JS, str(path)], capture_output=True, text=True, check=True).stdout
    return json.loads(out)

def leaves(obj, path):
    if isinstance(obj, dict):
        for k, v in obj.items():
            yield from leaves(v, f"{path}.{k}" if path else k)
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            yield from leaves(v, f"{path}[{i}]")
    else:
        yield path, obj

def is_text(path, value):
    field = re.sub(r"\[\d+\]$", "", path).rsplit(".", 1)[-1]
    if field in SKIP_FIELDS or not isinstance(value, str):
        return False
    v = value.strip()
    return len(v) > 1 and "{{" not in v and not re.fullmatch(r"[\d\s→_]+", v)

def set_path(obj, path, value):
    parts = re.findall(r"[^.\[\]]+|\[\d+\]", path)
    cur = obj
    for p in parts[:-1]:
        cur = cur[int(p[1:-1])] if p.startswith("[") else cur[p]
    last = parts[-1]
    if last.startswith("["):
        cur[int(last[1:-1])] = value
    else:
        cur[last] = value

# ------------------------------------------------------------------ run sheet parity
def md_profile(text):
    body = re.sub(r"```.*?```", "", text, flags=re.S)
    code = sorted(re.findall(r"`([^`]+)`", body))
    no_code = re.sub(r"`[^`]+`", "", body)
    no_links = re.sub(r"\]\([^)]*\)", "]", no_code)
    return {
        "headings": [len(m) for m in re.findall(r"^(#+) ", body, re.M)],
        "table rows": len(re.findall(r"^\|", body, re.M)),
        "checklist items": len(re.findall(r"^\s*- \[ \]", body, re.M)),
        "list items": len(re.findall(r"^\s*(?:-|\d+\.) ", body, re.M)),
        "code spans": code,
        "numbers": numbers(no_links),
    }

# ------------------------------------------------------------------ in-game gap list
def ingame_gaps(cards):
    es_csv = collections.defaultdict(set)
    for r in csv.reader(open(REPO / "game" / "translations" / "es.csv", encoding="utf-8")):
        if len(r) >= 2:
            es_csv[r[0]].add(r[1])
    items = collections.OrderedDict()
    for rel in GAP_SOURCES:
        src = (REPO / "game" / "src" / rel).read_text()
        for m in re.finditer(r't!\(\s*"((?:[^"\\]|\\.)*)"', src):
            s = m.group(1)
            if re.search(r"[A-Za-z]", re.sub(r"\{[^}]*\}", "", s)):
                items.setdefault(s, []).append(rel)
    for t in ["Energy", "Food & Agriculture", "Land & Biodiversity", "Industry, Transport & Geoengineering", "Society & Economy"]:
        items.setdefault(t, []).append("workshop.rs (WorkshopTheme::title)")
    for cid, c in sorted(cards.items(), key=lambda kv: kv[1]["name"]):
        items.setdefault(c["name"], []).append(f"WORKSHOP.world card {cid[:8]}")
    rows = []
    for s, srcs in items.items():
        es = " | ".join(sorted(es_csv.get(s, [])))
        rows.append({"english": s, "sources": "; ".join(dict.fromkeys(srcs)), "es_csv": es, "ca": ""})
    return rows

# ------------------------------------------------------------------ main
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--write-gaps", action="store_true")
    ap.add_argument("--render", metavar="DIR")
    a = ap.parse_args()
    errors, flags, short_flags, collisions = [], [], [], []
    err = errors.append

    rows = load_catalog()
    w1 = load_w1()
    derived = w1["derived"]
    by_key = {}
    for r in rows:
        k = r["key"]
        if k in by_key:
            err(f"duplicate key {k}")
        by_key[k] = r
        if not re.fullmatch(r"[a-z][a-z0-9_]*(\.[a-z0-9_.-]+)+", k):
            err(f"malformed key {k}")
        for lang in LANGS:
            if not r[lang].strip():
                err(f"{k}: empty {lang}")

    # placeholders and params
    for r in rows:
        k, params = r["key"], parse_params(r["params"])
        sets = {lang: set(PH.findall(r[lang])) for lang in LANGS}
        if not (sets["en"] == sets["es"] == sets["ca"]):
            err(f"{k}: placeholder mismatch {sets}")
        if sets["en"] != set(params):
            err(f"{k}: placeholders {sorted(sets['en'])} vs params {sorted(params)}")
        for name, spec in params.items():
            if spec == "runtime":
                continue
            src, _, kind = spec.partition(":")
            if src not in derived:
                err(f"{k}: param {name} uses unknown derivation {src}")
            elif kind not in ("pct", "int", "dec1", "dec2"):
                err(f"{k}: param {name} has unknown format {kind}")

    # IDs
    names = {lang: {} for lang in LANGS}
    for r in rows:
        parts = r["key"].split(".")
        if parts[0] == "card":
            cid = parts[1]
            if cid not in w1["cards"]:
                err(f"{r['key']}: card ID not in the workshop world (fixtures 2022 row)")
            elif parts[2] == "name":
                if r["en"] != w1["cards"][cid]["name"]:
                    err(f"{r['key']}: en '{r['en']}' != engine name '{w1['cards'][cid]['name']}'")
                for lang in LANGS:
                    names[lang][cid] = r[lang]
        elif parts[0] == "metric" and parts[1] != "direction" and parts[1] != "shortage":
            mid = ".".join(parts[1:-1])
            if mid not in w1["metrics"]:
                err(f"{r['key']}: metric ID {mid} not in fixtures meta.units / numeric row")
        elif parts[0] == "mandate" and parts[1] not in w1["mandates"]:
            err(f"{r['key']}: mandate ID {parts[1]} not in scenarios.json")

    # numbers agree across languages; derived values only through params
    for r in rows:
        rendered = {lang: render(r, lang, derived, runtime="") for lang in LANGS}
        nums = {lang: numbers(rendered[lang]) for lang in LANGS}
        if not (nums["en"] == nums["es"] == nums["ca"]):
            err(f"{r['key']}: numbers differ {nums}")
    veg = by_key.get("card.692486e9-b3d0-47cb-8f42-05ab925b0d93.effect")
    if veg and ("Three-quarters" in veg["en"]) != w1["veg75"]:
        err("Vegetarian Mandate effect says 'Three-quarters' but the baseline no longer gives 75%")

    # card names quoted in other strings use the locale's card name
    for r in rows:
        for cid, en_name in names["en"].items():
            if r["key"] == f"card.{cid}.name" or en_name not in r["en"]:
                continue
            for lang in ("es", "ca"):
                if names[lang][cid] not in r[lang]:
                    err(f"{r['key']}: mentions '{en_name}' but {lang} lacks '{names[lang][cid]}'")

    # mat table codes match the mandate codes
    codes = {lang: " · ".join(by_key[f"mandate.{m}.code"][lang] for m in MANDATE_ORDER) for lang in LANGS}
    for lang in LANGS:
        if by_key["print.mat.tables"][lang] != codes[lang]:
            err(f"print.mat.tables {lang} '{by_key['print.mat.tables'][lang]}' != mandate codes '{codes[lang]}'")
    for lang in LANGS:
        cs = [by_key[f"mandate.{m}.code"][lang] for m in MANDATE_ORDER]
        if len(set(cs)) != 4:
            err(f"mandate codes not unique in {lang}: {cs}")

    # template coverage
    src_map = {}
    for r in rows:
        for tok in filter(None, (t.strip() for t in r["sources"].split(";"))):
            alias, _, path = tok.partition(":")
            if alias not in TEMPLATES:
                err(f"{r['key']}: unknown template alias {alias}")
                continue
            for f in TEMPLATES[alias]:
                if (f, path) in src_map:
                    err(f"{f}:{path} claimed by {src_map[(f, path)]} and {r['key']}")
                src_map[(f, path)] = r["key"]
    data = {}
    covered = 0
    for f in sorted({f for fs in TEMPLATES.values() for f in fs}):
        data[f] = template_data(PRINT / f)
        for block, obj in data[f].items():
            if obj is None:
                continue
            for path, value in leaves(obj, block):
                if not is_text(path, value):
                    continue
                key = src_map.get((f, path))
                if key is None:
                    err(f"{f}:{path} '{value}' has no catalog key")
                    continue
                covered += 1
                en = render(by_key[key], "en", derived, runtime="{}")
                if en != value:
                    err(f"{f}:{path} template text '{value}' != catalog {key} en '{en}'")
        if data[f].get("CARDS"):
            for c in data[f]["CARDS"]:
                if c["id"] in w1["costs"] and c["cost"] != w1["costs"][c["id"]]:
                    err(f"{f}: card {c['id'][:8]} cost {c['cost']} != scenarios.json {w1['costs'][c['id']]}")
    for (f, path), key in src_map.items():
        obj = data.get(f, {})
        try:
            block, rest = path.split(".", 1) if "." in path.split("[")[0] else (path.split("[")[0], path[len(path.split("[")[0]):])
            cur = obj[block]
            for p in re.findall(r"[^.\[\]]+|\[\d+\]", rest):
                cur = cur[int(p[1:-1])] if p.startswith("[") else cur[p]
        except (KeyError, IndexError, TypeError):
            err(f"{key}: stale source {f}:{path}")

    # line fit
    for r in rows:
        if not r["max_chars"]:
            continue
        mx = int(r["max_chars"])
        ln = {lang: len(render(r, lang, derived, runtime="00")) for lang in LANGS}
        if ln["en"] > mx:
            err(f"{r['key']}: en {ln['en']} chars > max_chars {mx}")
        for lang in ("es", "ca"):
            if ln[lang] > mx:
                (short_flags if ln["en"] < SHORT_LABEL else flags).append(
                    f"{r['key']} [{lang}] {ln[lang]} > {mx} (en {ln['en']}): {render(r, lang, derived)!r}")
        if r["key"].endswith(".effect") and ln["en"] > EFFECT_MAX_EN:
            err(f"{r['key']}: en effect {ln['en']} chars > {EFFECT_MAX_EN}")

    # locales.rs hash collisions
    by_en = collections.defaultdict(list)
    for r in rows:
        by_en[r["en"]].append(r)
    for en, rs in by_en.items():
        if len(rs) > 1 and (len({x["es"] for x in rs}) > 1 or len({x["ca"] for x in rs}) > 1):
            collisions.append(f"'{en}': " + ", ".join(f"{x['key']}→{x['es']!r}/{x['ca']!r}" for x in rs))

    # same English as game/translations/es.csv but a different Spanish text: these collide if the
    # catalog is registered next to es.csv in the hash-of-English backend, and the screen and the
    # paper then show different words for the same thing.
    es_csv = collections.defaultdict(set)
    for rr in csv.reader(open(REPO / "game" / "translations" / "es.csv", encoding="utf-8")):
        if len(rr) >= 2:
            es_csv[rr[0]].add(rr[1])
    es_conflicts = [f"{r['key']} '{r['en']}': catalog {r['es']!r} vs es.csv {' | '.join(sorted(es_csv[r['en']]))!r}"
                    for r in rows if r["en"] in es_csv and r["es"] not in es_csv[r["en"]]]

    # run sheet parity
    en_text = (ASSEMBLY / "run-sheet.md").read_text()
    en_prof = md_profile(en_text)
    for lang in ("es", "ca"):
        p = HERE / f"run-sheet.{lang}.md"
        if not p.exists():
            err(f"missing {p.name}")
            continue
        text = p.read_text()
        # catalog phrases quoted in the English run sheet must appear in the translation (case-insensitive)
        for r in rows:
            en_s, tr_s = r["en"].rstrip("."), r[lang].rstrip(".")
            if PH.search(en_s) or len(en_s) < 2 or not r["key"].startswith(("mandate.", "card.", "prompt.", "role.", "metric.")):
                continue
            if re.search(r"(?<![\w])" + re.escape(en_s) + r"(?![\w])", en_text) and tr_s.lower() not in text.lower():
                err(f"{p.name}: run-sheet.md quotes {r['key']} '{en_s}' but the translation lacks '{tr_s}'")
        prof = md_profile(text)
        for field, v in en_prof.items():
            if prof[field] != v:
                if isinstance(v, list):
                    a_, b_ = collections.Counter(v), collections.Counter(prof[field])
                    err(f"{p.name}: {field} differ (only en: {sorted((a_ - b_).elements())}; only {lang}: {sorted((b_ - a_).elements())})")
                else:
                    err(f"{p.name}: {field} {prof[field]} != en {v}")

    # in-game gaps
    gaps = ingame_gaps(w1["cards"])
    if a.write_gaps:
        with open(HERE / "ingame-gaps.csv", "w", newline="", encoding="utf-8") as fh:
            w = csv.DictWriter(fh, ["english", "sources", "es_csv", "ca"], lineterminator="\n")
            w.writeheader(); w.writerows(gaps)

    # optional render for proofs
    if a.render:
        out = pathlib.Path(a.render)
        for lang in LANGS:
            (out / lang).mkdir(parents=True, exist_ok=True)
            for f, objs in data.items():
                html = (PRINT / f).read_text()
                objs = json.loads(json.dumps(objs))
                for (ff, path), key in src_map.items():
                    if ff == f:
                        set_path(objs, path, render(by_key[key], lang, derived, runtime="{}"))
                code = "".join(f"const {n} = {json.dumps(v, ensure_ascii=False, indent=1)};\n" for n, v in objs.items() if v is not None)
                html = re.sub(r"(<script>)[\s\S]*?(?=/\* ===== renderer)", lambda m: m.group(1) + "\n" + code, html, count=1)
                html = html.replace('<html lang="en">', f'<html lang="{lang}">', 1)
                html = re.sub(r"<title>.*?</title>", f"<title>{by_key['print.doc_title.' + DOC_TITLE[f]][lang]}</title>", html, count=1)
                (out / lang / f).write_text(html)

    # report
    ns = collections.Counter(r["key"].split(".")[0] for r in rows)
    print(f"catalog: {len(rows)} keys × 3 languages ({', '.join(f'{k} {v}' for k, v in sorted(ns.items()))})")
    print(f"templates: {len(data)} files, {covered} visible strings matched to catalog keys")
    print(f"derived params: {len(derived)} values from fixtures/a_no_action 2022–2023 rows, scenarios.json and model-baseline.md")
    missing_es = sum(1 for g in gaps if not g["es_csv"])
    print(f"in-game strings an assembly session can show: {len(gaps)}; Catalan source: 0; also missing in es.csv: {missing_es}")
    print(f"\nERRORS ({len(errors)})")
    for e in errors:
        print("  " + e)
    print(f"\nLINE-FIT FLAGS: es/ca over the tested 35% headroom ({len(flags)})")
    for x in flags:
        print("  " + x)
    print(f"\nLINE-FIT FLAGS, short labels (<{SHORT_LABEL} en chars; ratio unreliable, verify in the A4 proof) ({len(short_flags)})")
    for x in short_flags:
        print("  " + x)
    print(f"\nLOCALES.RS COLLISIONS: catalog rows with the same English, different translation ({len(collisions)})")
    for x in collisions:
        print("  " + x)
    print(f"\nES.CSV CONFLICTS: same English as game/translations/es.csv, different Spanish ({len(es_conflicts)})")
    for x in es_conflicts:
        print("  " + x)
    print("\nRESULT: " + ("FAIL" if errors else "PASS"))
    sys.exit(1 if errors else 0)

if __name__ == "__main__":
    main()
