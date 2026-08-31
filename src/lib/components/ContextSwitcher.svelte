<script lang="ts">
  import Selector from './Selector.svelte';
  import { app } from '$lib/stores/app.svelte';
  import { fijarContexto } from '$lib/tauri';
  import type { Contexto } from '$lib/types';

  interface Props {
    contexto: Contexto | null;
    align?: 'left' | 'right';
    compact?: boolean;
    onchange?: (ctx: Contexto) => void;
  }
  let { contexto, align = 'left', compact = false, onchange }: Props = $props();

  let abierto = $state(false);
  let raizEl: HTMLDivElement | null = $state(null);

  const opciones = $derived.by(() =>
    app.proyectos.map((p) => {
      const esp = app.espacios.find((e) => e.espacioId === p.espacioId);
      return {
        value: `${p.espacioId}::${p.proyectoId}`,
        label: `${esp?.nombre ?? '?'} / ${p.nombre}`,
        color: esp?.color
      };
    })
  );

  const valorActual = $derived(
    contexto ? `${contexto.espacioId}::${contexto.proyectoId}` : null
  );

  function toggle() {
    if (abierto) {
      abierto = false;
      return;
    }
    void abrir();
  }

  async function abrir() {
    try {
      await app.hidratarDestinos();
    } catch (err) {
      console.warn('no se pudieron cargar destinos', err);
    }
    abierto = true;
  }

  function cerrar() {
    abierto = false;
  }

  function onDocClick(e: MouseEvent) {
    if (!raizEl) return;
    if (!raizEl.contains(e.target as Node)) cerrar();
  }

  $effect(() => {
    if (abierto) {
      document.addEventListener('mousedown', onDocClick);
      return () => document.removeEventListener('mousedown', onDocClick);
    }
  });

  async function elegir(v: string) {
    const [espacioId, proyectoId] = v.split('::');
    const esp = app.espacios.find((e) => e.espacioId === espacioId);
    const proy = app.proyectos.find((p) => p.proyectoId === proyectoId);
    if (!esp || !proy) return;
    const nuevo: Contexto = {
      espacioId,
      proyectoId,
      espacioNombre: esp.nombre,
      espacioColor: esp.color,
      proyectoNombre: proy.nombre,
      origen: 'usuario',
      etiqueta: `${esp.nombre} / ${proy.nombre}`
    };
    app.contexto = nuevo;
    abierto = false;
    onchange?.(nuevo);
    try {
      await fijarContexto(espacioId, proyectoId, 'usuario');
    } catch (err) {
      console.error('no se pudo fijar contexto', err);
    }
  }
</script>

<div class="raiz" bind:this={raizEl}>
  <button
    type="button"
    class="chip"
    class:compact
    onclick={toggle}
    title="clic para cambiar"
  >
    <span class="dot" style:background={contexto?.espacioColor ?? 'var(--apagado)'}></span>
    <span class="lbl">{contexto?.etiqueta ?? 'sin contexto'}</span>
    <span class="chev">▾</span>
  </button>
  {#if abierto}
    <div class="pop" class:right={align === 'right'}>
      <Selector
        options={opciones}
        value={valorActual}
        filterPlaceholder="Filtrar espacio o proyecto…"
        onselect={elegir}
        onclose={cerrar}
      />
    </div>
  {/if}
</div>

<style>
  .raiz { position: relative; display: inline-flex; }
  .chip {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    background: var(--superficie-2);
    border: 1px solid var(--borde);
    border-radius: var(--radio-s);
    padding: 3px 9px;
    cursor: pointer;
    font-family: var(--font-ui);
    color: var(--tinta);
  }
  .chip.compact { font-size: 11px; padding: 2px 8px; }
  .chip:hover { border-color: var(--indigo); }
  .dot { width: 8px; height: 8px; border-radius: 2px; flex-shrink: 0; }
  .lbl { white-space: nowrap; }
  .chev { color: var(--apagado); }
  .pop {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    z-index: 30;
  }
  .pop.right { left: auto; right: 0; }
</style>
