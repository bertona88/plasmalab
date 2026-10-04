<script lang="ts">
  const repository = 'https://github.com/bertona88/plasmalab';
  const rungs = [
    {
      number: '01',
      title: 'Toybox',
      verb: 'Make something happen.',
      detail: 'Choose, disturb, pause, inspect, save, and return.',
      gate: 'A complete learning loop with visible assumptions.',
    },
    {
      number: '02',
      title: 'Laboratory',
      verb: 'Investigate what happened.',
      detail: 'Measure a reference case. Track structures. Test a prediction.',
      gate: 'Named observables, measured errors, and honest limits.',
    },
    {
      number: '03',
      title: 'Adaptive SELFS',
      verb: 'Change the description.',
      detail: 'Let a tested representation earn limited responsibility.',
      gate: 'One useful transition, full cost, and a tested recovery path.',
    },
  ];
</script>

<svelte:head><title>PlasmaLab · Foundation</title></svelte:head>

<div class="shell">
  <header>
    <a class="wordmark" href="#main" aria-label="PlasmaLab home"
      ><span class="mark" aria-hidden="true">◌</span> PlasmaLab</a
    >
    <nav aria-label="Project">
      <a href={`${repository}/blob/main/docs/ladder.md`}>The ladder ↗</a><a
        href={repository}>Repository ↗</a
      >
    </nav>
  </header>

  <main id="main">
    <section class="intro" aria-labelledby="intro-title">
      <p class="eyebrow">Browser plasma laboratory / Foundation</p>
      <h1 id="intro-title">
        Play with the conditions.<br /><span>Understand the response.</span>
      </h1>
      <p class="lede">
        A place to build intuition about plasmas, follow emerging structures,
        and discover when a different description becomes useful.
      </p>
      <p class="status">
        <span aria-hidden="true"></span> Repository scaffold · simulations are not
        implemented yet
      </p>
    </section>

    <section aria-labelledby="ladder-title">
      <div class="section-heading">
        <h2 id="ladder-title">One destination. Three useful steps.</h2>
        <span>Keep what each step teaches.</span>
      </div>
      <div class="rungs">
        {#each rungs as rung}
          <article class="rung">
            <div class="rung-top">
              <span class="number">{rung.number}</span><span class="tag"
                >Planned</span
              >
            </div>
            <h3>{rung.title}</h3>
            <p class="verb">{rung.verb}</p>
            <p>{rung.detail}</p>
            <details>
              <summary>What earns the next step?</summary>
              <p>{rung.gate}</p>
            </details>
          </article>
        {/each}
      </div>
    </section>

    <section class="next" aria-labelledby="next-title">
      <div>
        <p class="eyebrow">First proposed experiment</p>
        <h2 id="next-title">Displace. Release. Observe.</h2>
        <p>
          Start with a small charge-separation oscillator. Change its initial
          displacement and watch the restoring response, with the model’s
          assumptions always in view.
        </p>
        <a
          class="button"
          href={`${repository}/blob/main/docs/experiments/charge-separation.md`}
          >Read the experiment brief <span aria-hidden="true">↗</span></a
        >
      </div>
      <div class="notebook">
        <p class="eyebrow">The first loop to build</p>
        <ol>
          <li>Set the conditions</li>
          <li>Disturb and inspect the response</li>
          <li>Compare against a reference</li>
          <li>Save, export, and reopen</li>
        </ol>
        <p class="note">
          An experiment plan, not a running simulation. Structure tracking and
          adaptive representations come later.
        </p>
      </div>
    </section>
  </main>

  <footer>
    <span>Aim at 3. Ship through 1 and 2.</span><a
      href={`${repository}/blob/main/docs/SELFS_WOFI_abstract.md`}
      >The SELFS hypothesis ↗</a
    >
  </footer>
</div>
