<script lang="ts">
  import Sparkles from '@lucide/svelte/icons/sparkles';

  interface Props {
    texto: string;
    onaceptar?: () => void;
    onignorar?: () => void;
  }
  let { texto, onaceptar, onignorar }: Props = $props();
</script>

<div class="banner">
  <span class="icono"><Sparkles size={15} strokeWidth={1.5} /></span>
  <span class="msg">{@html texto}</span>
  {#if onaceptar}
    <button class="aceptar" onclick={onaceptar}>Aceptar</button>
  {/if}
  {#if onignorar}
    <button class="ignorar" onclick={onignorar}>Ignorar</button>
  {/if}
</div>

<style>
  .banner {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 9px 12px;
    background: var(--indigo-suave);
    border: 1px solid var(--indigo-borde);
    border-radius: var(--radio-l);
  }
  .icono { display: inline-flex; color: var(--indigo); flex-shrink: 0; }
  .msg { flex: 1; font-size: 12px; color: var(--indigo-hover); }
  .msg :global(strong) { color: var(--indigo); }
  button {
    font-family: var(--font-ui);
    font-size: 11px;
    border-radius: var(--radio-m);
    padding: 4px 10px;
    border: 1px solid var(--indigo-borde);
    background: transparent;
    color: var(--texto-2);
    cursor: pointer;
    font-weight: 500;
  }
  button.aceptar {
    background: var(--indigo);
    color: #FFFFFF;
    border-color: var(--indigo);
    font-weight: 600;
  }
  button.aceptar:hover { background: var(--indigo-hover); }
</style>
