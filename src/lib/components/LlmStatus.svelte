<script lang="ts">
  import Chip from './Chip.svelte';

  interface Props {
    activo: boolean | null;
    aviso?: string | null;
    /** Frase completa (caja de captura). Si no, chip corto. */
    frase?: boolean;
  }

  let { activo, aviso = null, frase = false }: Props = $props();

  const textoCaida = $derived(
    aviso?.trim() || 'Sin conexión con la IA. La nota se guardará tal cual.'
  );
</script>

{#if activo === null}
  {#if frase}
    <span class="estado comprobando">comprobando si la IA responde…</span>
  {:else}
    <Chip variant="neutral" mono>comprobando IA…</Chip>
  {/if}
{:else if activo}
  {#if frase}
    <span class="estado ok">IA lista · clasifica al guardar</span>
  {:else}
    <Chip variant="ok" mono title="El modelo responderá al guardar">IA lista</Chip>
  {/if}
{:else if frase}
  <span class="estado alerta" title={textoCaida}>{textoCaida}</span>
{:else}
  <Chip variant="alerta" mono title={textoCaida}>sin IA</Chip>
{/if}

<style>
  .estado {
    font-family: var(--font-mono);
    font-size: 10px;
    line-height: 1.35;
    max-width: 280px;
  }
  .comprobando { color: var(--apagado); }
  .ok { color: var(--ok-texto); }
  .alerta { color: var(--alerta-texto); }
</style>
