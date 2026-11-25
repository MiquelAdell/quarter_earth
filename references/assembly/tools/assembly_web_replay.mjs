// Open-loop replay of a native assembly fixture's Hector inputs through the
// browser climate path (game/assets/js/hector: hector.wasm + init.js), using
// the same bookkeeping as tgav.js `Temperature`: base scenario sliced to the
// start year, flat {source: values} map, defaults for every other key, a new
// Temperature (and Hector instance) per world phase restored via
// setEmissions, and endDate = startYear + series length.
//
// Usage: node references/assembly/tools/assembly_web_replay.mjs <fixture.json>  -> JSON on stdout
// Read-only: copies init.js to a temp dir with absolute imports and passes
// the wasm bytes directly (Node has no fetch for file URLs).
import { readFileSync, writeFileSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../../..');
const js = join(root, 'game/assets/js/hector');
const assets = join(root, 'game/assets/hector');

const init = readFileSync(join(js, 'init.js'), 'utf8')
  .replace("import wasmHector from './hector.js';",
    `import wasmHector from '${pathToFileURL(join(js, 'hector.js'))}';\n` +
    `import { readFileSync } from 'node:fs';\n` +
    `const WASM = readFileSync(${JSON.stringify(join(js, 'hector.wasm'))});`)
  .replace("import emissions from './emissions.js';",
    `import emissions from '${pathToFileURL(join(js, 'emissions.js'))}';`)
  .replace('new wasmHector()', 'wasmHector({ wasmBinary: WASM })');
if (!init.includes('wasmBinary')) throw new Error('init.js changed; update the patch');
const tmp = mkdtempSync(join(tmpdir(), 'hector-node-'));
writeFileSync(join(tmp, 'init.mjs'), init);
const { default: initHector } = await import(pathToFileURL(join(tmp, 'init.mjs')));

const load = (f) => JSON.parse(readFileSync(join(assets, f), 'utf8'));
const config = load('config.json');
const defaults = load('rcp26.default_emissions.json');
const base = load('rcp26.to_2050.json');
const outputs = { 'temperature.Tgav': { component: 'temperature', variable: 'Tgav' } };

const fixture = JSON.parse(readFileSync(process.argv[2], 'utf8'));
const rows = new Map(fixture.rows.map((r) => [r.year, r.numeric]));
const start = fixture.meta.start_year;
const end = fixture.meta.death_year;

let stored = null;
const out = [];
for (let cycle = start; cycle < end; cycle += 5) {
  const em = { startYear: base.startYear, data: {} };
  for (const k of Object.keys(base.data)) em.data[k] = base.data[k].slice(0, start - base.startYear);
  if (stored) em.data = stored;
  const hector = await initHector(config, outputs);
  for (let y = cycle; y < Math.min(cycle + 5, end); y++) {
    const row = rows.get(y + 1);
    const e = { ffi_emissions: row.hector_ffi_pgc, CH4_emissions: row.hector_ch4_tg, N2O_emissions: row.hector_n2o_tg };
    for (const k of Object.keys(defaults)) em.data[k].push(e[k] !== undefined ? e[k] : defaults[k]);
    const endDate = em.startYear + em.data.ffi_emissions.length;
    const t = hector.run(endDate, em)['temperature.Tgav'];
    out.push({ year: y + 1, web_tgav: t[t.length - 1], native_tgav: row.tgav_input_c });
  }
  stored = JSON.parse(JSON.stringify(em.data));
}
process.stdout.write(JSON.stringify(out));
