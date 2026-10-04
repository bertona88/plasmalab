import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import initialize, { Lab } from '../apps/web/src/wasm/plasmalab_core.js';

await initialize({
  module_or_path: readFileSync(
    new URL('../apps/web/src/wasm/plasmalab_core_bg.wasm', import.meta.url),
  ),
});
function send(lab, command) {
  const response = JSON.parse(lab.dispatch(JSON.stringify(command)));
  assert.equal(response.ok, true, response.error);
  return response;
}
const first = new Lab('cascade');
const second = new Lab('cascade');
const original = send(first, { type: 'observe' }).frame;
for (const lab of [first, second]) {
  send(lab, { type: 'step', count: 80 });
  send(lab, {
    type: 'perturb',
    x: 0.47,
    y: 0.52,
    strength: -0.7,
    radius: 0.06,
  });
  send(lab, { type: 'step', count: 25 });
}
assert.deepEqual(
  send(first, { type: 'snapshot' }),
  send(second, { type: 'snapshot' }),
  'WASM deterministic stepping',
);
const checkpoint = send(first, { type: 'snapshot' }).snapshot;
const resumed = new Lab('quiet');
send(resumed, { type: 'restore', snapshot: checkpoint });
assert.deepEqual(
  send(first, { type: 'step', count: 40 }),
  send(resumed, { type: 'step', count: 40 }),
  'WASM checkpoint continuation',
);
const beforeBadLoad = send(resumed, { type: 'snapshot' });
const bad = structuredClone(checkpoint);
bad.version += 1;
assert.equal(
  JSON.parse(
    resumed.dispatch(JSON.stringify({ type: 'restore', snapshot: bad })),
  ).ok,
  false,
);
assert.deepEqual(
  send(resumed, { type: 'snapshot' }),
  beforeBadLoad,
  'Invalid checkpoint must not mutate the model',
);
const reset = send(first, { type: 'reset' }).frame;
assert.deepEqual(
  reset.field,
  original.field,
  'Reset restores the original fields',
);
assert.equal(reset.step, 0);
for (const lab of [first, second, resumed]) lab.free();
console.log(
  'WASM integration: deterministic stepping, full checkpoint continuation, atomic rejection, reset passed.',
);
