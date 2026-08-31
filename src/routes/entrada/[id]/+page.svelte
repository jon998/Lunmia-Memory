<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import ArrowLeft from '@lucide/svelte/icons/arrow-left';
  import Bell from '@lucide/svelte/icons/bell';
  import CalendarRange from '@lucide/svelte/icons/calendar-range';
  import Chip from '$lib/components/Chip.svelte';
  import Button from '$lib/components/Button.svelte';
  import ProposalBanner from '$lib/components/ProposalBanner.svelte';
  import OriginalFold from '$lib/components/OriginalFold.svelte';
  import AgendaField from '$lib/components/AgendaField.svelte';
  import LimiteHint from '$lib/components/LimiteHint.svelte';
  import ContenidoVista from '$lib/components/ContenidoVista.svelte';
  import { LIMITES } from '$lib/limites';
  import {
    obtenerEntrada,
    obtenerEventoPorEntrada,
    obtenerRecordatorioPorEntrada,
    actualizarContenido,
    actualizarEtiquetas,
    eliminarEntrada
  } from '$lib/tauri';
  import type { Entrada, Evento } from '$lib/types';
  import { fechaCorta } from '$lib/relative-time';
  import { diasDeRango, etiquetaRango } from '$lib/evento-fecha';
  import { agendaVacia, eventoACivil, persistirAgenda, type AgendaCivil } from '$lib/agenda';
  import { estiloTipo, fusionarTipos } from '$lib/tipos';
  import { app } from '$lib/stores/app.svelte';

  let entrada = $state<Entrada | null>(null);
  let evento = $state<Evento | null>(null);
  let contenido = $state('');
  let etiquetas = $state<string[]>([]);
  let nuevaEtiqueta = $state('');
  let propuestaVisible = $state(false);
  let tipoActual = $state('nota');
  let agenda = $state<AgendaCivil>(agendaVacia());
  let guardado = $state(false);
  let noEncontrada = $state(false);
  let editando = $state(false);

  $effect(() => {
    const id = $page.params.id;
    untrack(() => { void cargar(id); });
  });

  async function cargar(id: string | undefined) {
    entrada = null;
    evento = null;
    noEncontrada = false;
    if (!id) {
      noEncontrada = true;
      return;
    }
    const e = await obtenerEntrada(id);
    if (!e) {
      noEncontrada = true;
      return;
    }
    entrada = e;
    contenido = e.contenido;
    etiquetas = [...e.etiquetas];
    tipoActual = e.tipoNombre ?? 'nota';
    try {
      evento = await obtenerEventoPorEntrada(id);
    } catch {
      evento = null;
    }
    let disparaAt: string | null = null;
    try {
      const rec = await obtenerRecordatorioPorEntrada(id);
      disparaAt = rec?.disparaAt ?? null;
    } catch {
      disparaAt = null;
    }
    agenda = evento ? eventoACivil(evento, undefined, disparaAt) : agendaVacia();
    editando = false;
  }

  const opcionesTipo = $derived(fusionarTipos(app.tipos.map((t) => t.nombre)));
  let nuevoTipo = $state('');

  async function crearTipoDesdeDetalle() {
    const nombre = nuevoTipo.trim().toLowerCase();
    if (!nombre) return;
    try {
      const t = await app.crearTipo(nombre);
      tipoActual = t.nombre;
      nuevoTipo = '';
    } catch (err) {
      console.error(err);
    }
  }

  function agregarEtiqueta(v: string) {
    const t = v.trim().toLowerCase().replace(/\s+/g, '-');
    if (!t || etiquetas.includes(t)) return;
    etiquetas = [...etiquetas, t];
    nuevaEtiqueta = '';
  }

  function quitarEtiqueta(t: string) {
    etiquetas = etiquetas.filter((x) => x !== t);
  }

  async function guardar() {
    if (!entrada) return;
    await Promise.all([
      actualizarContenido(entrada.entryId, contenido),
      actualizarEtiquetas(entrada.entryId, etiquetas)
    ]);
    try {
      await persistirAgenda({
        entryId: entrada.entryId,
        espacioId: entrada.espacioId,
        proyectoId: entrada.proyectoId,
        titulo: contenido.split('\n')[0]?.slice(0, 72) || 'Recordatorio',
        agenda,
        borrarSiInactivo: true
      });
      evento = await obtenerEventoPorEntrada(entrada.entryId);
    } catch (err) {
      console.error(err);
    }
    guardado = true;
    setTimeout(() => (guardado = false), 1800);
  }

  async function eliminar() {
    if (!entrada) return;
    const extra = evento && diasDeRango(evento) > 1
      ? `\n\nCubre ${diasDeRango(evento)} días: se quita el rango entero, no un día suelto.`
      : '\n\nSi tiene fecha, también sale del calendario (todos los días del rango).';
    if (!confirm(`¿Eliminar por completo?\n\n«${contenido.slice(0, 80)}»${extra}`)) return;
    await eliminarEntrada(entrada.entryId);
    await app.avisarAgenda();
    await app.refrescarEntradas();
    goto('/');
  }

  function volver() {
    void goto('/');
  }

  function teclaGlobal(e: KeyboardEvent) {
    if (e.key !== 'Escape') return;
    const t = e.target as HTMLElement | null;
    if (t && (t.tagName === 'INPUT' || t.tagName === 'TEXTAREA' || t.tagName === 'SELECT')) return;
    volver();
  }

  onMount(() => {
    document.addEventListener('keydown', teclaGlobal);
    let unlisten: (() => void) | undefined;
    void (async () => {
      try {
        const { listen } = await import('@tauri-apps/api/event');
        unlisten = await listen<{ entryId?: string }>('entrada:clasificada', (ev) => {
          const id = $page.params.id;
          if (!id || editando) return;
          if (ev.payload?.entryId && ev.payload.entryId !== id) return;
          void cargar(id);
        });
      } catch {
        /* sin Tauri */
      }
    })();
    return () => {
      document.removeEventListener('keydown', teclaGlobal);
      unlisten?.();
    };
  });
</script>

<div class="fondo">
  <div class="ventana">
    {#if entrada}
      <header class="barra">
        <button type="button" class="volver" onclick={volver}>
          <ArrowLeft size={16} strokeWidth={2} />
          Volver
        </button>
        <span class="lugar">{entrada.espacioNombre} / {entrada.proyectoNombre}</span>
        <div class="sello">
          {#if entrada.esProvisional}<Chip variant="alerta">provisional</Chip>{/if}
          {#if entrada.confianzaCapa != null && entrada.confianzaCapa !== 3}
            <Chip variant="alerta" title="Se guardó sin el modelo">sin IA</Chip>
          {/if}
          <Chip variant={estiloTipo(tipoActual)}>{tipoActual}</Chip>
        </div>
      </header>
      <div class="cuerpo">

        <div class="editor">
          {#if entrada.confianzaCapa != null && entrada.confianzaCapa !== 3}
            <div class="aviso-ia">Esta nota se guardó sin IA: el modelo no respondió y el texto quedó como lo escribiste.</div>
          {/if}
          {#if editando}
            <textarea
              bind:value={contenido}
              oninput={() => (propuestaVisible = !!entrada && contenido !== entrada.contenido)}
              maxlength={LIMITES.entrada}
              rows="3"
            ></textarea>
            <LimiteHint valor={contenido} max={LIMITES.entrada} />
            {#if propuestaVisible}
              <div class="banner-wrap">
                <ProposalBanner
                  texto={'Editaste el contenido. Lunmia sugiere reclasificar como <strong>tarea</strong> — tú decides.'}
                  onaceptar={() => { tipoActual = 'tarea'; propuestaVisible = false; }}
                  onignorar={() => (propuestaVisible = false)}
                />
              </div>
            {/if}
            <div class="editor-pie">
              <button type="button" class="link" onclick={() => (editando = false)}>Ver formato</button>
            </div>
          {:else}
            <div class="lectura">
              <ContenidoVista {contenido} />
            </div>
            <div class="editor-pie">
              <button type="button" class="link" onclick={() => (editando = true)}>Editar texto</button>
            </div>
          {/if}
        </div>

        <div class="tres">
          <label>
            <span class="lbl">TIPO</span>
            <select bind:value={tipoActual}>
              {#each opcionesTipo as t (t)}
                <option value={t}>{t}</option>
              {/each}
              {#if tipoActual && !opcionesTipo.includes(tipoActual)}
                <option value={tipoActual}>{tipoActual}</option>
              {/if}
            </select>
            <input
              class="mini-tipo"
              bind:value={nuevoTipo}
              maxlength={LIMITES.nombre}
              placeholder="+ otro tipo ↵"
              onkeydown={(e) => {
                if (e.key === 'Enter' && nuevoTipo.trim()) {
                  e.preventDefault();
                  void crearTipoDesdeDetalle();
                }
              }}
            />
          </label>
          <label>
            <span class="lbl">ESPACIO</span>
            <input value={entrada.espacioNombre} readonly />
          </label>
          <label>
            <span class="lbl">PROYECTO</span>
            <input value={entrada.proyectoNombre} readonly />
          </label>
        </div>

        <div class="tags">
          <span class="lbl">ETIQUETAS</span>
          <div class="tags-wrap">
            {#each etiquetas as t (t)}
              <span class="tag">
                #{t}
                <button type="button" onclick={() => quitarEtiqueta(t)}>×</button>
              </span>
            {/each}
            <input
              bind:value={nuevaEtiqueta}
              maxlength={LIMITES.nombre}
              placeholder="+ añadir (↵)"
              onkeydown={(e) => { if (e.key === 'Enter' && nuevaEtiqueta.trim()) { e.preventDefault(); agregarEtiqueta(nuevaEtiqueta); } }}
            />
            <span class="ai-nota">las propone la IA; libres, sin límite</span>
          </div>
        </div>

        {#if evento && agenda.activo}
          <div class="record">
            <span class="record-icono"><CalendarRange size={17} strokeWidth={1.5} /></span>
            <div class="record-body">
              <div class="record-t">{etiquetaRango(evento)}</div>
              <div class="record-r">
                {#if diasDeRango(evento) > 1}
                  rango de {diasDeRango(evento)} días
                {:else if evento.allDay}
                  todo el día · aviso a las {agenda.hora}
                {:else}
                  {agenda.hora} → {agenda.horaFin} · dispara aunque Lunmia esté cerrada
                {/if}
              </div>
            </div>
          </div>
        {:else}
          <div class="record">
            <span class="record-icono"><Bell size={17} strokeWidth={1.5} /></span>
            <div class="record-body">
              <div class="record-t">Sin fecha en el calendario</div>
              <div class="record-r">marca el recordatorio y elige día y hora</div>
            </div>
          </div>
        {/if}
        <AgendaField bind:value={agenda} />

        <OriginalFold
          contenido={entrada.contenidoOriginal}
          origen={entrada.origen === 'captura_pantalla' ? 'OCR' : entrada.origen}
          fecha={fechaCorta(entrada.createdAt)}
        />

        <div class="pie">
          <span class="pie-meta">{entrada.origen} · {fechaCorta(entrada.createdAt)}</span>
          <div class="acciones">
            <button class="pie-ghost" onclick={eliminar}>Eliminar</button>
            <Button variant="secondary">Archivar</Button>
            <Button variant="primary" onclick={guardar}>{guardado ? 'Guardado ✓' : 'Guardar'}</Button>
          </div>
        </div>
      </div>
    {:else if noEncontrada}
      <div class="cargando">No se encontró esta entrada.</div>
    {:else}
      <div class="cargando">Cargando entrada…</div>
    {/if}
  </div>
</div>

<style>
  .fondo { min-height: 100vh; display: flex; align-items: center; justify-content: center; padding: 32px; background: #E7E7E3; }
  .ventana {
    width: 760px; max-width: 100%;
    background: var(--papel); border-radius: 12px;
    box-shadow: var(--sombra-flotante); border: 1px solid var(--borde);
    overflow: hidden;
  }
  .barra {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 18px;
    background: var(--superficie-2);
    border-bottom: 1px solid var(--borde);
  }
  .volver {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-family: var(--font-ui);
    font-size: 13px;
    font-weight: 600;
    color: var(--tinta);
    background: var(--superficie);
    border: 1px solid var(--tinta);
    border-radius: var(--radio-m);
    padding: 6px 12px 6px 10px;
    cursor: pointer;
    flex-shrink: 0;
    box-shadow: var(--sombra-tecla);
  }
  .volver:hover {
    background: var(--tinta);
    color: var(--papel);
  }
  .lugar {
    font-size: 12px;
    color: var(--texto-2);
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .sello { display: flex; align-items: center; gap: 6px; margin-left: auto; flex-shrink: 0; }
  .cuerpo { padding: 22px 30px 24px; display: flex; flex-direction: column; gap: 16px; }
  .cargando { padding: 60px; text-align: center; color: var(--apagado); }
  .editor {
    background: var(--superficie); border: 1.5px solid var(--tinta);
    border-radius: var(--radio-l); box-shadow: var(--sombra-captura);
    overflow: hidden;
  }
  .aviso-ia {
    font-family: var(--font-mono);
    font-size: 11px;
    line-height: 1.4;
    color: var(--alerta-texto);
    background: var(--alerta-suave);
    border-bottom: 1px solid var(--alerta-borde);
    padding: 8px 16px;
  }
  .editor textarea {
    width: 100%; box-sizing: border-box;
    font-family: var(--font-ui); font-size: 15px; line-height: 1.6;
    padding: 14px 16px; border: none; background: transparent;
    outline: none; resize: vertical; min-height: 84px;
  }
  .banner-wrap { padding: 0 10px 10px; }
  .editor :global(.lim) { display: block; text-align: right; padding: 0 16px 10px; }
  .lectura { padding: 14px 16px; min-width: 0; }
  .editor-pie { padding: 0 16px 12px; }
  .link {
    background: none; border: none; padding: 0;
    font-family: var(--font-mono); font-size: 10px;
    color: var(--apagado); letter-spacing: 0.06em;
    cursor: pointer;
  }
  .link:hover { color: var(--indigo); }
  .tres { display: flex; gap: 10px; }
  .tres label { flex: 1; display: flex; flex-direction: column; gap: 6px; }
  .lbl { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); letter-spacing: 0.1em; }
  .tres select, .tres input {
    font-family: var(--font-ui); font-size: 13px;
    padding: 7px 10px;
    border: 1px solid var(--borde-fuerte); border-radius: var(--radio-m);
    background: var(--superficie); outline: none;
  }
  .mini-tipo {
    font-size: 12px !important;
    padding: 5px 10px !important;
    border-style: dashed !important;
    color: var(--apagado);
  }
  .tags { display: flex; flex-direction: column; gap: 6px; }
  .tags-wrap { display: flex; gap: 6px; flex-wrap: wrap; align-items: center; }
  .tag {
    display: inline-flex; align-items: center; gap: 5px;
    font-family: var(--font-mono); font-size: 11px;
    background: var(--indigo-suave); color: var(--indigo);
    border: 1px solid var(--indigo-borde); border-radius: var(--radio-s);
    padding: 3px 8px;
  }
  .tag button {
    background: transparent; border: none;
    color: var(--apagado); font-size: 12px; cursor: pointer;
    padding: 0;
  }
  .tag button:hover { color: var(--error); }
  .tags-wrap input {
    width: 110px;
    font-family: var(--font-ui); font-size: 12px;
    padding: 4px 8px;
    border: 1px dashed var(--borde-fuerte); border-radius: var(--radio-s);
    background: transparent; outline: none;
  }
  .ai-nota { font-size: 11px; color: var(--apagado); }
  .record {
    display: flex; align-items: center; gap: 12px;
    background: var(--superficie); border: 1px solid var(--borde);
    border-radius: var(--radio-l); padding: 12px 14px;
  }
  .record-icono { color: var(--texto-2); flex-shrink: 0; }
  .record-body { flex: 1; }
  .record-t { font-size: 13px; font-weight: 500; }
  .record-r { font-size: 11px; color: var(--apagado); margin-top: 1px; }
  .pie {
    display: flex; align-items: center; gap: 10px;
    padding-top: 14px; border-top: 1px solid var(--borde);
  }
  .pie-meta {
    font-family: var(--font-mono); font-size: 10px; color: var(--apagado);
    margin-right: auto;
    min-width: 0;
  }
  .acciones { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
  .pie-ghost {
    background: transparent; color: var(--apagado);
    border: none; border-radius: var(--radio-m);
    padding: 6px 10px;
    font-family: var(--font-ui); font-size: 12px; cursor: pointer;
  }
  .pie-ghost:hover { color: var(--error); background: var(--error-suave); }
</style>
