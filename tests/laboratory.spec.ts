import { expect, test, type Page, type TestInfo } from '@playwright/test';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname } from 'node:path';
import type { Snapshot } from '../apps/web/src/generated/model';

const pageErrors = new WeakMap<Page, string[]>();

async function attachArtifact(
  info: TestInfo,
  name: string,
  options: { contentType: string; body: Buffer },
) {
  // Body-only attachments stay in the reporter's memory. Persist the actual
  // evidence so CI's test-results upload contains passing-run screenshots too.
  const path = info.outputPath(name);
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, options.body);
  await info.attach(name, { path, contentType: options.contentType });
}

test.beforeEach(({ page }) => {
  const errors: string[] = [];
  pageErrors.set(page, errors);
  page.on('pageerror', (error) => errors.push(error.message));
});

test.afterEach(({ page }) => {
  expect(pageErrors.get(page), 'Uncaught browser errors').toEqual([]);
});

async function openLab(page: Page) {
  await page.goto('/');
  await expect(
    page.getByRole('button', { name: 'Run', exact: true }),
  ).toBeEnabled();
}

async function displayedStep(page: Page): Promise<number> {
  return Number(
    (await page.getByTestId('step-count').textContent())?.replaceAll(',', ''),
  );
}

/** Inspect the portable, public checkpoint produced by the real Rust/WASM
 * worker. Tests never inject substitute model state or simulation rules. */
async function checkpoint(page: Page): Promise<Snapshot> {
  const downloadPromise = page.waitForEvent('download');
  await page
    .getByRole('button', { name: 'Export checkpoint as JSON', exact: true })
    .click();
  const download = await downloadPromise;
  const path = await download.path();
  expect(path).not.toBeNull();
  return JSON.parse(await readFile(path!, 'utf8')) as Snapshot;
}

async function importCheckpoint(
  page: Page,
  snapshot: unknown,
  name = 'experiment.json',
) {
  await page.getByTestId('import-checkpoint').setInputFiles({
    name,
    mimeType: 'application/json',
    buffer: Buffer.from(JSON.stringify(snapshot)),
  });
}

test('run, pause, one fixed step, and reset control the actual model', async ({
  page,
}) => {
  await openLab(page);
  const initial = await checkpoint(page);
  expect(initial.state.step).toBe(0);

  await page.getByRole('button', { name: 'Step', exact: true }).click();
  await expect.poll(() => displayedStep(page)).toBe(1);
  const stepped = await checkpoint(page);
  expect(stepped.state.step).toBe(1);
  expect(stepped.state.flux).not.toEqual(initial.state.flux);

  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect.poll(() => displayedStep(page)).toBeGreaterThan(1);
  await page.getByRole('button', { name: 'Pause', exact: true }).click();
  const paused = await checkpoint(page);
  // A deliberate wall-time interval checks pause, not numerical fidelity.
  await page.waitForTimeout(250);
  expect(await checkpoint(page)).toEqual(paused);

  await page.getByRole('button', { name: 'Step', exact: true }).click();
  await expect.poll(() => displayedStep(page)).toBe(paused.state.step + 1);
  await page.getByRole('button', { name: 'Reset', exact: true }).click();
  await expect.poll(() => displayedStep(page)).toBe(0);
  expect(await checkpoint(page)).toEqual(initial);
});

test('presets, controls, and pointer/button interventions reach Rust and diagnostics', async ({
  page,
}) => {
  await openLab(page);
  await page.getByRole('button', { name: 'Quiet sheet', exact: true }).click();
  const quiet = await checkpoint(page);
  expect(quiet.experiment.preset).toBe('quiet');
  await page.getByRole('button', { name: /What am I looking at/ }).click();
  await expect(page.getByRole('dialog')).toContainText(
    quiet.experiment.assumptions[0],
  );
  await page
    .getByRole('button', { name: 'Close model explanation', exact: true })
    .click();

  const loading = page.getByRole('slider', {
    name: 'Sheet loading',
    exact: true,
  });
  await loading.focus();
  await loading.press('End');
  await expect(loading).toHaveValue('2');
  const configured = await checkpoint(page);
  expect(configured.experiment.parameters.drive).toBe(2);
  expect(configured.experiment.interventions.at(-1)).toMatchObject({
    type: 'parameters',
    step: 0,
    parameters: { drive: 2 },
  });

  await page
    .getByRole('button', { name: 'Disturb the center', exact: true })
    .click();
  const perturbed = await checkpoint(page);
  expect(perturbed.state.flux).not.toEqual(configured.state.flux);
  expect(perturbed.state.step).toBe(0);
  expect(perturbed.experiment.interventions.at(-1)).toMatchObject({
    type: 'perturb',
    step: 0,
  });

  const canvas = page.getByTestId('field-canvas');
  const size = await canvas.boundingBox();
  expect(size).not.toBeNull();
  await canvas.click({
    position: { x: size!.width * 0.7, y: size!.height * 0.45 },
  });
  const clicked = await checkpoint(page);
  expect(clicked.state.flux).not.toEqual(perturbed.state.flux);
  expect(clicked.experiment.interventions).toHaveLength(
    perturbed.experiment.interventions.length + 1,
  );
  const intervention = clicked.experiment.interventions.at(-1)!;
  expect(intervention.type).toBe('perturb');
  if (intervention.type !== 'perturb')
    throw new Error('Pointer input was not recorded as a perturbation');
  expect(intervention.x).toBeCloseTo(0.7, 2);
  expect(intervention.y).toBeCloseTo(0.45, 2);

  await page.getByRole('button', { name: 'Step', exact: true }).click();
  await expect.poll(() => displayedStep(page)).toBe(1);
  const observed = await checkpoint(page);
  const stress = Number(await page.getByTestId('stress-value').textContent());
  expect(Number.isFinite(stress)).toBe(true);
  expect(stress).toBeGreaterThanOrEqual(0);
  expect(
    Number(
      (await page.getByTestId('events-value').textContent())?.replaceAll(
        ',',
        '',
      ),
    ),
  ).toBe(observed.state.total_events);
  const candidates = Number(
    await page.getByTestId('candidates-value').textContent(),
  );
  expect(Number.isInteger(candidates)).toBe(true);
  expect(candidates).toBeGreaterThanOrEqual(0);

  await page
    .getByRole('button', { name: 'Island cascade', exact: true })
    .click();
  const cascade = await checkpoint(page);
  expect(cascade.experiment.preset).toBe('cascade');
  expect(cascade.state.step).toBe(0);
  expect(cascade.experiment.interventions).toEqual([]);
  expect(cascade.state.flux).not.toEqual(quiet.state.flux);
});

test('browser save survives reload and resumes every field, RNG, and intervention exactly', async ({
  page,
}) => {
  await openLab(page);
  await page
    .getByRole('button', { name: 'Disturb the center', exact: true })
    .click();
  await page.getByRole('button', { name: 'Step', exact: true }).click();
  const saved = await checkpoint(page);
  await page.getByRole('button', { name: 'Save', exact: true }).click();
  await expect(page.getByTestId('status')).toContainText(/saved/i);

  await page.getByRole('button', { name: 'Step', exact: true }).click();
  const uninterrupted = await checkpoint(page);
  await page.reload();
  await expect(
    page.getByRole('button', { name: 'Load', exact: true }),
  ).toBeEnabled();
  await page.getByRole('button', { name: 'Load', exact: true }).click();
  await expect(page.getByTestId('status')).toContainText(/load|restor/i);
  expect(await checkpoint(page)).toEqual(saved);

  await page.getByRole('button', { name: 'Step', exact: true }).click();
  expect(await checkpoint(page)).toEqual(uninterrupted);
});

test('JSON export/import restores an altered experiment and its exact continuation', async ({
  page,
}) => {
  await openLab(page);
  await page
    .getByRole('button', { name: 'Stirred sheet', exact: true })
    .click();
  await page
    .getByRole('button', { name: 'Disturb the center', exact: true })
    .click();
  await page.getByRole('button', { name: 'Step', exact: true }).click();
  const saved = await checkpoint(page);
  await page.getByRole('button', { name: 'Step', exact: true }).click();
  const uninterrupted = await checkpoint(page);

  await page.getByRole('button', { name: 'Quiet sheet', exact: true }).click();
  await importCheckpoint(page, saved);
  await expect(page.getByTestId('status')).toContainText(/import|restor/i);
  expect(await checkpoint(page)).toEqual(saved);
  await expect(
    page.getByRole('button', { name: 'Stirred sheet', exact: true }),
  ).toHaveAttribute('aria-pressed', 'true');
  await page.getByRole('button', { name: 'Step', exact: true }).click();
  expect(await checkpoint(page)).toEqual(uninterrupted);
});

test('unsupported versions and invalid imported state leave the live run untouched', async ({
  page,
}) => {
  await openLab(page);
  await page
    .getByRole('button', { name: 'Disturb the center', exact: true })
    .click();
  await page.getByRole('button', { name: 'Step', exact: true }).click();
  const baseline = await checkpoint(page);

  await importCheckpoint(
    page,
    { ...baseline, version: 999 },
    'future-version.json',
  );
  await expect(page.getByRole('alert')).toContainText(/version|unsupported/i);
  expect(await checkpoint(page)).toEqual(baseline);

  const invalid = structuredClone(baseline);
  invalid.state.flux.pop();
  await importCheckpoint(page, invalid, 'truncated-state.json');
  await expect(page.getByRole('alert')).toContainText(
    /field|flux|length|dimension|state/i,
  );
  expect(await checkpoint(page)).toEqual(baseline);

  await page.getByTestId('import-checkpoint').setInputFiles({
    name: 'broken.json',
    mimeType: 'application/json',
    buffer: Buffer.from('{not JSON'),
  });
  await expect(page.getByRole('alert')).toContainText(/valid JSON/i);
  expect(await checkpoint(page)).toEqual(baseline);

  await page.getByRole('button', { name: 'Step', exact: true }).click();
  expect((await checkpoint(page)).state.step).toBe(baseline.state.step + 1);
});

test('a browser storage failure preserves the run and allows export and a later save', async ({
  page,
}) => {
  await openLab(page);
  await page.getByRole('button', { name: 'Step', exact: true }).click();
  const baseline = await checkpoint(page);
  // Fail exactly the next real IndexedDB open. The worker and model remain real.
  await page.evaluate(() => {
    const open = IDBFactory.prototype.open;
    IDBFactory.prototype.open = function () {
      IDBFactory.prototype.open = open;
      throw new DOMException('Browser storage is unavailable', 'SecurityError');
    };
  });
  await page.getByRole('button', { name: 'Save', exact: true }).click();
  await expect(page.getByRole('alert')).toContainText(/storage|unavailable/i);
  expect(await checkpoint(page)).toEqual(baseline);

  await page.getByRole('button', { name: 'Step', exact: true }).click();
  expect((await checkpoint(page)).state.step).toBe(baseline.state.step + 1);
  await page.getByRole('button', { name: 'Save', exact: true }).click();
  await expect(page.getByTestId('status')).toContainText(/saved/i);
  await expect(page.getByRole('alert')).toHaveCount(0);
});

test('records real browser progression and a desktop view without imposing a speed target', async ({
  page,
  browser,
}, testInfo) => {
  await openLab(page);
  const environment = await page.evaluate(() => ({
    userAgent: navigator.userAgent,
    hardwareConcurrency: navigator.hardwareConcurrency,
    viewport: { width: innerWidth, height: innerHeight },
  }));
  const started = performance.now();
  const samples = [{ elapsedMs: 0, step: await displayedStep(page) }];
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  for (let i = 0; i < 6; i += 1) {
    // Deliberate sampling interval for observational performance evidence.
    await page.waitForTimeout(250);
    samples.push({
      elapsedMs: performance.now() - started,
      step: await displayedStep(page),
    });
  }
  await page.getByRole('button', { name: 'Pause', exact: true }).click();
  const final = await checkpoint(page);
  expect(final.state.step).toBeGreaterThan(samples[0].step);
  await attachArtifact(testInfo, 'browser-evidence.json', {
    contentType: 'application/json',
    body: Buffer.from(
      JSON.stringify(
        {
          measuredAt: new Date().toISOString(),
          browserVersion: browser.version(),
          platform: process.platform,
          architecture: process.arch,
          ...environment,
          model: final.experiment.model,
          modelVersion: final.experiment.model_version,
          preset: final.experiment.preset,
          grid: {
            width: final.experiment.initial_conditions.width,
            height: final.experiment.initial_conditions.height,
          },
          samples,
          finalStep: final.state.step,
          interpretation:
            'UI-observed completed steps during an actual Rust/WASM run. Includes browser scheduling and presentation. Not solver-only throughput, rendering FPS, physical validation, or cross-device performance.',
        },
        null,
        2,
      ),
    ),
  });
  await attachArtifact(testInfo, 'desktop-laboratory.png', {
    contentType: 'image/png',
    body: await page.screenshot({ fullPage: true }),
  });
});

test('the narrow viewport keeps the field and complete interaction usable', async ({
  page,
}, testInfo) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await openLab(page);
  const viewport = await page.evaluate(() => ({
    content: document.documentElement.scrollWidth,
    viewport: innerWidth,
  }));
  expect(viewport.content).toBeLessThanOrEqual(viewport.viewport);
  await expect(page.getByTestId('field-canvas')).toBeVisible();
  await page.getByRole('button', { name: 'Step', exact: true }).click();
  await expect.poll(() => displayedStep(page)).toBe(1);
  await page
    .getByRole('button', { name: 'Disturb the center', exact: true })
    .click();
  const state = await checkpoint(page);
  expect(state.experiment.interventions.at(-1)?.type).toBe('perturb');
  await attachArtifact(testInfo, 'narrow-laboratory.png', {
    contentType: 'image/png',
    body: await page.screenshot({ fullPage: true }),
  });
});
