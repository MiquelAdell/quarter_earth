import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

const source = await readFile(new URL('./tgav.js', import.meta.url), 'utf8');
const moduleSource = source.replace(
  "import initHector from './init';",
  'const initHector = async () => ({});',
);
const { Temperature } = await import(
  `data:text/javascript;base64,${Buffer.from(moduleSource).toString('base64')}`,
);

test('waits for all climate inputs before accepting emissions', async () => {
  const originalFetch = global.fetch;
  const responses = {
    '/hector/config.json': { components: {} },
    '/hector/rcp26.default_emissions.json': { ffi_emissions: 1 },
    '/hector/rcp26.to_2050.json': {
      startYear: 2020,
      data: { ffi_emissions: [0, 0, 0, 0, 0] },
    },
  };
  let releaseFetches;
  const fetchesReleased = new Promise((resolve) => {
    releaseFetches = resolve;
  });

  global.fetch = async (url) => {
    await fetchesReleased;
    return { json: async () => responses[url] };
  };

  try {
    const climate = new Temperature(2022);
    climate.setEmissions({ ffi_emissions: [4, 5] });
    assert.equal(climate.isReady(), false);

    releaseFetches();
    await new Promise((resolve) => setTimeout(resolve, 0));

    assert.equal(climate.isReady(), true);
    assert.deepEqual(climate.getEmissions(), { ffi_emissions: [4, 5] });

    climate.addEmissions({ ffi_emissions: 6 });
    assert.deepEqual(climate.getEmissions(), { ffi_emissions: [4, 5, 6] });
  } finally {
    global.fetch = originalFetch;
  }
});
