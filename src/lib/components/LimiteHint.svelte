<script lang="ts">
  import { cercaDelLimite } from '$lib/limites';

  let { valor, max }: { valor: string; max: number } = $props();

  const visible = $derived(cercaDelLimite(valor, max));
  const tope = $derived(valor.length >= max);
</script>

{#if visible}
  <span class="lim" class:tope>{valor.length} / {max}</span>
{/if}

<style>
  .lim {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--apagado);
    white-space: nowrap;
    flex-shrink: 0;
  }
  .lim.tope { color: var(--alerta); }
</style>
