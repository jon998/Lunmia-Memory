<script lang="ts">
  import House from '@lucide/svelte/icons/house';
  import Calendar from '@lucide/svelte/icons/calendar';
  import Pin from '@lucide/svelte/icons/pin';
  import Inbox from '@lucide/svelte/icons/inbox';
  import Plus from '@lucide/svelte/icons/plus';
  import type { Espacio, Proyecto } from '$lib/types';
  import { app } from '$lib/stores/app.svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import ContextMenu from './ContextMenu.svelte';
  import { LIMITES } from '$lib/limites';

  interface Filtro { espacioId: string | null; proyectoId: string | null; }
  interface Props {
    filtro: Filtro;
    onfiltro: (f: Filtro) => void;
    vista: 'lista' | 'calendario';
    onvista: (v: 'lista' | 'calendario') => void;
  }
  let { filtro, onfiltro, vista, onvista }: Props = $props();

  type Menu =
    | { tipo: 'espacio'; id: string; x: number; y: number }
    | { tipo: 'proyecto'; id: string; espacioId: string; x: number; y: number }
    | null;
  let menu = $state<Menu>(null);
  let renombrando = $state<{ tipo: 'espacio' | 'proyecto'; id: string; valor: string } | null>(null);
  let creando = $state<{ espacioId: string; valor: string } | null>(null);

  function cuentaEspacio(e: Espacio) {
    return app.entradas.filter((x) => x.espacioId === e.espacioId).length;
  }
  function cuentaProyecto(p: Proyecto) {
    return app.entradas.filter((x) => x.proyectoId === p.proyectoId).length;
  }

  const fijados = $derived.by(() => {
    const items: Array<{ tipo: 'espacio' | 'proyecto'; id: string; nombre: string; espacioNombre?: string; cuenta: number }> = [];
    for (const e of app.espacios) if (e.fijado) items.push({ tipo: 'espacio', id: e.espacioId, nombre: e.nombre, cuenta: cuentaEspacio(e) });
    for (const p of app.proyectos) if (p.fijado) {
      const e = app.espacios.find((x) => x.espacioId === p.espacioId);
      items.push({ tipo: 'proyecto', id: p.proyectoId, nombre: p.nombre, espacioNombre: e?.nombre, cuenta: cuentaProyecto(p) });
    }
    return items;
  });

  function menuEspacio(e: MouseEvent, esp: Espacio) {
    e.preventDefault();
    menu = { tipo: 'espacio', id: esp.espacioId, x: e.clientX, y: e.clientY };
  }
  function menuProyecto(e: MouseEvent, p: Proyecto) {
    e.preventDefault();
    menu = { tipo: 'proyecto', id: p.proyectoId, espacioId: p.espacioId, x: e.clientX, y: e.clientY };
  }

  const itemsMenu = $derived.by(() => {
    if (!menu) return [];
    if (menu.tipo === 'espacio') {
      const esp = app.espacios.find((e) => e.espacioId === menu!.id);
      if (!esp) return [];
      return [
        { label: 'Renombrar', action: () => renombrando = { tipo: 'espacio', id: esp.espacioId, valor: esp.nombre } },
        { label: 'Añadir proyecto', action: () => creando = { espacioId: esp.espacioId, valor: '' } },
        { label: esp.fijado ? 'Desfijar' : 'Fijar', action: () => void app.fijarEspacio(esp.espacioId, !esp.fijado) },
        { separator: true, label: '', action: () => {} },
        { label: 'Eliminar espacio', danger: true, action: () => confirmarEliminarEspacio(esp) }
      ];
    }
    const p = app.proyectos.find((x) => x.proyectoId === menu!.id);
    if (!p) return [];
    return [
      { label: 'Renombrar', action: () => renombrando = { tipo: 'proyecto', id: p.proyectoId, valor: p.nombre } },
      { label: p.fijado ? 'Desfijar' : 'Fijar', action: () => void app.fijarProyecto(p.proyectoId, !p.fijado) },
      { separator: true, label: '', action: () => {} },
      {
        label: p.esPorDefecto ? 'Proyecto por defecto — no se elimina' : 'Eliminar proyecto',
        danger: !p.esPorDefecto,
        disabled: p.esPorDefecto,
        action: () => confirmarEliminarProyecto(p)
      }
    ];
  });

  function confirmarEliminarEspacio(esp: Espacio) {
    const n = app.entradas.filter((x) => x.espacioId === esp.espacioId).length;
    const msg = n
      ? `Eliminar "${esp.nombre}" archivará ${n} entrada${n === 1 ? '' : 's'} con este espacio. ¿Continuar?`
      : `Eliminar "${esp.nombre}"?`;
    if (confirm(msg)) void app.eliminarEspacio(esp.espacioId);
  }
  function confirmarEliminarProyecto(p: Proyecto) {
    if (p.esPorDefecto) return;
    if (confirm(`Eliminar el proyecto "${p.nombre}"?`)) void app.eliminarProyecto(p.proyectoId);
  }

  async function commitRenombrar() {
    if (!renombrando) return;
    const valor = renombrando.valor.trim();
    if (!valor) { renombrando = null; return; }
    if (renombrando.tipo === 'espacio') await app.renombrarEspacio(renombrando.id, valor);
    else await app.renombrarProyecto(renombrando.id, valor);
    renombrando = null;
  }

  async function commitCrear() {
    if (!creando) return;
    const nombre = creando.valor.trim();
    const espacioId = creando.espacioId;
    if (!nombre) { creando = null; return; }
    await app.crearProyectoEn(espacioId, nombre);
    creando = null;
  }
</script>

<aside class="sidebar">
  <div class="marca">
    <img src="/assets/logo.png" alt="Lunmia" width="30" height="30" />
    <div>
      <div class="nombre">Lunmia</div>
      <div class="lema">todo en tu Mac</div>
    </div>
  </div>

  <button
    class="item nav"
    class:sel={!filtro.espacioId && vista === 'lista' && $page.url.pathname === '/'}
    onclick={() => { onfiltro({ espacioId: null, proyectoId: null }); onvista('lista'); goto('/'); }}
  >
    <House size={15} strokeWidth={1.5} />
    Inicio
    <span class="cuenta">{app.entradas.length}</span>
  </button>

  <button
    class="item nav"
    class:sel={!filtro.espacioId && vista === 'calendario'}
    onclick={() => { onfiltro({ espacioId: null, proyectoId: null }); onvista('calendario'); }}
  >
    <Calendar size={15} strokeWidth={1.5} />
    Todos los calendarios
  </button>

  <button
    class="item nav"
    class:sel={$page.url.pathname === '/bandeja'}
    onclick={() => goto('/bandeja')}
  >
    <Inbox size={15} strokeWidth={1.5} />
    Bandeja
    <span class="pill">{app.stats.sinTriage}</span>
  </button>

  {#if fijados.length > 0}
    <div class="titulo">FIJADOS</div>
    {#each fijados as f (f.id)}
      <button
        class="item small"
        onclick={() => onfiltro(
          f.tipo === 'proyecto'
            ? { espacioId: app.proyectos.find((p) => p.proyectoId === f.id)?.espacioId ?? null, proyectoId: f.id }
            : { espacioId: f.id, proyectoId: null }
        )}
      >
        <Pin size={13} strokeWidth={1.5} />
        <span class="lbl">{f.espacioNombre ? `${f.espacioNombre} / ${f.nombre}` : f.nombre}</span>
        <span class="cuenta">{f.cuenta}</span>
      </button>
    {/each}
  {/if}

  <div class="titulo">ESPACIOS</div>
  {#each app.espacios as e (e.espacioId)}
    <div class="fila-esp">
      {#if renombrando?.tipo === 'espacio' && renombrando.id === e.espacioId}
        <span class="dot" style:background={e.color}></span>
        <input
          class="rename"
          bind:value={renombrando.valor}
          maxlength={LIMITES.nombre}
          onblur={commitRenombrar}
          onkeydown={(ev) => { if (ev.key === 'Enter') commitRenombrar(); if (ev.key === 'Escape') renombrando = null; }}
          autofocus
        />
        {:else}
          <div
            class="item esp"
            role="button"
            tabindex="0"
            class:sel={filtro.espacioId === e.espacioId && !filtro.proyectoId}
            onclick={() => onfiltro({ espacioId: e.espacioId, proyectoId: null })}
            ondblclick={() => renombrando = { tipo: 'espacio', id: e.espacioId, valor: e.nombre }}
            oncontextmenu={(ev) => menuEspacio(ev, e)}
            onkeydown={(ev) => ev.key === 'Enter' && onfiltro({ espacioId: e.espacioId, proyectoId: null })}
          >
            <span class="dot" style:background={e.color}></span>
            <span class="lbl">{e.nombre}</span>
            {#if e.fijado}<Pin size={11} strokeWidth={1.5} class="pin-on" />{/if}
            <button
              type="button"
              class="add"
              title="Añadir proyecto"
              onclick={(ev) => { ev.stopPropagation(); creando = { espacioId: e.espacioId, valor: '' }; }}
            >
              <Plus size={12} strokeWidth={1.8} />
            </button>
            <span class="cuenta">{cuentaEspacio(e)}</span>
          </div>
        {/if}
    </div>

    {#each app.proyectosDe(e.espacioId) as p (p.proyectoId)}
      {#if renombrando?.tipo === 'proyecto' && renombrando.id === p.proyectoId}
        <div class="fila-proy">
          <input
            class="rename nested"
            bind:value={renombrando.valor}
            maxlength={LIMITES.nombre}
            onblur={commitRenombrar}
            onkeydown={(ev) => { if (ev.key === 'Enter') commitRenombrar(); if (ev.key === 'Escape') renombrando = null; }}
            autofocus
          />
        </div>
      {:else}
        <button
          class="item nested"
          class:sel={filtro.proyectoId === p.proyectoId}
          onclick={() => onfiltro({ espacioId: e.espacioId, proyectoId: p.proyectoId })}
          ondblclick={() => renombrando = { tipo: 'proyecto', id: p.proyectoId, valor: p.nombre }}
          oncontextmenu={(ev) => menuProyecto(ev, p)}
        >
          <span class="lbl">{p.nombre}</span>
          {#if p.fijado}<Pin size={10} strokeWidth={1.5} class="pin-on" />{/if}
          <span class="cuenta">{cuentaProyecto(p)}</span>
        </button>
      {/if}
    {/each}

    {#if creando?.espacioId === e.espacioId}
      <div class="fila-proy">
        <input
          class="rename nested"
          bind:value={creando.valor}
          maxlength={LIMITES.nombre}
          placeholder="Nuevo proyecto…"
          onblur={commitCrear}
          onkeydown={(ev) => { if (ev.key === 'Enter') commitCrear(); if (ev.key === 'Escape') creando = null; }}
          autofocus
        />
      </div>
    {/if}
  {/each}

  <div class="footer">
    {#if app.contexto}
      <div class="titulo">CONTEXTO ACTIVO</div>
      <div class="ctx"><span class="dot" style:background={app.contexto.espacioColor}></span>{app.contexto.etiqueta}</div>
      <div class="nota">aprendido · L–V 7:00–16:00</div>
    {/if}
  </div>
</aside>

{#if menu}
  <ContextMenu items={itemsMenu} x={menu.x} y={menu.y} onclose={() => menu = null} />
{/if}

<style>
  .sidebar {
    width: 230px;
    background: var(--superficie-2);
    border-right: 1px solid var(--borde);
    padding: 16px 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    overflow-y: auto;
    flex-shrink: 0;
    height: 100%;
    box-sizing: border-box;
  }
  .marca { display: flex; align-items: center; gap: 10px; padding: 2px 8px 12px; }
  .marca img { border-radius: 6px; }
  .nombre { font-family: var(--font-display); font-size: 14px; font-weight: 700; }
  .lema { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); }
  .item {
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radio-m);
    padding: 6px 8px;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-family: var(--font-ui);
    font-weight: 500;
    font-size: 13px;
    color: var(--tinta);
    text-align: left;
    width: 100%;
  }
  .item:hover { background: rgba(255,255,255,0.6); }
  .item.sel { background: var(--superficie); border-color: var(--borde); }
  .item.nested { padding: 4px 8px 4px 26px; font-size: 12px; color: var(--texto-2); font-weight: 400; }
  .item.small { font-size: 12px; padding: 5px 8px; }
  .lbl { flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .cuenta { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); margin-left: 0; }
  .pill { font-family: var(--font-mono); font-size: 10px; background: var(--indigo-suave); color: var(--indigo); border-radius: 4px; padding: 1px 6px; margin-left: auto; }
  .dot { width: 8px; height: 8px; border-radius: 2px; flex-shrink: 0; }
  .titulo { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); letter-spacing: 0.1em; padding: 12px 8px 4px; }
  .footer { margin-top: auto; padding: 10px 8px 0; border-top: 1px solid var(--borde); display: flex; flex-direction: column; gap: 6px; }
  .ctx { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--texto-2); }
  .nota { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); }

  .item.esp .add {
    background: transparent;
    border: none;
    color: var(--apagado);
    padding: 2px;
    border-radius: 4px;
    cursor: pointer;
    display: none;
    align-items: center;
    justify-content: center;
  }
  .item.esp:hover .add { display: inline-flex; }
  .item.esp .add:hover { background: var(--indigo-suave); color: var(--indigo); }
  :global(.pin-on) { color: var(--indigo); flex-shrink: 0; }
  .fila-esp, .fila-proy { display: flex; align-items: center; gap: 6px; }
  .rename {
    flex: 1;
    font-family: var(--font-ui);
    font-size: 13px;
    padding: 5px 8px;
    border: 1px solid var(--indigo);
    border-radius: var(--radio-m);
    background: var(--superficie);
    outline: none;
    color: var(--tinta);
  }
  .rename.nested { margin-left: 22px; font-size: 12px; padding: 3px 8px; }
</style>
