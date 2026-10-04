<script lang="ts">
  export let name = 'play';
  export let size = 16;
  const paths: Record<string, string> = {
    play: 'm9 5 11 7-11 7V5Z',
    pause: 'M8 5v14M16 5v14',
    step: 'm6 5 10 7-10 7V5ZM20 5v14',
    reset: 'M3 10a9 9 0 1 1 2 8M3 4v6h6',
    save: 'M4 3h13l4 4v14H3V3h1ZM7 3v6h10V3M7 21v-8h10v8',
    load: 'M3 7h7l2 3h10l-3 10H2V5h7l2 2M3 10h19',
    download: 'M12 3v12m-5-5 5 5 5-5M4 16v5h16v-5',
    upload: 'M12 16V4m-5 5 5-5 5 5M4 16v5h16v-5',
    info: 'M12 11v6M12 7h.01M22 12a10 10 0 1 1-20 0 10 10 0 0 1 20 0Z',
    arrow: 'M5 12h14m-5-5 5 5-5 5',
    close: 'm6 6 12 12M6 18 18 6',
    spark: 'm13 2-9 12h7l-1 8 10-12h-8l1-8Z',
    cursor: 'm4 3 15 9-7 2-3 7-5-18Z',
    target:
      'M12 2v4m0 12v4M2 12h4m12 0h4M19 12a7 7 0 1 1-14 0 7 7 0 0 1 14 0ZM14 12a2 2 0 1 1-4 0 2 2 0 0 1 4 0Z',
    layers: 'm12 3 10 5-10 5L2 8l10-5ZM2 12l10 5 10-5M2 16l10 5 10-5',
    check: 'm5 12 4 4L19 6',
    external: 'M14 3h7v7m0-7L10 14M9 3H3v18h18v-6',
  };
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.6"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"><path d={paths[name] ?? paths.info} /></svg
>
