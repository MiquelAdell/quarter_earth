#!/usr/bin/env python3
"""Print-proof every template in this folder with headless Chrome.

For each HTML file it reports:
  * the PDF page count and page size (via pdfinfo);
  * which clipping boxes (overflow:hidden — every card, slip, panel and the page
    itself) hide content, so the check fails instead of silently cutting text;
  * a STRESS run: card fields get long English sample text and every other visible
    string is stretched by --stretch (default 1.35, i.e. ~35% longer, standing in for
    Spanish/Catalan). Any text box whose content overflows its box is listed.

Usage:  python3 check-fit.py [--stretch 1.35] [--out /tmp/proof] [files...]
Needs Google Chrome and poppler's pdfinfo. Writes only to --out.
"""
import argparse, pathlib, re, subprocess, sys, tempfile, html

CHROME = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
HERE = pathlib.Path(__file__).resolve().parent

# Long but plausible English card text (longer than the July card proposal's copy).
SAMPLE = {
    "NAME": "Remediate and Protect Ecosystems",
    "EFFECT": ("Protects an extra 30% of remaining wild land from development. Extinction pressure falls "
               "over the next cycles; land available for farming and energy shrinks."),
    "REQUIRES": "Expand Nature Preserves",
    "EXCLUDES": "Luxury for All, Degrowth in Developed Regions",
    "COST": "15", "ID": "remediate_protect", "THEME": "land",
}
EVIDENCE = {"LABEL": "Land used by agriculture", "VALUE": "38.4", "UNIT": "% of habitable land",
            "NOTE": "Every new reserve competes with farmland for the same hectares"}

CHECK_JS = r"""
<script>
(function(){
  const stretch = %STRETCH%;
  if (stretch !== 1) {
    const w = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    const nodes = []; while (w.nextNode()) nodes.push(w.currentNode);
    nodes.forEach(n => { const t = n.nodeValue; if (t.trim().length > 1) {
      const extra = Math.round(t.length * (stretch - 1));
      n.nodeValue = t + (" " + t).repeat(5).slice(0, extra); } });
  }
  const bad = [];
  document.querySelectorAll("body *").forEach(el => {
    if (el.tagName === "SCRIPT") return;
    const cs = getComputedStyle(el);
    if (cs.overflow === "visible") return;  // only boxes that clip can hide text
    if (el.scrollHeight > el.clientHeight + 1 || el.scrollWidth > el.clientWidth + 1) {
      if (el.clientHeight === 0 && el.clientWidth === 0) return;
      bad.push(el.tagName.toLowerCase() + "." + (el.className || "") + " :: " + el.textContent.trim().slice(0, 50));
    }
  });
  const pre = document.createElement("pre"); pre.id = "fit-report";
  pre.textContent = "FIT-REPORT " + JSON.stringify(bad); document.body.appendChild(pre); pre.style.display = "none";
})();
</script>
"""

def run(cmd):
    return subprocess.run(cmd, capture_output=True, text=True, timeout=120)

def stressed(src: str, stretch: float) -> str:
    def fill(m):
        return SAMPLE.get(m.group(2), m.group(0))
    s = re.sub(r"\{\{CARD_(\$\{n\}|\d+)_([A-Z]+)\}\}", fill, src)
    s = re.sub(r"\{\{[A-Z]{2}_EV_\d_([A-Z]+)\}\}", lambda m: EVIDENCE.get(m.group(1), m.group(0)), s)
    return s.replace("</body>", CHECK_JS.replace("%STRETCH%", str(stretch)) + "</body>")

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--stretch", type=float, default=1.35)
    ap.add_argument("--out", default=tempfile.mkdtemp(prefix="qe-proof-"))
    ap.add_argument("files", nargs="*")
    a = ap.parse_args()
    out = pathlib.Path(a.out); out.mkdir(parents=True, exist_ok=True)
    files = [pathlib.Path(f).resolve() for f in a.files] or sorted(HERE.glob("*.html"))
    failed = False
    for f in files:
        pdf = out / (f.stem + ".pdf")
        run([CHROME, "--headless=new", "--no-pdf-header-footer", f"--print-to-pdf={pdf}", f.as_uri()])
        info = run(["pdfinfo", str(pdf)]).stdout
        pages = re.search(r"Pages:\s+(\d+)", info); size = re.search(r"Page size:\s+(.+)", info)
        for label, stretch in (("as-is", 1.0), (f"stress x{a.stretch}", a.stretch)):
            tmp = out / f"{f.stem}.{label.split()[0]}.html"
            tmp.write_text(stressed(f.read_text(), stretch))
            dom = run([CHROME, "--headless=new", "--virtual-time-budget=2000", "--dump-dom", tmp.as_uri()]).stdout
            m = re.search(r"FIT-REPORT (\[.*?\])</pre>", dom, re.S)
            bad = html.unescape(m.group(1)) if m else "[report missing]"
            ok = bad == "[]"
            failed |= not ok
            print(f"{f.name:42} pages={pages.group(1) if pages else '?':2} {size.group(1).strip() if size else '?':32} {label:13} {'OK' if ok else 'OVERFLOW ' + bad}")
    print(f"PDFs in {out}")
    sys.exit(1 if failed else 0)

if __name__ == "__main__":
    main()
