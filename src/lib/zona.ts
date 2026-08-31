/** Preferencia de zona IANA. Afecta calendario, etiquetas de fecha y “hoy/mañana”. */

export interface Zona {
  id: string;
  ciudad: string;
  grupo: string;
}

export const ZONAS: Zona[] = [
  { id: 'America/Mexico_City', ciudad: 'Ciudad de México', grupo: 'México' },
  { id: 'America/Tijuana', ciudad: 'Tijuana', grupo: 'México' },
  { id: 'America/Hermosillo', ciudad: 'Hermosillo', grupo: 'México' },
  { id: 'America/Mazatlan', ciudad: 'Mazatlán', grupo: 'México' },
  { id: 'America/Merida', ciudad: 'Mérida', grupo: 'México' },
  { id: 'America/Cancun', ciudad: 'Cancún', grupo: 'México' },
  { id: 'America/Chihuahua', ciudad: 'Chihuahua', grupo: 'México' },
  { id: 'America/Guatemala', ciudad: 'Guatemala', grupo: 'América' },
  { id: 'America/Bogota', ciudad: 'Bogotá', grupo: 'América' },
  { id: 'America/Lima', ciudad: 'Lima', grupo: 'América' },
  { id: 'America/Panama', ciudad: 'Panamá', grupo: 'América' },
  { id: 'America/Caracas', ciudad: 'Caracas', grupo: 'América' },
  { id: 'America/Santiago', ciudad: 'Santiago', grupo: 'América' },
  { id: 'America/Argentina/Buenos_Aires', ciudad: 'Buenos Aires', grupo: 'América' },
  { id: 'America/Sao_Paulo', ciudad: 'São Paulo', grupo: 'América' },
  { id: 'America/New_York', ciudad: 'Nueva York', grupo: 'América' },
  { id: 'America/Chicago', ciudad: 'Chicago', grupo: 'América' },
  { id: 'America/Denver', ciudad: 'Denver', grupo: 'América' },
  { id: 'America/Los_Angeles', ciudad: 'Los Ángeles', grupo: 'América' },
  { id: 'America/Toronto', ciudad: 'Toronto', grupo: 'América' },
  { id: 'Atlantic/Canary', ciudad: 'Islas Canarias', grupo: 'Europa' },
  { id: 'Europe/Madrid', ciudad: 'Madrid', grupo: 'Europa' },
  { id: 'Europe/London', ciudad: 'Londres', grupo: 'Europa' },
  { id: 'Europe/Paris', ciudad: 'París', grupo: 'Europa' },
  { id: 'Europe/Berlin', ciudad: 'Berlín', grupo: 'Europa' },
  { id: 'Europe/Lisbon', ciudad: 'Lisboa', grupo: 'Europa' },
  { id: 'Africa/Lagos', ciudad: 'Lagos', grupo: 'África' },
  { id: 'Asia/Dubai', ciudad: 'Dubái', grupo: 'Asia' },
  { id: 'Asia/Shanghai', ciudad: 'Shanghái', grupo: 'Asia' },
  { id: 'Asia/Tokyo', ciudad: 'Tokio', grupo: 'Asia' },
  { id: 'Australia/Sydney', ciudad: 'Sídney', grupo: 'Pacífico' },
  { id: 'Pacific/Auckland', ciudad: 'Auckland', grupo: 'Pacífico' },
  { id: 'UTC', ciudad: 'UTC', grupo: 'Otras' }
];

const CLAVE = 'zona_horaria';

let activa = detectarZonaSistema();

export function claveZona(): string {
  return CLAVE;
}

export function getZonaActiva(): string {
  return activa;
}

export function setZonaActiva(id: string) {
  activa = esZonaValida(id) ? id : detectarZonaSistema();
}

export function detectarZonaSistema(): string {
  try {
    const id = Intl.DateTimeFormat().resolvedOptions().timeZone;
    if (id && esZonaValida(id)) return id;
  } catch {
    /* ignore */
  }
  return 'America/Mexico_City';
}

export function esZonaValida(id: string): boolean {
  try {
    Intl.DateTimeFormat('en', { timeZone: id }).format(new Date());
    return true;
  } catch {
    return false;
  }
}

export function etiquetaZona(id: string): string {
  const z = ZONAS.find((x) => x.id === id);
  if (z) return z.ciudad;
  return id.replace(/_/g, ' ').split('/').pop() ?? id;
}

export function offsetZona(id: string, at = new Date()): string {
  try {
    const parts = new Intl.DateTimeFormat('en-US', {
      timeZone: id,
      timeZoneName: 'shortOffset'
    }).formatToParts(at);
    return parts.find((p) => p.type === 'timeZoneName')?.value ?? '';
  } catch {
    return '';
  }
}

export function horaEnZona(id: string, at = new Date()): string {
  return at.toLocaleTimeString('es-MX', {
    timeZone: id,
    hour: '2-digit',
    minute: '2-digit'
  });
}

export interface PartesFecha {
  y: number;
  m: number;
  d: number;
  h: number;
  min: number;
}

export function partesEnZona(at: Date, timeZone = getZonaActiva()): PartesFecha {
  const fmt = new Intl.DateTimeFormat('en-CA', {
    timeZone,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hourCycle: 'h23'
  });
  const parts = fmt.formatToParts(at);
  const num = (type: Intl.DateTimeFormatPartTypes) =>
    Number(parts.find((p) => p.type === type)?.value ?? 0);
  return { y: num('year'), m: num('month'), d: num('day'), h: num('hour'), min: num('minute') };
}

export function hoyEnZona(timeZone = getZonaActiva()): { y: number; m: number; d: number } {
  const p = partesEnZona(new Date(), timeZone);
  return { y: p.y, m: p.m, d: p.d };
}

function pad(n: number): string {
  return String(n).padStart(2, '0');
}

export function ymd(y: number, m: number, d: number): string {
  return `${y}-${pad(m)}-${pad(d)}`;
}

export function hm(h: number, min: number): string {
  return `${pad(h)}:${pad(min)}`;
}

/** Interpreta fecha+hora civil en la zona preferida y devuelve el instante UTC. */
export function zonedCivilToUtc(fecha: string, hora: string, timeZone = getZonaActiva()): Date {
  const [y, m, d] = fecha.split('-').map(Number);
  const [hh, mm] = hora.split(':').map(Number);
  const guess = new Date(Date.UTC(y, m - 1, d, hh, mm, 0));
  const p = partesEnZona(guess, timeZone);
  const asZone = Date.UTC(p.y, p.m - 1, p.d, p.h, p.min);
  const wanted = Date.UTC(y, m - 1, d, hh, mm);
  return new Date(guess.getTime() - (asZone - wanted));
}
