<script lang="ts">
  export let values: number[] = [];
  export let color = 'cyan';
  export let label = 'Recent observations';
  $: minimum = Math.min(...values, 0);
  $: maximum = Math.max(...values, 0.000001);
  $: span = maximum - minimum || 1;
  $: points = values
    .map(
      (value, index) =>
        `${(index / Math.max(1, values.length - 1)) * 260},${41 - ((value - minimum) / span) * 33}`,
    )
    .join(' ');
</script>

<svg
  class="sparkline {color}"
  viewBox="0 0 260 48"
  preserveAspectRatio="none"
  role="img"
  aria-label={label}
>
  <path class="sparkline-baseline" d="M0 41H260" />
  {#if values.length > 1}<polyline
      {points}
      fill="none"
      stroke="currentColor"
      stroke-width="1.6"
      vector-effect="non-scaling-stroke"
    />{/if}
</svg>
