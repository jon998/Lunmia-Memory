<script lang="ts">
  import { goto } from '$app/navigation';
  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import type { Entrada } from '$lib/types';
  import Chip from './Chip.svelte';
  import SpaceDot from './SpaceDot.svelte';
  import { estiloTipo } from '$lib/tipos';
  import { previewLista } from '$lib/contenido';

  interface Props {
    entrada: Entrada;
    href?: string;
    onclick?: () => void;
    oneliminar?: () => void;
    mostrarFlecha?: boolean;
    hace?: string;
  }
  let { entrada, href, onclick, oneliminar, mostrarFlecha = false, hace }: Props = $props();

  const tipoEstilo = estiloTipo;

  function metaText(): string {
    const partes = [entrada.espacioNombre, entrada.proyectoNombre];
    if (hace) partes.push(hace);
    partes.push(entrada.origen === 'captura_pantalla' ? 'captura' : entrada.origen);
    return partes.join(' · ');
  }

  function handleClick() {
    if (onclick) onclick();
    else if (href) void goto(href);
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handleClick(); }
  }
</script>

<div
  class="row"
  role={onclick || href ? 'button' : undefined}
  tabindex={onclick || href ? 0 : undefined}
  title={onclick || href ? 'Abrir detalle' : undefined}
  onclick={handleClick}
  onkeydown={handleKey}
>
  <SpaceDot color={entrada.espacioColor} />
  <div class="body">
    <div class="texto">{previewLista(entrada.contenido)}</div>
    <div class="meta">{metaText()}</div>
  </div>
  {#if entrada.esProvisional}
    <Chip variant="alerta">provisional</Chip>
  {/if}
  {#if entrada.confianzaCapa != null && entrada.confianzaCapa !== 3}
    <Chip variant="alerta" title="Se guardó sin el modelo. El texto quedó como lo escribiste.">sin IA</Chip>
  {/if}
  {#if entrada.tipoNombre}
    <Chip variant={tipoEstilo(entrada.tipoNombre)}>{entrada.tipoNombre}</Chip>
  {:else}
    <Chip variant="neutral">clasificando…</Chip>
  {/if}
  {#if oneliminar}
    <button
      type="button"
      class="borrar"
      title="Eliminar todo (nota y calendario)"
      aria-label="Eliminar todo"
      onclick={(e) => { e.stopPropagation(); oneliminar(); }}
    >
      <Trash2 size={14} strokeWidth={1.75} />
    </button>
  {/if}
  {#if mostrarFlecha}
    <span class="flecha" aria-hidden="true"><ArrowRight size={15} strokeWidth={1.5} /></span>
  {/if}
</div>

<style>
  .row {
    display: flex;
    align-items: center;
    gap: 10px;
    border: 1px solid var(--borde);
    border-radius: var(--radio-l);
    padding: 9px 12px;
    background: var(--superficie);
    cursor: pointer;
    transition: border-color 0.12s ease;
  }
  .row:hover { border-color: var(--indigo-borde); background: var(--indigo-suave); }
  .body { flex: 1; min-width: 0; }
  .texto {
    font-size: 13px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .meta {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--apagado);
    margin-top: 2px;
  }
  .flecha {
    display: inline-flex;
    color: var(--apagado);
    flex-shrink: 0;
  }
  .borrar {
    display: inline-flex; align-items: center; justify-content: center;
    width: 28px; height: 28px; flex-shrink: 0;
    border: none; border-radius: var(--radio-s);
    background: transparent; color: var(--apagado);
    cursor: pointer; opacity: 0;
  }
  .row:hover .borrar, .row:focus-within .borrar { opacity: 1; }
  .borrar:hover { color: var(--error); background: var(--error-suave); }
</style>
