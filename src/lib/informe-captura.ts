import { agendaVacia, eventoACivil, type AgendaCivil } from './agenda';
import {
  capturarTexto,
  isTauri,
  obtenerEntrada,
  obtenerEventoPorEntrada,
  obtenerRecordatorioPorEntrada
} from './tauri';

const TIMEOUT_MS = 25_000;

export interface InformeCaptura {
  entryId: string;
  contenido: string;
  tipoNombre: string | null;
  etiquetas: string[];
  espacioId: string;
  proyectoId: string;
  agenda: AgendaCivil;
  /** true si el clasificador LLM no corrió (capa ≠ 3). */
  sinIa: boolean;
}

const vistas = new Set<string>();
const esperas = new Map<string, () => void>();
let listener: Promise<void> | null = null;

async function asegurarEscucha() {
  if (listener) return listener;
  if (!isTauri()) {
    listener = Promise.resolve();
    return listener;
  }
  listener = (async () => {
    const { listen } = await import('@tauri-apps/api/event');
    await listen<{ entryId?: string }>('entrada:clasificada', (ev) => {
      const id = ev.payload?.entryId;
      if (!id) return;
      vistas.add(id);
      const fin = esperas.get(id);
      if (fin) {
        esperas.delete(id);
        fin();
      }
    });
  })();
  return listener;
}

async function esperarClasificacion(entryId: string) {
  await asegurarEscucha();
  if (!isTauri() || vistas.has(entryId)) return;
  await new Promise<void>((resolve) => {
    const t = setTimeout(() => {
      esperas.delete(entryId);
      resolve();
    }, TIMEOUT_MS);
    esperas.set(entryId, () => {
      clearTimeout(t);
      resolve();
    });
  });
}

export async function hidratarInforme(entryId: string): Promise<InformeCaptura> {
  const entrada = await obtenerEntrada(entryId);
  let agenda = agendaVacia();
  try {
    const ev = await obtenerEventoPorEntrada(entryId);
    if (ev) {
      let disparaAt: string | null = null;
      try {
        const rec = await obtenerRecordatorioPorEntrada(entryId);
        disparaAt = rec?.disparaAt ?? null;
      } catch {
        disparaAt = null;
      }
      agenda = eventoACivil(ev, undefined, disparaAt);
    }
  } catch {
    /* sin evento */
  }
  return {
    entryId,
    contenido: entrada?.contenido ?? '',
    tipoNombre: entrada?.tipoNombre ?? null,
    etiquetas: entrada?.etiquetas ?? [],
    espacioId: entrada?.espacioId ?? '',
    proyectoId: entrada?.proyectoId ?? '',
    agenda,
    sinIa: (entrada?.confianzaCapa ?? -1) !== 3
  };
}

/** Guarda al instante y espera a que el pipeline deje tipo, texto y agenda. */
export async function capturarConInforme(
  contenido: string,
  opts: { espacioId?: string; proyectoId?: string; tipoNombre?: string } = {}
): Promise<InformeCaptura> {
  await asegurarEscucha();
  const resp = await capturarTexto(contenido, opts);
  await esperarClasificacion(resp.entryId);
  return hidratarInforme(resp.entryId);
}
