import { spawnSync } from 'node:child_process';
import { mkdirSync } from 'node:fs';

function run(command, args) {
  const result = spawnSync(command, args, { stdio: 'inherit' });
  if (result.error) {
    console.error(
      `Could not run ${command}. See README.md for Rust/WASM setup.`,
      result.error.message,
    );
    process.exit(1);
  }
  if (result.status !== 0) process.exit(result.status ?? 1);
}

mkdirSync('apps/web/src/wasm', { recursive: true });
run('cargo', [
  'build',
  '--locked',
  '--release',
  '--target',
  'wasm32-unknown-unknown',
  '-p',
  'plasmalab-core',
]);
run('wasm-bindgen', [
  '--target',
  'web',
  '--out-dir',
  'apps/web/src/wasm',
  '--out-name',
  'plasmalab_core',
  'target/wasm32-unknown-unknown/release/plasmalab_core.wasm',
]);
