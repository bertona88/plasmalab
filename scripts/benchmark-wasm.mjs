import { readFileSync } from 'node:fs';
import { performance } from 'node:perf_hooks';
import os from 'node:os';
import initialize, { Lab } from '../apps/web/src/wasm/plasmalab_core.js';

await initialize({
  module_or_path: readFileSync(
    new URL('../apps/web/src/wasm/plasmalab_core_bg.wasm', import.meta.url),
  ),
});
const samples = [];
for (let repeat = 0; repeat < 3; repeat++) {
  const lab = new Lab('cascade');
  lab.dispatch('{"type":"step","count":120}');
  const start = performance.now();
  let bytes = 0;
  for (let i = 0; i < 100; i++) {
    const raw = lab.dispatch('{"type":"step","count":2}');
    bytes = raw.length;
    const response = JSON.parse(raw);
    if (!response.ok) throw new Error(response.error);
  }
  const ms = performance.now() - start;
  samples.push({
    steps: 200,
    milliseconds: ms,
    steps_per_second: 200000 / ms,
    frame_bytes: bytes,
  });
  lab.free();
}
console.log(
  JSON.stringify(
    {
      runtime: process.version,
      platform: `${os.platform()} ${os.arch()}`,
      cpu: os.cpus()[0]?.model ?? 'not exposed',
      grid: [128, 80],
      dt: 0.12,
      preset: 'cascade',
      scope:
        'WASM evolution + observation + JSON encode/decode, two steps per frame. Excludes worker messaging and canvas.',
      samples,
    },
    null,
    2,
  ),
);
