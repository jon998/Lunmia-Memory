<script lang="ts">
  import { goto } from '$app/navigation';
  import ChevronLeft from '@lucide/svelte/icons/chevron-left';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';
  import type { Evento } from '$lib/types';
  import {
    cubreDia,
    diaCalendario,
    etiquetaRango,
    ultimoDiaVisible
  } from '$lib/evento-fecha';
  import { hoyEnZona } from '$lib/zona';
  import { app } from '$lib/stores/app.svelte';

  interface Props {
    eventos: Evento[];
    espacioFiltro?: string | null;
    anio: number;
    mes: number;
    onnavegar: (anio: number, mes: number) => void;
    onvermas?: (dia: Date) => void;
  }
  let { eventos, espacioFiltro = null, anio, mes, onnavegar, onvermas }: Props = $props();

  const MAX_LANES = 3;
  const MESES = [
    'enero', 'febrero', 'marzo', 'abril', 'mayo', 'junio',
    'julio', 'agosto', 'septiembre', 'octubre', 'noviembre', 'diciembre'
  ];

  const hoyZ = $derived(hoyEnZona(app.zonaHoraria));
  const anios = $derived.by(() => {
    const y = hoyZ.y;
    const arr: number[] = [];
    for (let i = y - 6; i <= y + 8; i++) arr.push(i);
    if (!arr.includes(anio)) arr.push(anio);
    return arr.sort((a, b) => a - b);
  });
  const esHoyVisible = $derived(anio === hoyZ.y && mes === hoyZ.m - 1);

  function prevMes() {
    if (mes === 0) onnavegar(anio - 1, 11);
    else onnavegar(anio, mes - 1);
  }
  function nextMes() {
    if (mes === 11) onnavegar(anio + 1, 0);
    else onnavegar(anio, mes + 1);
  }
  function irHoy() {
    onnavegar(hoyZ.y, hoyZ.m - 1);
  }

  interface Celda {
    date: Date | null;
    num: number | '';
    hoy: boolean;
  }
  interface Seg {
    evento: Evento;
    col: number;
    span: number;
    esInicio: boolean;
    esFin: boolean;
    titulo: string;
    color: string;
    href: string | null;
  }
  interface Semana {
    dias: Celda[];
    lanes: Seg[][];
    extras: Array<{ col: number; n: number; date: Date }>;
  }

  const filtrados = $derived(
    eventos.filter((e) => !espacioFiltro || e.espacioId === espacioFiltro)
  );

  const celdas = $derived.by((): Celda[] => {
    const primero = new Date(anio, mes, 1);
    const diasEnMes = new Date(anio, mes + 1, 0).getDate();
    const startOffset = (primero.getDay() + 6) % 7;
    const arr: Celda[] = [];
    for (let i = 0; i < startOffset; i++) arr.push({ date: null, num: '', hoy: false });
    for (let d = 1; d <= diasEnMes; d++) {
      const date = new Date(anio, mes, d);
      arr.push({
        date,
        num: d,
        hoy: anio === hoyZ.y && mes === hoyZ.m - 1 && d === hoyZ.d
      });
    }
    while (arr.length < 42) arr.push({ date: null, num: '', hoy: false });
    return arr;
  });

  const semanas = $derived.by((): Semana[] => {
    const out: Semana[] = [];
    for (let w = 0; w < 6; w++) {
      const dias = celdas.slice(w * 7, w * 7 + 7);
      const weekStart = dias.find((d) => d.date)?.date ?? null;
      const weekEnd = [...dias].reverse().find((d) => d.date)?.date ?? null;
      if (!weekStart || !weekEnd) {
        out.push({ dias, lanes: [], extras: [] });
        continue;
      }

      const overlapping = filtrados
        .filter((ev) => dias.some((d) => d.date && cubreDia(ev, d.date)))
        .sort((a, b) => {
          const da = diaCalendario(a.inicioAt, a.allDay).getTime();
          const db = diaCalendario(b.inicioAt, b.allDay).getTime();
          if (da !== db) return da - db;
          return ultimoDiaVisible(b).getTime() - ultimoDiaVisible(a).getTime();
        });

      const segs: Seg[] = overlapping.map((ev) => {
        const evStart = diaCalendario(ev.inicioAt, ev.allDay);
        const evLast = ultimoDiaVisible(ev);
        let col0 = 0;
        let col1 = 6;
        for (let i = 0; i < 7; i++) {
          if (dias[i].date && dias[i].date!.getTime() >= evStart.getTime()) {
            col0 = i;
            break;
          }
        }
        for (let i = 6; i >= 0; i--) {
          if (dias[i].date && dias[i].date!.getTime() <= evLast.getTime()) {
            col1 = i;
            break;
          }
        }
        const href = ev.entryId ? `/entrada/${ev.entryId}` : null;
        return {
          evento: ev,
          col: col0 + 1,
          span: Math.max(1, col1 - col0 + 1),
          esInicio: evStart.getTime() >= weekStart.getTime(),
          esFin: evLast.getTime() <= weekEnd.getTime(),
          titulo: ev.titulo,
          color: ev.color || ev.espacioColor,
          href
        };
      });

      const lanes: Seg[][] = [];
      const overflow: Seg[] = [];
      for (const seg of segs) {
        let placed = false;
        for (const lane of lanes) {
          const choca = lane.some((s) => {
            const a0 = s.col;
            const a1 = s.col + s.span;
            const b0 = seg.col;
            const b1 = seg.col + seg.span;
            return a0 < b1 && b0 < a1;
          });
          if (!choca) {
            lane.push(seg);
            placed = true;
            break;
          }
        }
        if (placed) continue;
        if (lanes.length < MAX_LANES) {
          lanes.push([seg]);
        } else {
          overflow.push(seg);
        }
      }

      const extraMap = new Map<number, number>();
      for (const s of overflow) {
        for (let c = s.col; c < s.col + s.span; c++) {
          extraMap.set(c, (extraMap.get(c) ?? 0) + 1);
        }
      }
      const extras = [...extraMap.entries()]
        .map(([col, n]) => {
          const date = dias[col - 1]?.date;
          return date ? { col, n, date } : null;
        })
        .filter((x): x is { col: number; n: number; date: Date } => x !== null);
      out.push({ dias, lanes, extras });
    }
    return out;
  });

  function abrir(seg: Seg, e?: Event) {
    e?.preventDefault();
    e?.stopPropagation();
    if (seg.href) void goto(seg.href);
  }
</script>

<div class="grid-wrap">
  <div class="cabecera">
    <div class="nav">
      <button type="button" class="nav-btn" onclick={prevMes} title="Mes anterior" aria-label="Mes anterior">
        <ChevronLeft size={16} strokeWidth={1.75} />
      </button>
      <select class="sel mes-sel" value={mes} onchange={(e) => onnavegar(anio, Number((e.currentTarget as HTMLSelectElement).value))} aria-label="Mes">
        {#each MESES as nombre, i}
          <option value={i}>{nombre}</option>
        {/each}
      </select>
      <select class="sel anio-sel" value={anio} onchange={(e) => onnavegar(Number((e.currentTarget as HTMLSelectElement).value), mes)} aria-label="Año">
        {#each anios as y}
          <option value={y}>{y}</option>
        {/each}
      </select>
      <button type="button" class="nav-btn" onclick={nextMes} title="Mes siguiente" aria-label="Mes siguiente">
        <ChevronRight size={16} strokeWidth={1.75} />
      </button>
      {#if !esHoyVisible}
        <button type="button" class="hoy-btn" onclick={irHoy}>Hoy</button>
      {/if}
    </div>
    <span class="nota">
      {espacioFiltro ? '1 espacio = 1 calendario' : 'vista agregada · crear un evento aquí pide elegir Espacio'}
    </span>
  </div>
  <div class="header">
    <span>LUN</span><span>MAR</span><span>MIÉ</span><span>JUE</span><span>VIE</span><span>SÁB</span><span>DOM</span>
  </div>
  <div class="semanas">
    {#each semanas as sem}
      <div class="semana">
        <div class="celdas">
          {#each sem.dias as d}
            <div class="celda" class:vacia={d.num === ''} class:hoy={d.hoy}>
              {#if d.num !== ''}
                <span class="num">{d.num}</span>
              {/if}
            </div>
          {/each}
        </div>
        {#if sem.lanes.length > 0 || sem.extras.length > 0}
          <div class="capa" style:--lanes={sem.lanes.length + (sem.extras.length ? 1 : 0)}>
            {#each sem.lanes as lane, li}
              <div class="lane" style:grid-row={li + 1}>
                {#each lane as seg}
                  <button
                    type="button"
                    class="barra"
                    class:inicio={seg.esInicio}
                    class:fin={seg.esFin}
                    class:rango={seg.span > 1}
                    title="{etiquetaRango(seg.evento)} · Abrir detalle"
                    style="grid-column: {seg.col} / span {seg.span}; --ev: {seg.color}"
                    onclick={(e) => abrir(seg, e)}
                  >
                    <span class="acento"></span>
                    <span class="titulo">{seg.titulo}</span>
                  </button>
                {/each}
              </div>
            {/each}
            {#each sem.extras as extra}
              <button
                type="button"
                class="extra"
                style:grid-column={extra.col}
                style:grid-row={sem.lanes.length + 1}
                title="Ver este día en la lista"
                onclick={(e) => {
                  e.stopPropagation();
                  onvermas?.(extra.date);
                }}
              >
                ver más
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/each}
  </div>
  {#if filtrados.length === 0}
    <div class="vacio">Nada con fecha este mes. Prueba «vacaciones del 18 al 22» — el rango pinta todos los días.</div>
  {/if}
</div>

<style>
  .grid-wrap { display: flex; flex-direction: column; gap: 10px; flex: 1; min-height: 0; }
  .cabecera { display: flex; align-items: center; gap: 14px; }
  .nav { display: flex; align-items: center; gap: 6px; }
  .nav-btn {
    width: 28px; height: 28px;
    display: inline-flex; align-items: center; justify-content: center;
    border: 1px solid var(--borde); border-radius: var(--radio-m);
    background: var(--superficie); color: var(--tinta); cursor: pointer;
  }
  .nav-btn:hover { background: var(--indigo-suave); border-color: var(--indigo-borde); color: var(--indigo); }
  .sel {
    font-family: var(--font-display); font-size: 15px; font-weight: 600;
    text-transform: capitalize;
    border: 1px solid transparent; background: transparent;
    color: var(--tinta); padding: 2px 4px; border-radius: var(--radio-s);
    cursor: pointer;
  }
  .sel:hover, .sel:focus { border-color: var(--borde); background: var(--superficie); }
  .anio-sel { font-family: var(--font-mono); font-size: 13px; font-weight: 500; text-transform: none; }
  .hoy-btn {
    font-family: var(--font-mono); font-size: 10px; letter-spacing: 0.06em;
    background: var(--indigo-suave); color: var(--indigo);
    border: 1px solid var(--indigo-borde); border-radius: var(--radio-s);
    padding: 3px 8px; cursor: pointer;
  }
  .hoy-btn:hover { background: #E2E0F6; }
  .nota { margin-left: auto; font-size: 11px; color: var(--apagado); }
  .header {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 4px;
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--apagado);
  }
  .header span { padding: 0 6px; }
  .semanas {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-height: 0;
    overflow-y: auto;
  }
  .semana {
    position: relative;
    flex: 1;
    min-height: 72px;
  }
  .celdas {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 4px;
    height: 100%;
  }
  .celda {
    background: var(--superficie);
    border: 1px solid var(--borde);
    border-radius: var(--radio-m);
    padding: 5px 6px;
    min-height: 66px;
  }
  .celda.vacia { background: transparent; border-color: transparent; }
  .celda.hoy { border-color: var(--indigo); }
  .celda.hoy .num { color: var(--indigo); }
  .num { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); }
  .capa {
    position: absolute;
    left: 0;
    right: 0;
    top: 22px;
    bottom: 4px;
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    grid-template-rows: repeat(var(--lanes, 1), 22px);
    gap: 2px 4px;
    padding: 0 2px;
    pointer-events: none;
  }
  .lane {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    grid-column: 1 / -1;
    gap: 4px;
    pointer-events: none;
  }
  .barra {
    display: flex;
    align-items: center;
    gap: 0;
    min-width: 0;
    height: 22px;
    font-size: 10px;
    font-family: var(--font-ui);
    color: var(--tinta);
    background: color-mix(in srgb, var(--ev) 18%, var(--papel));
    border: 1px solid color-mix(in srgb, var(--ev) 38%, var(--borde));
    border-radius: 3px;
    padding: 0;
    text-decoration: none;
    cursor: pointer;
    overflow: hidden;
    pointer-events: auto;
    line-height: 1;
    text-align: left;
    appearance: none;
    -webkit-appearance: none;
    margin: 0;
  }
  .barra:not(.inicio) { border-top-left-radius: 0; border-bottom-left-radius: 0; margin-left: -2px; }
  .barra:not(.fin) { border-top-right-radius: 0; border-bottom-right-radius: 0; margin-right: -2px; }
  .barra.rango { font-weight: 500; }
  .barra:hover {
    background: color-mix(in srgb, var(--ev) 32%, var(--papel));
    color: var(--tinta);
  }
  .acento {
    width: 3px;
    align-self: stretch;
    background: var(--ev);
    flex-shrink: 0;
  }
  .titulo {
    padding: 0 6px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .extra {
    font-family: var(--font-mono);
    font-size: 9px;
    letter-spacing: 0.04em;
    color: var(--indigo);
    padding: 0 4px;
    height: 22px;
    display: flex;
    align-items: center;
    background: transparent;
    border: none;
    cursor: pointer;
    pointer-events: auto;
    text-align: left;
    appearance: none;
    -webkit-appearance: none;
  }
  .extra:hover { text-decoration: underline; }
  .vacio { text-align: center; padding: 8px; font-size: 12px; color: var(--apagado); }
</style>
