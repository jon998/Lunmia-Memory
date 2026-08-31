import type { Evento } from './types';
import { getZonaActiva, partesEnZona } from './zona';

function pad(n: number): string {
  return String(n).padStart(2, '0');
}

/** Día de calendario. All-day usa la fecha flotante; el resto, la zona preferida. */
export function diaCalendario(iso: string, allDay: boolean, timeZone = getZonaActiva()): Date {
  if (allDay) {
    const m = /^(\d{4})-(\d{2})-(\d{2})/.exec(iso);
    if (m) return new Date(Number(m[1]), Number(m[2]) - 1, Number(m[3]));
  }
  const p = partesEnZona(new Date(iso), timeZone);
  return new Date(p.y, p.m - 1, p.d);
}

/** Fin exclusivo en fecha de calendario (el día posterior al último visible). */
export function finExclusivo(ev: Evento, timeZone = getZonaActiva()): Date {
  const fin = diaCalendario(ev.finAt, ev.allDay, timeZone);
  if (ev.allDay) return fin;
  const p = partesEnZona(new Date(ev.finAt), timeZone);
  if (p.h !== 0 || p.min !== 0) {
    return new Date(p.y, p.m - 1, p.d + 1);
  }
  return fin;
}

export function cubreDia(ev: Evento, day: Date, timeZone = getZonaActiva()): boolean {
  const start = diaCalendario(ev.inicioAt, ev.allDay, timeZone).getTime();
  const end = finExclusivo(ev, timeZone).getTime();
  const t = new Date(day.getFullYear(), day.getMonth(), day.getDate()).getTime();
  if (t >= start && t < end) return true;
  if (end <= start) return t === start;
  return false;
}

export function ultimoDiaVisible(ev: Evento, timeZone = getZonaActiva()): Date {
  const end = finExclusivo(ev, timeZone);
  const last = new Date(end);
  last.setDate(last.getDate() - 1);
  const start = diaCalendario(ev.inicioAt, ev.allDay, timeZone);
  return last < start ? start : last;
}

function cap(s: string): string {
  return s ? s.charAt(0).toUpperCase() + s.slice(1) : s;
}

function fmtDia(d: Date, withWeekday = true): string {
  return cap(
    d.toLocaleDateString('es-ES', {
      weekday: withWeekday ? 'long' : undefined,
      day: 'numeric',
      month: 'short'
    })
  );
}

function fmtHora(iso: string, timeZone = getZonaActiva()): string {
  return new Date(iso).toLocaleTimeString('es-ES', {
    timeZone,
    hour: '2-digit',
    minute: '2-digit'
  });
}

export function etiquetaRango(ev: Evento, timeZone = getZonaActiva()): string {
  const ini = diaCalendario(ev.inicioAt, ev.allDay, timeZone);
  const fin = ultimoDiaVisible(ev, timeZone);
  const mismoDia = ini.getTime() === fin.getTime();
  if (ev.allDay) {
    if (mismoDia) return `${fmtDia(ini)} · todo el día`;
    return `${fmtDia(ini)} → ${fmtDia(fin)}`;
  }
  if (mismoDia) return `${fmtDia(ini)}, ${fmtHora(ev.inicioAt, timeZone)}`;
  return `${fmtDia(ini)} ${fmtHora(ev.inicioAt, timeZone)} → ${fmtDia(fin)} ${fmtHora(ev.finAt, timeZone)}`;
}

export function diasDeRango(ev: Evento, timeZone = getZonaActiva()): number {
  const ini = diaCalendario(ev.inicioAt, ev.allDay, timeZone).getTime();
  const fin = ultimoDiaVisible(ev, timeZone).getTime();
  return Math.max(1, Math.round((fin - ini) / 86_400_000) + 1);
}

export function isoFechaLocal(d: Date, hour = 0, minute = 0): string {
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(hour)}:${pad(minute)}:00.000`;
}
