// Stores globales con Svelte 5 runes. Se acceden por getters/setters
// desde cualquier componente. Cargamos en `+layout.svelte` una sola vez.

import type { Contexto, Entrada, Espacio, Proyecto, Stats, TarjetaBandeja, Tipo } from '$lib/types';
import {
  crearProyecto,
  eliminarEspacio,
  eliminarProyecto,
  fijarEspacio,
  fijarProyecto,
  getConfig,
  infoSistema,
  listarBandeja,
  listarEntradas,
  listarEspacios,
  listarProyectos,
  listarTipos,
  crearTipo,
  obtenerContextoActivo,
  destinosCaptura,
  obtenerStats,
  renombrarEspacio,
  renombrarProyecto,
  setConfig
} from '$lib/tauri';
import { claveZona, detectarZonaSistema, setZonaActiva } from '$lib/zona';

class AppStore {
  espacios = $state<Espacio[]>([]);
  proyectos = $state<Proyecto[]>([]);
  entradas = $state<Entrada[]>([]);
  tipos = $state<Tipo[]>([]);
  bandeja = $state<TarjetaBandeja[]>([]);
  contexto = $state<Contexto | null>(null);
  stats = $state<Stats>({ capturasHoy: 0, sinTriage: 0, latenciaMediaMs: 0, perdidas: 0, provisionales: 0 });
  cargando = $state(false);
  zonaHoraria = $state(detectarZonaSistema());
  agendaRev = $state(0);
  destinosEnVuelo: Promise<void> | null = null;
  /** null = aún no se comprobó. El ping es `info_sistema` contra el proveedor. */
  llmActivo = $state<boolean | null>(null);
  llmAviso = $state<string | null>(null);
  llmPingAt = 0;

  tocarAgenda() {
    this.agendaRev += 1;
  }

  /** Pregunta al backend si el proveedor (DashScope / Ollama) responde ahora. */
  async comprobarLlm(forzar = false) {
    const ahora = Date.now();
    if (!forzar && this.llmActivo !== null && ahora - this.llmPingAt < 8_000) return;
    this.llmPingAt = ahora;
    try {
      const info = await infoSistema();
      this.llmActivo = info.llmActivo;
      this.llmAviso = info.llmAviso ?? null;
    } catch {
      this.llmActivo = false;
      this.llmAviso = 'No se pudo comprobar la IA. La nota se guardará tal cual.';
    }
  }

  async avisarAgenda() {
    this.tocarAgenda();
    if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
      try {
        const { emit } = await import('@tauri-apps/api/event');
        await emit('agenda:cambio');
      } catch {
        /* ventana aislada o sin Tauri */
      }
    }
  }

  async cargarZona() {
    try {
      const guardada = await getConfig(claveZona());
      if (guardada && guardada.trim()) {
        this.zonaHoraria = guardada.trim();
      } else {
        this.zonaHoraria = detectarZonaSistema();
        await setConfig(claveZona(), this.zonaHoraria);
      }
    } catch {
      this.zonaHoraria = detectarZonaSistema();
    }
    setZonaActiva(this.zonaHoraria);
  }

  async guardarZona(id: string) {
    this.zonaHoraria = id;
    setZonaActiva(id);
    await setConfig(claveZona(), id);
  }

  /** Lee espacios, proyectos y contexto activo desde SQLite.
   *  Obligatorio en el flotante: esa ventana no comparte el store de la principal. */
  async hidratarDestinos() {
    if (this.destinosEnVuelo) return this.destinosEnVuelo;
    this.destinosEnVuelo = (async () => {
      const [d, tipos] = await Promise.all([destinosCaptura(), listarTipos().catch(() => [] as Tipo[])]);
      this.espacios = d.espacios;
      this.proyectos = d.proyectos;
      this.contexto = d.contexto;
      this.tipos = tipos;
    })();
    try {
      await this.destinosEnVuelo;
    } finally {
      this.destinosEnVuelo = null;
    }
  }

  async cargarTodo() {
    this.cargando = true;
    await this.cargarZona();
    const [espacios, proyectos, entradas, tipos, bandeja, contexto, stats] = await Promise.all([
      listarEspacios(),
      listarProyectos(),
      listarEntradas({ limit: 200 }),
      listarTipos(),
      listarBandeja(),
      obtenerContextoActivo(),
      obtenerStats()
    ]);
    this.espacios = espacios;
    this.proyectos = proyectos;
    this.entradas = entradas;
    this.tipos = tipos;
    this.bandeja = bandeja;
    this.contexto = contexto;
    this.stats = stats;
    this.cargando = false;
  }

  async refrescarEntradas() {
    const [entradas, bandeja, stats] = await Promise.all([
      listarEntradas({ limit: 200 }),
      listarBandeja(),
      obtenerStats()
    ]);
    this.entradas = entradas;
    this.bandeja = bandeja;
    this.stats = stats;
  }

  proyectosDe(espacioId: string): Proyecto[] {
    return this.proyectos.filter((p) => p.espacioId === espacioId);
  }

  async refrescarEspaciosProyectos() {
    const [espacios, proyectos] = await Promise.all([listarEspacios(), listarProyectos()]);
    this.espacios = espacios;
    this.proyectos = proyectos;
  }

  async renombrarEspacio(espacioId: string, nombre: string) {
    await renombrarEspacio(espacioId, nombre);
    this.espacios = this.espacios.map((e) => (e.espacioId === espacioId ? { ...e, nombre } : e));
  }

  async eliminarEspacio(espacioId: string) {
    await eliminarEspacio(espacioId);
    this.espacios = this.espacios.filter((e) => e.espacioId !== espacioId);
    this.proyectos = this.proyectos.filter((p) => p.espacioId !== espacioId);
  }

  async fijarEspacio(espacioId: string, fijado: boolean) {
    await fijarEspacio(espacioId, fijado);
    this.espacios = this.espacios.map((e) => (e.espacioId === espacioId ? { ...e, fijado } : e));
  }

  async crearTipo(nombre: string) {
    const nuevo = await crearTipo(nombre);
    if (!this.tipos.some((t) => t.tipoId === nuevo.tipoId)) {
      this.tipos = [...this.tipos, nuevo];
    }
    return nuevo;
  }

  async crearProyectoEn(espacioId: string, nombre: string) {
    const nuevo = await crearProyecto(espacioId, nombre);
    this.proyectos = [...this.proyectos, nuevo];
    return nuevo;
  }

  async renombrarProyecto(proyectoId: string, nombre: string) {
    await renombrarProyecto(proyectoId, nombre);
    this.proyectos = this.proyectos.map((p) => (p.proyectoId === proyectoId ? { ...p, nombre } : p));
  }

  async eliminarProyecto(proyectoId: string) {
    await eliminarProyecto(proyectoId);
    this.proyectos = this.proyectos.filter((p) => p.proyectoId !== proyectoId);
  }

  async fijarProyecto(proyectoId: string, fijado: boolean) {
    await fijarProyecto(proyectoId, fijado);
    this.proyectos = this.proyectos.map((p) => (p.proyectoId === proyectoId ? { ...p, fijado } : p));
  }
}

export const app = new AppStore();
