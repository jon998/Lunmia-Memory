<script lang="ts">
  interface Props { children?: import('svelte').Snippet; label?: string }
  let { children, label }: Props = $props();
</script>

<span class="wrap">
  <span class="key">{@render children?.()}</span>
  {#if label}<span class="lbl">{label}</span>{/if}
</span>

<style>
  .wrap { display: inline-flex; align-items: center; gap: 5px; white-space: nowrap; }
  .key {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--tinta);
    border: 1px solid var(--borde-fuerte);
    border-radius: var(--radio-s);
    padding: 2px 7px;
    background: var(--superficie);
    box-shadow: var(--sombra-tecla);
    line-height: 1.4;
  }
  .lbl { font-size: 11px; color: var(--texto-2); }
</style>
