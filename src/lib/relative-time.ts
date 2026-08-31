import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import 'dayjs/locale/es';
import { getZonaActiva } from './zona';

dayjs.extend(relativeTime);
dayjs.locale('es');

export function hace(iso: string): string {
  const d = new Date(iso).getTime();
  const diff = Date.now() - d;
  if (diff < 60_000) return 'ahora';
  if (diff < 3_600_000) return `hace ${Math.floor(diff / 60_000)}m`;
  if (diff < 86_400_000) return `hace ${Math.floor(diff / 3_600_000)}h`;
  if (diff < 172_800_000) return 'ayer';
  if (diff < 604_800_000) return `hace ${Math.floor(diff / 86_400_000)}d`;
  return dayjs(iso).format('D MMM YYYY');
}

export function fechaCorta(iso: string, timeZone = getZonaActiva()): string {
  return new Date(iso).toLocaleString('es-ES', {
    timeZone,
    day: 'numeric',
    month: 'short',
    hour: '2-digit',
    minute: '2-digit'
  });
}
