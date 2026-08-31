// Wrappers tipados de invoke() para no manosear strings en la UI.
// Si Tauri no está disponible (dev en navegador puro), usamos mocks locales
// para que la UI se pueda desarrollar sin backend.

import type {
  CapturaConfirmada,
  CapturaOcrResultado,
  Contexto,
  DestinosCaptura,
  Entrada,
  EspacioPropuesto,
  EstadoOnboarding,
  EstadoPermisos,
  Espacio,
  Evento,
  FinalizarOnboardingResultado,
  InfoSistema,
  PropuestaSetup,
  Proyecto,
  Stats,
  TarjetaBandeja,
  Tipo
} from './types';
import { mockAdapter } from './mock-adapter';

async function realInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<T>(cmd, args);
}

function esTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (esTauri()) {
    return realInvoke<T>(cmd, args);
  }
  return mockAdapter<T>(cmd, args);
}

// ─────── espacios / proyectos ───────
export const listarEspacios = () => call<Espacio[]>('listar_espacios');
export const crearEspacio = (params: {
  nombre: string;
  tipo: 'trabajo' | 'personal' | 'cliente' | 'estudio';
  color: string;
}) => call<Espacio>('crear_espacio', { params });
export const eliminarEspacio = (espacioId: string) =>
  call<void>('eliminar_espacio', { espacioId });
export const renombrarEspacio = (espacioId: string, nombre: string) =>
  call<void>('renombrar_espacio', { params: { espacioId, nombre } });
export const fijarEspacio = (espacioId: string, fijado: boolean) =>
  call<void>('fijar_espacio', { params: { espacioId, fijado } });
export const cambiarColorEspacio = (espacioId: string, color: string) =>
  call<void>('cambiar_color_espacio', { params: { espacioId, color } });

export const listarProyectos = (espacioId?: string) =>
  call<Proyecto[]>('listar_proyectos', { params: { espacioId: espacioId ?? null } });
export const crearProyecto = (espacioId: string, nombre: string) =>
  call<Proyecto>('crear_proyecto', { params: { espacioId, nombre } });
export const eliminarProyecto = (proyectoId: string) =>
  call<void>('eliminar_proyecto', { proyectoId });
export const renombrarProyecto = (proyectoId: string, nombre: string) =>
  call<void>('renombrar_proyecto', { params: { proyectoId, nombre } });
export const fijarProyecto = (proyectoId: string, fijado: boolean) =>
  call<void>('fijar_proyecto', { params: { proyectoId, fijado } });

// ─────── contexto activo ───────
export const obtenerContextoActivo = () => call<Contexto | null>('obtener_contexto_activo');
export const destinosCaptura = () => call<DestinosCaptura>('destinos_captura');
export const fijarContexto = (espacioId: string, proyectoId: string, origen: 'usuario' | 'aprendido' = 'usuario') =>
  call<void>('fijar_contexto', { params: { espacioId, proyectoId, origen } });

// ─────── captura ───────
export const capturarTexto = (contenido: string, opts: { espacioId?: string; proyectoId?: string; tipoNombre?: string } = {}) =>
  call<CapturaConfirmada>('capturar_texto', {
    params: {
      contenido,
      espacioId: opts.espacioId ?? null,
      proyectoId: opts.proyectoId ?? null,
      tipoNombre: opts.tipoNombre ?? null
    }
  });

export const capturarPantalla = () => call<CapturaOcrResultado>('capturar_pantalla');

export const estadoOcr = () => call<{ disponible: boolean }>('estado_ocr');

// ─────── entradas ───────
export const listarEntradas = (opts: { espacioId?: string; proyectoId?: string; limit?: number } = {}) =>
  call<Entrada[]>('listar_entradas', {
    params: {
      espacioId: opts.espacioId ?? null,
      proyectoId: opts.proyectoId ?? null,
      limit: opts.limit ?? 200
    }
  });

export const listarTipos = () => call<Tipo[]>('listar_tipos');

export const crearTipo = (nombre: string) =>
  call<Tipo>('crear_tipo', { params: { nombre } });

export const obtenerEntrada = (entryId: string) =>
  call<Entrada | null>('obtener_entrada', { entryId });

export const actualizarContenido = (entryId: string, contenido: string) =>
  call<void>('actualizar_contenido', { params: { entryId, contenido } });

export const actualizarEtiquetas = (entryId: string, etiquetas: string[]) =>
  call<void>('actualizar_etiquetas', { params: { entryId, etiquetas } });

export const eliminarEntrada = (entryId: string) => call<void>('eliminar_entrada', { entryId });

export const moverEntrada = (entryId: string, espacioId: string, proyectoId: string) =>
  call<void>('mover_entrada', { params: { entryId, espacioId, proyectoId } });

// ─────── eventos (calendario) ───────
export const listarEventos = (opts: { desde: string; hasta: string; espacioId?: string | null }) =>
  call<Evento[]>('listar_eventos', {
    params: { desde: opts.desde, hasta: opts.hasta, espacioId: opts.espacioId ?? null }
  });

export const obtenerEvento = (eventoId: string) =>
  call<Evento | null>('obtener_evento', { eventoId });

export const obtenerEventoPorEntrada = (entryId: string) =>
  call<Evento | null>('obtener_evento_por_entrada', { entryId });

export const crearEvento = (params: {
  entryId?: string | null;
  espacioId: string;
  proyectoId?: string | null;
  titulo: string;
  descripcion?: string | null;
  inicioAt: string;
  finAt: string;
  allDay: boolean;
  ubicacion?: string | null;
  rrule?: string | null;
  color?: string | null;
}) =>
  call<string>('crear_evento', {
    params: {
      entryId: params.entryId ?? null,
      espacioId: params.espacioId,
      proyectoId: params.proyectoId ?? null,
      titulo: params.titulo,
      descripcion: params.descripcion ?? null,
      inicioAt: params.inicioAt,
      finAt: params.finAt,
      allDay: params.allDay,
      ubicacion: params.ubicacion ?? null,
      rrule: params.rrule ?? null,
      color: params.color ?? null
    }
  });

export const actualizarEvento = (params: {
  eventoId: string;
  titulo: string;
  descripcion?: string | null;
  inicioAt: string;
  finAt: string;
  allDay: boolean;
  espacioId: string;
  proyectoId?: string | null;
  ubicacion?: string | null;
  rrule?: string | null;
  color?: string | null;
}) =>
  call<void>('actualizar_evento', {
    params: {
      eventoId: params.eventoId,
      titulo: params.titulo,
      descripcion: params.descripcion ?? null,
      inicioAt: params.inicioAt,
      finAt: params.finAt,
      allDay: params.allDay,
      espacioId: params.espacioId,
      proyectoId: params.proyectoId ?? null,
      ubicacion: params.ubicacion ?? null,
      rrule: params.rrule ?? null,
      color: params.color ?? null
    }
  });

export const eliminarEvento = (eventoId: string) =>
  call<void>('eliminar_evento', { eventoId });

export const fijarRecordatorio = (entryId: string, titulo: string, disparaAt: string) =>
  call<void>('fijar_recordatorio', {
    params: { entryId, titulo, disparaAt }
  });

export const obtenerRecordatorioPorEntrada = (entryId: string) =>
  call<{ disparaAt: string } | null>('obtener_recordatorio_por_entrada', { entryId });

// ─────── bandeja ───────
export const listarBandeja = () => call<TarjetaBandeja[]>('listar_bandeja');
export const responderPregunta = (preguntaId: string, opcion: number | null, texto: string | null) =>
  call<void>('responder_pregunta', { params: { preguntaId, opcion, texto } });
export const descartarPregunta = (preguntaId: string) =>
  call<void>('descartar_pregunta', { preguntaId });
export const obtenerStats = () => call<Stats>('obtener_stats');

// ─────── ventanas / sistema ───────
export const abrirVentanaCaptura = () => call<void>('abrir_ventana_captura');
export const cerrarVentanaCaptura = () => call<void>('cerrar_ventana_captura');
export const recentrarVentanaCaptura = () => call<void>('recentrar_ventana_captura');
export const abrirVentanaPrincipal = () => call<void>('abrir_ventana_principal');
export const infoSistema = () => call<InfoSistema>('info_sistema');
export const getConfig = (clave: string) => call<string | null>('get_config', { clave });
export const setConfig = (clave: string, valor: string) => call<void>('set_config', { clave, valor });

// ─────── onboarding ───────
export const estadoOnboarding = () => call<EstadoOnboarding>('estado_onboarding');
export const finalizarOnboarding = (params: {
  oficio: string;
  atajo: string;
  autostart: boolean;
  capturaVoz: boolean;
  tipos: string[];
  espaciosNuevos: EspacioPropuesto[];
  espaciosAEliminar: string[];
  zonaHoraria: string;
}) => call<FinalizarOnboardingResultado>('finalizar_onboarding', { params });
export const proponerSetupInicial = (oficio: string) =>
  call<PropuestaSetup>('proponer_setup_inicial', { oficio });

// ─────── permisos macOS ───────
export const verificarPermisos = () => call<EstadoPermisos>('verificar_permisos');
export const solicitarPermisoNotificaciones = () =>
  call<boolean>('solicitar_permiso_notificaciones');
export const solicitarPermisoAccesibilidad = () =>
  call<boolean>('solicitar_permiso_accesibilidad');
export const solicitarPermisoMicrofono = () =>
  call<boolean>('solicitar_permiso_microfono');

// ─────── logs LLM ───────
export const rutaLogLlm = () => call<string>('ruta_log_llm');
export const abrirLogLlm = () => call<void>('abrir_log_llm');

export const isTauri = esTauri;
