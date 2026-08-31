<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import Search from '@lucide/svelte/icons/search';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import CaptureBox from '$lib/components/CaptureBox.svelte';
  import CalendarGrid from '$lib/components/CalendarGrid.svelte';
  import StatCard from '$lib/components/StatCard.svelte';
  import EntryRow from '$lib/components/EntryRow.svelte';
  import SegmentedSwitch from '$lib/components/SegmentedSwitch.svelte';
  import SavePanel from '$lib/components/SavePanel.svelte';
  import FiltroEntradas from '$lib/components/FiltroEntradas.svelte';
  import { app } from '$lib/stores/app.svelte';
  import { actualizarContenido, eliminarEntrada, listarEventos } from '$lib/tauri';
  import { persistirAgenda, type AgendaCivil } from '$lib/agenda';
  import { capturarConInforme, type InformeCaptura } from '$lib/informe-captura';
  import { hace } from '$lib/relative-time';
  import { cubreDia } from '$lib/evento-fecha';
  import { hoyEnZona } from '$lib/zona';
  import { tituloEntrada } from '$lib/tipos';
  import type { Evento } from '$lib/types';
  import { LIMITES } from '$lib/limites';

  const hoyIni = hoyEnZona();
  let filtro = $state<{ espacioId: string | null; proyectoId: string | null }>({ espacioId: null, proyectoId: null });
  let vista = $state<'lista' | 'calendario'>('lista');
  let busqueda = $state('');
  let tipoFiltro = $state<string | null>(null);
  let eventos = $state<Evento[]>([]);
  let calAnio = $state(hoyIni.y);
  let calMes = $state(hoyIni.m - 1);
  let diaFoco = $state<Date | null>(null);

  async function cargarEventos(espacioId: string | null, anio: number, mes: number) {
    const desde = new Date(anio, mes, 1);
    const hasta = new Date(anio, mes + 1, 1);
    try {
      eventos = await listarEventos({
        desde: desde.toISOString(),
        hasta: hasta.toISOString(),
        espacioId
      });
    } catch (err) {
      console.error(err);
      eventos = [];
    }
  }

  $effect(() => {
    const espacioId = filtro.espacioId;
    const stamp = app.entradas.map((e) => e.updatedAt).join('|');
    void stamp;
    void app.agendaRev;
    void cargarEventos(espacioId, calAnio, calMes);
  });

  onMount(async () => {
    try {
      await app.cargarTodo();
    } catch (err) {
      console.error(err);
    }
  });

  const enHome = $derived(!filtro.espacioId && !filtro.proyectoId && vista === 'lista' && !diaFoco);
  const enDestino = $derived(!!filtro.espacioId || !!filtro.proyectoId);
  const enDia = $derived(!!diaFoco);

  const extrasTipos = $derived(app.tipos.map((t) => t.nombre));

  const entradasFiltradas = $derived.by(() => {
    let lista = app.entradas;
    if (diaFoco) {
      const dia = diaFoco;
      const ids = new Set(
        eventos
          .filter((ev) => cubreDia(ev, dia))
          .map((ev) => ev.entryId)
          .filter((id): id is string => !!id)
      );
      lista = lista.filter((e) => ids.has(e.entryId));
    } else {
      if (enHome) {
        lista = lista.filter((e) => e.esProvisional || !e.tipoNombre || e.estado === 'inbox' || e.estado === 'pendiente_resolucion');
      }
      if (filtro.espacioId) lista = lista.filter((e) => e.espacioId === filtro.espacioId);
      if (filtro.proyectoId) lista = lista.filter((e) => e.proyectoId === filtro.proyectoId);
    }
    if (tipoFiltro) lista = lista.filter((e) => (e.tipoNombre ?? '') === tipoFiltro);
    const q = busqueda.trim().toLowerCase();
    if (q) {
      lista = lista.filter((e) => {
        const titulo = tituloEntrada(e.contenido).toLowerCase();
        const hay = titulo.includes(q) || e.contenido.toLowerCase().includes(q);
        if (enDestino || enDia) return hay || (e.tipoNombre ?? '').toLowerCase().includes(q);
        return (e.contenido + ' ' + (e.tipoNombre ?? '') + ' ' + e.espacioNombre + ' ' + e.proyectoNombre).toLowerCase().includes(q);
      });
    }
    return lista;
  });

  const tituloVista = $derived.by(() => {
    if (diaFoco) {
      return diaFoco.toLocaleDateString('es-ES', { weekday: 'long', day: 'numeric', month: 'short' });
    }
    if (vista === 'calendario') return filtro.espacioId ? `Calendario — ${nombreEspacio(filtro.espacioId)}` : 'Todos los calendarios';
    if (filtro.proyectoId) return app.proyectos.find((p) => p.proyectoId === filtro.proyectoId)?.nombre ?? '';
    if (filtro.espacioId) return nombreEspacio(filtro.espacioId);
    return 'Inicio';
  });

  function nombreEspacio(id: string | null): string {
    if (!id) return '';
    return app.espacios.find((e) => e.espacioId === id)?.nombre ?? '';
  }

  const tituloLista = $derived(enDia ? 'Ese día' : enHome ? 'Pendientes de clasificar' : 'Entradas');
  const notaFiltro = $derived.by(() => {
    const q = busqueda.trim();
    if (q) return `búsqueda · ${entradasFiltradas.length} resultados`;
    if (tipoFiltro) return `${entradasFiltradas.length} de tipo ${tipoFiltro}`;
    if (enDia) return `${entradasFiltradas.length} en el calendario`;
    if (enHome) return `${entradasFiltradas.length} por resolver · lo demás ya vive en sus espacios`;
    if (filtro.espacioId || filtro.proyectoId) return `${entradasFiltradas.length} en esta vista`;
    return '';
  });

  function irAlDia(dia: Date) {
    diaFoco = dia;
    tipoFiltro = null;
    busqueda = '';
    vista = 'lista';
  }

  function volverCalendario() {
    diaFoco = null;
    vista = 'calendario';
  }

  let resumenActivo = $state<InformeCaptura | null>(null);
  let clasificando = $state(false);
  let capturaGen = 0;

  async function guardarCaptura(texto: string, tipoNombre: string | null) {
    const espacioId = app.contexto?.espacioId ?? '';
    const proyectoId = app.contexto?.proyectoId ?? '';
    const gen = ++capturaGen;
    clasificando = true;
    resumenActivo = null;
    try {
      const informe = await capturarConInforme(texto, {
        espacioId: espacioId || undefined,
        proyectoId: proyectoId || undefined,
        tipoNombre: tipoNombre ?? undefined
      });
      await app.refrescarEntradas();
      if (gen !== capturaGen) return;
      if (espacioId && proyectoId) {
        resumenActivo = {
          ...informe,
          espacioId: informe.espacioId || espacioId,
          proyectoId: informe.proyectoId || proyectoId
        };
      }
    } catch (err) {
      console.error(err);
    } finally {
      if (gen === capturaGen) clasificando = false;
    }
  }

  async function confirmarResumen(opts: {
    espacioId: string;
    proyectoId: string;
    prompt: string | null;
    agenda: AgendaCivil | null;
    agendaTocada: boolean;
  }) {
    if (!resumenActivo) return;
    const { entryId } = resumenActivo;
    if (opts.espacioId !== resumenActivo.espacioId || opts.proyectoId !== resumenActivo.proyectoId) {
      try {
        const { moverEntrada } = await import('$lib/tauri');
        await moverEntrada(entryId, opts.espacioId, opts.proyectoId);
      } catch (err) { console.error(err); }
    }
    if (opts.prompt) {
      try {
        await actualizarContenido(entryId, `${resumenActivo.contenido}\n\n[corrección]: ${opts.prompt}`);
      } catch (err) { console.error(err); }
    }
    try {
      await persistirAgenda({
        entryId,
        espacioId: opts.espacioId,
        proyectoId: opts.proyectoId,
        titulo: resumenActivo.contenido.split('\n')[0]?.slice(0, 72) || 'Recordatorio',
        agenda: opts.agenda,
        borrarSiInactivo: opts.agendaTocada
      });
    } catch (err) {
      console.error(err);
    }
    await app.refrescarEntradas();
  }

  function confirmarBorrarTodo(contenido: string): boolean {
    return confirm(
      `¿Eliminar por completo?\n\n«${contenido.slice(0, 80)}»\n\nSe borra la nota, el recordatorio y el evento entero del calendario (todos los días del rango, no un día suelto).`
    );
  }

  async function eliminarDeLista(entryId: string, contenido: string) {
    if (!confirmarBorrarTodo(contenido)) return;
    await eliminarEntrada(entryId);
    await app.avisarAgenda();
    await app.refrescarEntradas();
  }
</script>

<div class="app">
  <Sidebar
    {filtro}
    onfiltro={(f) => { filtro = f; busqueda = ''; tipoFiltro = null; diaFoco = null; }}
    {vista}
    onvista={(v) => { vista = v; if (v === 'calendario') diaFoco = null; }}
  />

  <main class="main">
    <div class="header">
      <div class="titulo">{tituloVista}</div>
      <SegmentedSwitch
        options={[
          { value: 'lista', label: 'Lista' },
          { value: 'calendario', label: 'Calendario' }
        ]}
        value={vista}
        onchange={(v) => {
          vista = v as 'lista' | 'calendario';
          if (v === 'calendario') diaFoco = null;
        }}
      />
      {#if vista === 'lista' && !enDestino}
        <div class="buscar">
          <span class="icono"><Search size={15} strokeWidth={1.5} /></span>
          <input
            bind:value={busqueda}
            maxlength={LIMITES.busqueda}
            placeholder="Buscar por significado, no por palabra…"
          />
        </div>
      {/if}
    </div>

    {#if vista === 'lista'}
      {#if enHome}
        <CaptureBox
          contexto={app.contexto}
          extrasTipos={extrasTipos}
          llmActivo={app.llmActivo}
          llmAviso={app.llmAviso}
          onguardar={guardarCaptura}
        />

        <div class="stats">
          <StatCard valor={app.stats.capturasHoy} etiqueta="capturas hoy" />
          <StatCard valor={app.stats.sinTriage} etiqueta="sin triage" color="var(--indigo)" />
          <StatCard valor={`${(app.stats.latenciaMediaMs / 1000).toFixed(1)}s`} etiqueta="latencia media" />
          <StatCard valor={app.stats.perdidas} etiqueta="perdidas" color="var(--ok)" />
          <StatCard valor={app.stats.provisionales} etiqueta="provisionales" color="var(--alerta)" />
        </div>
      {/if}

      {#if enDestino || enDia}
        <FiltroEntradas
          {busqueda}
          tipo={tipoFiltro}
          extras={extrasTipos}
          onbusqueda={(q) => busqueda = q}
          ontipo={(t) => tipoFiltro = t}
        />
      {/if}

      <div class="lista">
        <div class="lista-cab">
          <span class="lista-t">{tituloLista}</span>
          <span class="lista-n">{notaFiltro}</span>
          {#if enDia}
            <button type="button" class="volver-cal" onclick={volverCalendario}>← calendario</button>
          {/if}
        </div>
        {#each entradasFiltradas as e (e.entryId)}
          <EntryRow
            entrada={e}
            hace={hace(e.createdAt)}
            mostrarFlecha={true}
            onclick={() => goto(enHome ? '/bandeja' : `/entrada/${e.entryId}`)}
            oneliminar={() => void eliminarDeLista(e.entryId, e.contenido)}
          />
        {/each}
        {#if entradasFiltradas.length === 0}
          <div class="vacio">Nada por aquí. O lo capturaste con otras palabras — prueba describirlo distinto.</div>
        {/if}
        {#if enHome && entradasFiltradas.length > 0}
          <a class="cta" href="/bandeja">Resolverlas en la Bandeja →</a>
        {/if}
      </div>
    {:else}
      <CalendarGrid
        eventos={eventos}
        espacioFiltro={filtro.espacioId}
        anio={calAnio}
        mes={calMes}
        onnavegar={(a, m) => { calAnio = a; calMes = m; }}
        onvermas={irAlDia}
      />
    {/if}
  </main>

  {#if clasificando}
    <div class="save-toast">
      <div class="pensando">
        <span class="pulso"></span>
        {#if app.llmActivo === false}
          Guardada · sin IA, el texto queda tal cual
        {:else}
          Guardada · Lunmia está clasificando…
        {/if}
      </div>
    </div>
  {:else if resumenActivo}
    <div class="save-toast">
      <SavePanel
        resumen={resumenActivo.contenido}
        espacios={app.espacios}
        proyectosDe={(id) => app.proyectosDe(id)}
        espacioId={resumenActivo.espacioId}
        proyectoId={resumenActivo.proyectoId}
        tipoNombre={resumenActivo.tipoNombre}
        etiquetas={resumenActivo.etiquetas}
        agendaInicial={resumenActivo.agenda}
        sinIa={resumenActivo.sinIa}
        onguardar={confirmarResumen}
        onclose={() => resumenActivo = null}
      />
    </div>
  {/if}
</div>

<style>
  .app { display: flex; height: 100vh; background: var(--papel); }
  .main { flex: 1; display: flex; flex-direction: column; padding: 22px 26px; gap: 16px; overflow: hidden; min-width: 0; }
  .header { display: flex; align-items: center; gap: 12px; }
  .titulo { font-family: var(--font-display); font-size: 20px; font-weight: 700; text-transform: capitalize; }
  .buscar { margin-left: auto; position: relative; width: 300px; }
  .buscar .icono { position: absolute; left: 10px; top: 8px; color: var(--apagado); }
  .buscar input {
    width: 100%;
    box-sizing: border-box;
    font-family: var(--font-ui);
    font-size: 13px;
    padding: 7px 12px 7px 32px;
    border: 1px solid var(--borde-fuerte);
    border-radius: var(--radio-m);
    background: var(--superficie);
    outline: none;
  }
  .stats { display: flex; gap: 10px; }
  .stats :global(.stat) { flex: 1; }
  .lista { flex: 1; display: flex; flex-direction: column; gap: 6px; overflow-y: auto; min-height: 0; }
  .lista-cab { display: flex; align-items: baseline; gap: 8px; }
  .lista-t { font-size: 13px; font-weight: 600; }
  .lista-n { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); }
  .volver-cal {
    margin-left: auto;
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.04em;
    color: var(--indigo);
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0;
  }
  .volver-cal:hover { text-decoration: underline; }
  .vacio { text-align: center; padding: 24px; font-size: 13px; color: var(--apagado); }
  .cta {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    border: 1px dashed var(--borde-fuerte);
    border-radius: var(--radio-l);
    padding: 9px 12px;
    font-size: 12px;
    font-weight: 500;
    color: var(--indigo);
    text-decoration: none;
    background: transparent;
  }
  .cta:hover { background: var(--indigo-suave); border-color: var(--indigo-borde); }
  .save-toast { position: fixed; bottom: 20px; left: 20px; z-index: 55; max-width: min(420px, calc(100vw - 40px)); }
  .pensando {
    display: flex;
    align-items: center;
    gap: 10px;
    background: var(--papel);
    border: 1px solid var(--borde);
    border-radius: var(--radio-xl);
    box-shadow: var(--sombra-flotante);
    padding: 12px 16px;
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--apagado);
  }
  .pulso {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--indigo);
    flex-shrink: 0;
    animation: lm-pulso 1s ease-in-out infinite;
  }
  @keyframes lm-pulso {
    0%, 100% { opacity: 0.35; transform: scale(0.85); }
    50% { opacity: 1; transform: scale(1); }
  }
</style>
