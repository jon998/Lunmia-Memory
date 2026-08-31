import type { Evento } from './types';
import {
  actualizarEvento,
  crearEvento,
  eliminarEvento,
  fijarRecordatorio,
  obtenerEventoPorEntrada
} from './tauri';
import { app } from './stores/app.svelte';
import { diaCalendario, ultimoDiaVisible } from './evento-fecha';
import { getZonaActiva, hm, hoyEnZona, partesEnZona, ymd, zonedCivilToUtc } from './zona';

export interface AgendaCivil {
  activo: boolean;
  /** Primer día (YYYY-MM-DD). */
  fecha: string;
  /** Último día inclusive. Igual a `fecha` si es un solo día. */
  fechaFin: string;
  hora: string;
  /** Hora de fin (reunión). En todo el día se ignora. */
  horaFin: string;
  allDay: boolean;
}

export function agendaVacia(): AgendaCivil {
  const h = hoyEnZona();
  const dia = ymd(h.y, h.m, h.d);
  return { activo: false, fecha: dia, fechaFin: dia, hora: '09:30', horaFin: '10:30', allDay: true };
}

function ymdDeDate(d: Date): string {
  return ymd(d.getFullYear(), d.getMonth() + 1, d.getDate());
}

function diaSiguiente(fecha: string): string {
  const [y, m, d] = fecha.split('-').map(Number);
  return ymdDeDate(new Date(y, m - 1, d + 1));
}

export function fechaFinEfectiva(ag: Pick<AgendaCivil, 'fecha' | 'fechaFin'>): string {
  return ag.fechaFin && ag.fechaFin >= ag.fecha ? ag.fechaFin : ag.fecha;
}

export function diasInclusiveAgenda(ag: Pick<AgendaCivil, 'fecha' | 'fechaFin'>): number {
  const [y1, m1, d1] = ag.fecha.split('-').map(Number);
  const fin = fechaFinEfectiva(ag);
  const [y2, m2, d2] = fin.split('-').map(Number);
  const a = Date.UTC(y1, m1 - 1, d1);
  const b = Date.UTC(y2, m2 - 1, d2);
  return Math.max(1, Math.round((b - a) / 86_400_000) + 1);
}

export function civilAEvento(
  ag: Pick<AgendaCivil, 'fecha' | 'fechaFin' | 'hora' | 'horaFin' | 'allDay'>,
  timeZone = getZonaActiva()
): { inicioAt: string; finAt: string; allDay: boolean } {
  const finDia = fechaFinEfectiva(ag);
  if (ag.allDay) {
    return {
      inicioAt: `${ag.fecha}T00:00:00.000Z`,
      finAt: `${diaSiguiente(finDia)}T00:00:00.000Z`,
      allDay: true
    };
  }
  const inicio = zonedCivilToUtc(ag.fecha, ag.hora, timeZone);
  const fin = zonedCivilToUtc(finDia, ag.horaFin || ag.hora, timeZone);
  const finAt =
    fin.getTime() > inicio.getTime()
      ? fin.toISOString()
      : new Date(inicio.getTime() + 60 * 60 * 1000).toISOString();
  return { inicioAt: inicio.toISOString(), finAt, allDay: false };
}

export function eventoACivil(
  ev: Evento,
  timeZone = getZonaActiva(),
  disparaAt?: string | null
): AgendaCivil {
  const ini = diaCalendario(ev.inicioAt, ev.allDay, timeZone);
  const finVis = ultimoDiaVisible(ev, timeZone);
  const fecha = ymdDeDate(ini);
  const fechaFin = ymdDeDate(finVis);
  if (ev.allDay) {
    let hora = '09:30';
    if (disparaAt) {
      const p = partesEnZona(new Date(disparaAt), timeZone);
      hora = hm(p.h, p.min);
    }
    return { activo: true, fecha, fechaFin, hora, horaFin: '10:30', allDay: true };
  }
  const p = partesEnZona(new Date(ev.inicioAt), timeZone);
  const pf = partesEnZona(new Date(ev.finAt), timeZone);
  return {
    activo: true,
    fecha,
    fechaFin,
    hora: hm(p.h, p.min),
    horaFin: hm(pf.h, pf.min),
    allDay: false
  };
}

export async function persistirAgenda(opts: {
  entryId: string;
  espacioId: string;
  proyectoId: string;
  titulo: string;
  agenda: AgendaCivil | null;
  /** En el detalle: si desactivas el recordatorio, se quita del calendario. En captura no, para no pisar la clasificación. */
  borrarSiInactivo?: boolean;
}): Promise<void> {
  const existente = await obtenerEventoPorEntrada(opts.entryId);
  if (!opts.agenda?.activo || !opts.agenda.fecha) {
    if (opts.borrarSiInactivo && existente) {
      await eliminarEvento(existente.eventoId);
      await app.avisarAgenda();
    }
    return;
  }
  const { inicioAt, finAt, allDay } = civilAEvento(opts.agenda);
  if (existente) {
    await actualizarEvento({
      eventoId: existente.eventoId,
      titulo: opts.titulo,
      descripcion: opts.titulo,
      inicioAt,
      finAt,
      allDay,
      espacioId: opts.espacioId,
      proyectoId: opts.proyectoId
    });
  } else {
    await crearEvento({
      entryId: opts.entryId,
      espacioId: opts.espacioId,
      proyectoId: opts.proyectoId,
      titulo: opts.titulo,
      descripcion: opts.titulo,
      inicioAt,
      finAt,
      allDay
    });
  }
  try {
    await fijarRecordatorio(
      opts.entryId,
      opts.titulo,
      zonedCivilToUtc(opts.agenda.fecha, opts.agenda.hora).toISOString()
    );
  } catch (err) {
    console.error(err);
  }
  await app.avisarAgenda();
}
