<script lang="ts">
  interface Props {
    contenido: string;
    origen: string;
    fecha: string;
  }
  let { contenido, origen, fecha }: Props = $props();
  let abierto = $state(false);
</script>

<div class="wrap">
  <button
    type="button"
    class="cabecera"
    onclick={() => (abierto = !abierto)}
    aria-expanded={abierto}
  >
    <span class="flecha" class:rot={abierto}>▸</span>
    ORIGINAL INMUTABLE · {origen.toUpperCase()} · {fecha}
  </button>
  {#if abierto}
    <pre class="cuerpo">{contenido}</pre>
    <div class="nota">Lo que entró al sistema, nunca se toca. Tu edición vive arriba.</div>
  {/if}
</div>

<style>
  .wrap { display: flex; flex-direction: column; gap: 6px; }
  .cabecera {
    background: transparent;
    border: none;
    padding: 0;
    display: flex;
    align-items: center;
    gap: 6px;
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--apagado);
    letter-spacing: 0.1em;
    cursor: pointer;
    text-align: left;
  }
  .flecha { display: inline-block; transition: transform 0.15s ease; }
  .flecha.rot { transform: rotate(90deg); }
  .cuerpo {
    font-family: var(--font-mono);
    font-size: 11px;
    line-height: 1.7;
    color: var(--texto-2);
    background: var(--superficie-2);
    border: 1px solid var(--borde);
    border-radius: var(--radio-m);
    padding: 12px 14px;
    margin: 0;
    white-space: pre-wrap;
  }
  .nota { font-size: 11px; color: var(--apagado); }
</style>
