<script lang="ts">
  import { onMount } from 'svelte';
  import Icon from './Icon.svelte';
  import Sparkline from './Sparkline.svelte';
  import { createExecution } from './lib/execution';
  import type { Frame, Command } from './lib/contracts';
  import {
    saveLocal,
    loadLocal,
    exportSnapshot,
    readSnapshotFile,
  } from './lib/persistence';
  import { createFieldRenderer } from './renderer';

  type Execution = Awaited<ReturnType<typeof createExecution>>;
  type Parameters = Frame['experiment']['parameters'];
  let execution: Execution | null = null;
  let catalog: Execution['catalog'] | null = null;
  let frame: Frame | null = null;
  let canvas: HTMLCanvasElement;
  let importInput: HTMLInputElement;
  let aboutDialog: HTMLDialogElement;
  let renderer: ReturnType<typeof createFieldRenderer> | null = null;
  let running = false;
  let loading = true;
  let fileBusy = false;
  let status = 'Preparing your experiment…';
  let error = '';
  let contours = true;
  let candidates = true;
  let strength = -0.9;
  const brushRadius = 0.07;
  let cursor: { x: number; y: number } | null = null;
  let dragging = false;
  let lastPerturb = 0;
  let selectedPreset = 'tearing';
  let history: { step: number; energy: number; events: number }[] = [];
  let timer: ReturnType<typeof setTimeout> | undefined;
  let destroyed = false;
  let stepping = false;
  const repository = 'https://github.com/bertona88/plasmalab';
  const presetMeta: Record<
    string,
    { number: string; short: string; shape: string }
  > = {
    quiet: { number: '01', short: 'A quiet sheet', shape: 'quiet' },
    tearing: { number: '02', short: 'At the threshold', shape: 'tearing' },
    cascade: { number: '03', short: 'A restless field', shape: 'cascade' },
    driven: { number: '04', short: 'Keep it moving', shape: 'driven' },
  };
  $: energyHistory = history.map((point) => point.energy);
  $: eventHistory = history.map((point) => point.events);
  $: preset = catalog?.presets.find((item) => item.id === selectedPreset);
  $: if (renderer && frame)
    renderer.draw(frame, {
      contours,
      candidates,
      cursor,
      cursorRadius: brushRadius,
    });

  function accept(next: Frame, clear = false) {
    frame = next;
    selectedPreset = next.experiment.preset;
    if (
      clear ||
      (history.length && next.step < history[history.length - 1].step)
    )
      history = [];
    const sample = {
      step: next.step,
      energy: next.observations.stress_energy,
      events: next.observations.events_this_step,
    };
    if (history.length && history[history.length - 1].step === next.step)
      history = [...history.slice(0, -1), sample];
    else history = [...history.slice(-99), sample];
  }
  function fail(reason: unknown) {
    error = reason instanceof Error ? reason.message : String(reason);
    pause();
  }
  async function send(command: Command, clear = false) {
    if (!execution) return;
    try {
      const next = await execution.command(command);
      if (!destroyed) accept(next, clear);
    } catch (reason) {
      fail(reason);
    }
  }
  async function tick() {
    if (!running || !execution || destroyed || stepping) return;
    stepping = true;
    await send({ type: 'step', count: 2 });
    stepping = false;
    if (running && !destroyed) timer = setTimeout(tick, 40);
  }
  function pause() {
    running = false;
    clearTimeout(timer);
  }
  function toggleRun() {
    if (running) pause();
    else {
      running = true;
      void tick();
    }
  }
  async function choosePreset(id: string) {
    pause();
    error = '';
    await send({ type: 'reset', preset: id }, true);
    status = `${catalog?.presets.find((item) => item.id === id)?.name ?? id} ready. Press Run or disturb the field.`;
  }
  async function reset() {
    pause();
    error = '';
    await send({ type: 'reset' }, true);
    status =
      'Initial field and starting parameters restored. Intervention history cleared.';
  }
  async function step() {
    pause();
    await send({ type: 'step', count: 1 });
  }
  async function configure(key: keyof Parameters, value: number) {
    if (!frame) return;
    await send({
      type: 'configure',
      parameters: { ...frame.experiment.parameters, [key]: value },
    });
  }
  async function perturb(x = 0.5, y = 0.5) {
    await send({ type: 'perturb', x, y, strength, radius: brushRadius });
    status =
      'Flux disturbance applied. Run or single-step to follow the response.';
  }
  function point(event: PointerEvent) {
    const rect = canvas.getBoundingClientRect();
    return {
      x: Math.max(0, Math.min(1, (event.clientX - rect.left) / rect.width)),
      y: Math.max(0, Math.min(1, (event.clientY - rect.top) / rect.height)),
    };
  }
  function pointerDown(event: PointerEvent) {
    if (!execution || event.button !== 0) return;
    canvas.setPointerCapture(event.pointerId);
    canvas.focus({ preventScroll: true });
    dragging = true;
    cursor = point(event);
    lastPerturb = performance.now();
    void perturb(cursor.x, cursor.y);
  }
  function pointerMove(event: PointerEvent) {
    cursor = point(event);
    if (dragging && performance.now() - lastPerturb > 70) {
      lastPerturb = performance.now();
      void perturb(cursor.x, cursor.y);
    }
  }
  function pointerUp() {
    dragging = false;
  }
  function fieldKey(event: KeyboardEvent) {
    const directions: Record<string, [number, number]> = {
      ArrowLeft: [-0.025, 0],
      ArrowRight: [0.025, 0],
      ArrowUp: [0, -0.025],
      ArrowDown: [0, 0.025],
    };
    if (directions[event.key]) {
      event.preventDefault();
      const [x, y] = directions[event.key];
      cursor = {
        x: Math.max(0.02, Math.min(0.98, (cursor?.x ?? 0.5) + x)),
        y: Math.max(0.02, Math.min(0.98, (cursor?.y ?? 0.5) + y)),
      };
    } else if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      void perturb(cursor?.x ?? 0.5, cursor?.y ?? 0.5);
    }
  }
  async function save() {
    if (!execution) return;
    pause();
    fileBusy = true;
    error = '';
    try {
      await saveLocal(await execution.snapshot());
      status =
        'Checkpoint saved in this browser. Export a copy to keep it elsewhere.';
    } catch (reason) {
      fail(reason);
    } finally {
      fileBusy = false;
    }
  }
  async function load() {
    if (!execution) return;
    pause();
    fileBusy = true;
    error = '';
    try {
      const snapshot = await loadLocal();
      if (!snapshot) {
        status = 'No local checkpoint yet. Save this experiment first.';
        return;
      }
      accept(await execution.restore(snapshot), true);
      status =
        'Saved state restored exactly. The experiment is paused and ready to resume.';
    } catch (reason) {
      fail(reason);
    } finally {
      fileBusy = false;
    }
  }
  async function exportFile() {
    if (!execution) return;
    pause();
    fileBusy = true;
    error = '';
    try {
      exportSnapshot(await execution.snapshot());
      status =
        'Checkpoint exported, including the field, parameters, and intervention history.';
    } catch (reason) {
      fail(reason);
    } finally {
      fileBusy = false;
    }
  }
  async function importFile(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    if (!file || !execution) return;
    pause();
    fileBusy = true;
    error = '';
    try {
      accept(await execution.restore(await readSnapshotFile(file)), true);
      status =
        'Imported checkpoint restored. Press Run to continue from this state.';
    } catch (reason) {
      fail(reason);
    } finally {
      fileBusy = false;
      input.value = '';
    }
  }
  function number(value: number | undefined) {
    if (value === undefined) return '—';
    if (Math.abs(value) >= 10000) return value.toExponential(1);
    return value.toLocaleString('en-US', { maximumFractionDigits: 2 });
  }
  onMount(() => {
    renderer = createFieldRenderer(canvas);
    const resize = new ResizeObserver(() => {
      if (frame && renderer)
        renderer.draw(frame, {
          contours,
          candidates,
          cursor,
          cursorRadius: brushRadius,
        });
    });
    resize.observe(canvas);
    void (async () => {
      try {
        execution = await createExecution();
        if (destroyed) {
          execution.dispose();
          return;
        }
        catalog = execution.catalog;
        accept(await execution.command({ type: 'observe' }));
        loading = false;
        status =
          'Press Run, or drag through the central sheet. Every disturbance becomes part of your experiment.';
      } catch (reason) {
        loading = false;
        fail(reason);
      }
    })();
    return () => {
      destroyed = true;
      pause();
      resize.disconnect();
      execution?.dispose();
    };
  });
</script>

<svelte:head
  ><title>PlasmaLab · The reconnection lab</title><meta
    name="description"
    content="A small, interactive plasma-inspired laboratory. Disturb a field, follow its response, and save your experiment. Explicitly a toy model, powered by Rust."
  /></svelte:head
>

<div class="app-shell">
  <header class="app-header">
    <a class="wordmark" href="#laboratory" aria-label="PlasmaLab laboratory"
      ><span class="brand-symbol" aria-hidden="true"><i></i><i></i><i></i></span
      >Plasma<span>Lab</span></a
    >
    <div class="header-context">
      <span class="header-divider"></span>A browser-sized laboratory
    </div>
    <nav aria-label="Project">
      <span class="rung-tag">RUNG 01 <span>/</span> TOYBOX</span><a
        href={`${repository}/blob/main/docs/ladder.md`}
        target="_blank"
        rel="noreferrer"
        >The bigger picture <Icon name="external" size={13} /></a
      >
    </nav>
  </header>
  <main id="laboratory">
    <section class="lab-heading" aria-labelledby="lab-title">
      <div>
        <p class="eyebrow">
          <span class="tiny-line"></span>EXPERIMENT 001 / FLUX & INSTABILITY
        </p>
        <h1 id="lab-title">The reconnection lab<span>.</span></h1>
        <p class="lab-subtitle">Disturb the field. Follow what emerges.</p>
      </div>
      <button class="model-button" on:click={() => aboutDialog.showModal()}
        ><span class="toy-tag">TOY MODEL</span><span>What am I looking at?</span
        ><Icon name="info" size={16} /></button
      >
    </section>
    <div class="laboratory-grid">
      <aside class="control-panel" aria-label="Experiment controls">
        <section class="control-section presets-section">
          <div class="section-label">
            <h2>Starting conditions</h2>
            <span>01 — 04</span>
          </div>
          <div class="preset-grid">
            {#each catalog?.presets ?? [] as item}<button
                class:chosen={selectedPreset === item.id}
                class="preset"
                aria-label={item.name}
                aria-pressed={selectedPreset === item.id}
                on:click={() => choosePreset(item.id)}
                disabled={loading}
                ><span class="preset-top"
                  ><span>{presetMeta[item.id]?.number ?? '·'}</span
                  >{#if selectedPreset === item.id}<span class="selected-dot"
                    ></span>{/if}</span
                ><span
                  class="preset-glyph {presetMeta[item.id]?.shape ?? ''}"
                  aria-hidden="true"><i></i><i></i><i></i></span
                ><strong>{item.name}</strong><span class="preset-hint"
                  >{presetMeta[item.id]?.short ?? ''}</span
                ></button
              >{/each}
            {#if !catalog}<div class="loading-presets">
                Preparing conditions…
              </div>{/if}
          </div>
          <p class="preset-description">
            {preset?.description ??
              'Four different ways to explore one simple set of rules.'}
          </p>
        </section>
        <section class="control-section parameter-section">
          <div class="section-label">
            <h2>Shape the response</h2>
            <Icon name="layers" size={14} />
          </div>
          {#each catalog?.parameters ?? [] as parameter}<div class="parameter">
              <label for={`parameter-${parameter.key}`}
                ><span>{parameter.label}</span><output
                  >{frame
                    ? number(
                        frame.experiment.parameters[
                          parameter.key as keyof Parameters
                        ],
                      )
                    : '—'}</output
                ></label
              ><input
                id={`parameter-${parameter.key}`}
                aria-label={parameter.label}
                type="range"
                min={parameter.min}
                max={parameter.max}
                step={parameter.step}
                value={frame?.experiment.parameters[
                  parameter.key as keyof Parameters
                ] ?? parameter.min}
                on:change={(event) =>
                  configure(
                    parameter.key as keyof Parameters,
                    Number(event.currentTarget.value),
                  )}
                disabled={loading}
                aria-describedby={`description-${parameter.key}`}
              />
              <p id={`description-${parameter.key}`}>{parameter.description}</p>
            </div>{/each}
        </section>
        <section class="control-section perturb-section">
          <div class="section-label">
            <h2>Make a disturbance</h2>
            <Icon name="cursor" size={14} />
          </div>
          <p>Click or drag through the field to add a local flux bump.</p>
          <div class="brush-controls">
            <label for="brush-strength"
              >Strength <output>{strength.toFixed(1)}</output></label
            ><input
              id="brush-strength"
              type="range"
              min="-1.5"
              max="1.5"
              step="0.1"
              bind:value={strength}
            />
            <div class="range-ends">
              <span>Negative</span><span>Positive</span>
            </div>
          </div>
          <button
            class="disturb-button"
            disabled={loading}
            on:click={() => perturb()}
            ><Icon name="target" size={15} />Disturb the center <Icon
              name="arrow"
              size={14}
            /></button
          >
          <p class="keyboard-hint">
            Or focus the field: arrows move, Enter disturbs.
          </p>
        </section>
      </aside>
      <div class="experiment-column">
        <section class="field-panel" aria-label="Interactive flux field">
          <div class="field-toolbar">
            <div class="field-heading">
              <span class="live-dot" class:running></span>
              <h2>
                {loading
                  ? 'INITIALIZING'
                  : running
                    ? 'EXPERIMENT RUNNING'
                    : 'EXPERIMENT PAUSED'}
              </h2>
            </div>
            <div class="field-display-controls">
              <button
                class:active={contours}
                on:click={() => (contours = !contours)}
                aria-pressed={contours}
                ><Icon name="layers" size={13} /><span>Contours</span></button
              ><button
                class:active={candidates}
                on:click={() => (candidates = !candidates)}
                aria-pressed={candidates}
                ><Icon name="target" size={13} /><span>Candidates</span></button
              >
            </div>
          </div>
          <div class="canvas-wrap">
            <canvas
              bind:this={canvas}
              data-testid="field-canvas"
              tabindex="0"
              role="button"
              aria-label="Interactive toy flux field. Click or drag to disturb it. Arrow keys move the disturbance cursor; Enter applies a disturbance."
              on:pointerdown={pointerDown}
              on:pointermove={pointerMove}
              on:pointerup={pointerUp}
              on:pointercancel={pointerUp}
              on:pointerleave={() => {
                if (!dragging) cursor = null;
              }}
              on:keydown={fieldKey}
              on:blur={() => {
                cursor = null;
              }}
            ></canvas>
            <div class="field-coordinate top-left">
              <span class="coordinate-cross">+</span>ψ / FLUX PROJECTION
            </div>
            <div class="field-coordinate top-right">
              {frame?.width ?? '—'} × {frame?.height ?? '—'} <span>CELLS</span>
            </div>
            {#if loading}<div class="canvas-empty">
                <span class="loader"></span><span>Opening the laboratory</span>
              </div>{/if}{#if !loading && !frame}<div class="canvas-empty">
                <Icon name="info" size={26} /><span
                  >The experiment could not start.</span
                ><span class="small">See the error below for details.</span>
              </div>{/if}
            <div class="field-coordinate bottom-left">
              <span class="field-key cyan"></span>+Bₓ
              <span class="field-key amber"></span>−Bₓ
              <span class="field-key rose"></span>RELAXATION
            </div>
            <div class="field-coordinate bottom-right">
              {cursor
                ? `x ${cursor.x.toFixed(2)} · y ${cursor.y.toFixed(2)}`
                : 'CLICK + DRAG TO DISTURB'}
            </div>
          </div>
          <div class="transport-bar">
            <div class="transport-buttons">
              <button
                class="run-button"
                disabled={loading || !execution}
                on:click={toggleRun}
                ><Icon name={running ? 'pause' : 'play'} size={16} />{running
                  ? 'Pause'
                  : 'Run'}</button
              ><button
                class="transport-button"
                on:click={step}
                disabled={loading || !execution}
                title="Advance one fixed model step"
                ><Icon name="step" size={15} />Step</button
              ><button
                class="transport-button"
                on:click={reset}
                disabled={loading || !execution}
                title="Reset to initial conditions"
                ><Icon name="reset" size={15} />Reset</button
              >
            </div>
            <div class="clock">
              <span>MODEL TIME</span><strong
                >{frame ? frame.time.toFixed(2) : '0.00'}<span> τ</span></strong
              ><span class="step-label"
                >STEP <span data-testid="step-count">{frame?.step ?? 0}</span
                ></span
              >
            </div>
          </div>
        </section>
        <section class="diagnostics" aria-label="Live model observations">
          <article class="metric energy-metric">
            <div class="metric-label">
              <span class="metric-dot cyan"></span>Stress energy
              <span class="unit">TOY UNITS</span>
            </div>
            <div class="metric-value">
              <span data-testid="stress-value"
                >{frame?.observations.stress_energy.toPrecision(3) ?? '—'}</span
              ><span class="metric-symbol">E</span>
            </div>
            <Sparkline
              values={energyHistory}
              label="Stress energy over recent observed steps; independent vertical scale"
            />
            <p>A field-gradient measure</p>
          </article>
          <article class="metric">
            <div class="metric-label">
              <span class="metric-dot rose"></span>Threshold events
              <span class="unit">TOTAL</span>
            </div>
            <div class="metric-value">
              <span data-testid="events-value"
                >{frame?.observations.total_events.toLocaleString() ??
                  '—'}</span
              ><span class="metric-symbol"><Icon name="spark" size={19} /></span
              >
            </div>
            <Sparkline
              values={eventHistory}
              color="rose"
              label="Cell threshold events in recent observed steps; independent vertical scale"
            />
            <p>
              {frame?.observations.events_this_step ?? 0} cell events in the latest
              step
            </p>
          </article>
          <article class="metric candidate-metric">
            <div class="metric-label">
              <span class="metric-dot amber"></span>Flux candidates
              <span class="unit">OBSERVED</span>
            </div>
            <div class="metric-value">
              <span data-testid="candidates-value"
                >{frame?.observations.candidate_count ?? '—'}</span
              ><span class="metric-symbol"
                ><Icon name="target" size={21} /></span
              >
            </div>
            <div class="candidate-detail">
              <span>Largest region</span><strong
                >{frame?.observations.largest_candidate ?? 0}<small>
                  cells</small
                ></strong
              >
            </div>
            <p>Snapshot labels · no continuity tracking</p>
          </article>
        </section>
        <section class="notebook-strip" aria-label="Experiment checkpoint">
          <div class="notebook-text">
            <span class="notebook-icon"><Icon name="save" size={19} /></span>
            <div>
              <h2>Keep your experiment</h2>
              <p>Save this exact field. Come back with a new question.</p>
            </div>
          </div>
          <div class="checkpoint-actions">
            <button disabled={loading || fileBusy || !execution} on:click={save}
              ><Icon name="save" size={14} />Save</button
            ><button
              disabled={loading || fileBusy || !execution}
              on:click={load}><Icon name="load" size={15} />Load</button
            ><span class="action-divider"></span><button
              class="icon-button"
              disabled={loading || fileBusy || !execution}
              on:click={exportFile}
              title="Export checkpoint as JSON"
              aria-label="Export checkpoint as JSON"
              ><Icon name="download" size={16} /></button
            ><button
              class="icon-button"
              disabled={loading || fileBusy || !execution}
              on:click={() => importInput.click()}
              title="Import checkpoint from JSON"
              aria-label="Import checkpoint from JSON"
              ><Icon name="upload" size={16} /></button
            ><input
              class="visually-hidden"
              tabindex="-1"
              aria-label="Choose a checkpoint file"
              data-testid="import-checkpoint"
              type="file"
              accept="application/json,.json"
              bind:this={importInput}
              on:change={importFile}
            />
          </div>
        </section>
        <div
          class="status-line"
          class:error={!!error}
          data-testid="status"
          role={error ? 'alert' : 'status'}
          aria-live="polite"
        >
          <Icon name={error ? 'info' : 'check'} size={13} /><span
            >{error || status}</span
          >{#if error}<button
              on:click={() => (error = '')}
              aria-label="Dismiss error"><Icon name="close" size={13} /></button
            >{/if}
        </div>
      </div>
    </div>
    <footer class="lab-footer">
      <span
        ><span class="engine-dot"></span>Rust / WASM
        <span class="footer-divider">·</span>Fixed model steps
        <span class="footer-divider">·</span>Runs in your browser</span
      ><span
        >Observe. Question. Disturb. Repeat.<a
          href={repository}
          target="_blank"
          rel="noreferrer"
          aria-label="Open PlasmaLab repository"
          ><Icon name="external" size={13} /></a
        ></span
      >
    </footer>
  </main>
</div>

<dialog
  class="model-dialog"
  bind:this={aboutDialog}
  aria-labelledby="about-title"
  on:click={(event) => {
    if (event.target === aboutDialog) aboutDialog.close();
  }}
  on:keydown={(event) => {
    if (event.key === 'Escape') aboutDialog.close();
  }}
>
  <div class="model-dialog-inner">
    <div class="dialog-top">
      <span class="eyebrow">THE RULES OF THIS LITTLE UNIVERSE</span><button
        class="icon-button"
        on:click={() => aboutDialog.close()}
        aria-label="Close model explanation"
        ><Icon name="close" size={20} /></button
      >
    </div>
    <h2 id="about-title">A toy worth asking<br />questions of.</h2>
    <p class="dialog-lede">
      This is a 2D, plasma-inspired cellular field model. It makes local stress,
      threshold relaxation, and evolving field patterns visible. It is not
      quantitative MHD or PIC.
    </p>
    <div class="model-facts">
      <div><span>SCIENTIFIC OWNER</span><strong>Rust model</strong></div>
      <div><span>EXECUTION</span><strong>CPU / WebAssembly</strong></div>
      <div>
        <span>MODEL VERSION</span><strong
          >{catalog?.model_version ?? '—'}</strong
        >
      </div>
    </div>
    <section>
      <h3>Read the field</h3>
      <p>
        The scalar flux ψ is the evolving grid field. Contours connect equal ψ;
        the direction proxy is B = (∂yψ, −∂xψ). Cyan and amber show opposite
        signs of Bₓ. The current proxy is −8∇²ψ in grid units. Rose highlights
        cells recently relaxed by the threshold rule. Display contrast adapts to
        the current field; colors are not absolute measurements.
      </p>
    </section>
    <section>
      <h3>Simple local rules</h3>
      <ol>
        {#each catalog?.rules ?? [] as rule}<li>{rule}</li>{/each}
      </ol>
    </section>
    <section>
      <h3>Observations, not new entities</h3>
      <p>
        Stress energy is a field-gradient diagnostic in toy units. Events count
        cells that trigger the threshold rule, not distinct physical
        reconnection events. Candidate rings mark connected flux anomalies
        relative to the initial sheet. They are not proof of closed magnetic
        islands or validated SELFS. Labels are reassigned on each observation;
        they do not establish a structure’s identity, lifetime, splitting, or
        merging.
      </p>
    </section>
    <section>
      <h3>Know the limits</h3>
      <ul>
        {#each catalog?.assumptions ?? [] as assumption}<li>
            {assumption}
          </li>{/each}
      </ul>
      <p>
        Stronger physical fidelity would require a specified plasma model,
        justified numerics, and reference and convergence evidence. An evocative
        picture does not provide those checks.
      </p>
    </section>
    <section>
      <h3>Your experiment, continued</h3>
      <p>
        Save keeps a versioned checkpoint in this browser: configuration,
        interventions, and the complete model state. Load restores that state
        and pauses. Export downloads a portable copy; local browser storage
        alone is not a backup. Resuming one checkpoint is distinct from
        guaranteeing identical trajectories across every browser or future model
        version.
      </p>
      <p>
        The small charts show up to 100 recent observations from this session,
        with independent vertical scales. They restart after a reset or load.
        The model advances in fixed steps of {catalog?.dt ?? '—'} toy time units;
        slower rendering slows progress, not scientific fidelity.
      </p>
    </section>
    <div class="dialog-footer">
      <span>RUNG 1 · MAKE SOMETHING HAPPEN</span><button
        class="run-button"
        on:click={() => aboutDialog.close()}
        >Back to the field <Icon name="arrow" size={15} /></button
      >
    </div>
  </div>
</dialog>
