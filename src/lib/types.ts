// Tipos compartidos con el backend Rust. Los campos siguen la serialización
// serde con `rename_all = "camelCase"`.

export type EstadoEntrada =
  | 'inbox'
  | 'pendiente_resolucion'
  | 'triaged'
  | 'activo'
  | 'archivado';

export type OrigenEntrada = 'texto' | 'voz' | 'captura_pantalla' | 'telegram';

export type TipoEspacio = 'trabajo' | 'personal' | 'cliente' | 'estudio';

export interface Espacio {
  espacioId: string;
  nombre: string;
  tipo: TipoEspacio;
  color: string;
  fijado: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface Proyecto {
  proyectoId: string;
  espacioId: string;
  nombre: string;
  esPorDefecto: boolean;
  fijado: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface Tipo {
  tipoId: string;
  nombre: string;
  descripcion: string | null;
  usosTotal: number;
  archivado: boolean;
  ultimaUsoAt: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface Entrada {
  entryId: string;
  contenidoOriginal: string;
  contenido: string;
  tipoId: string | null;
  tipoNombre: string | null;
  espacioId: string;
  espacioNombre: string;
  espacioColor: string;
  proyectoId: string;
  proyectoNombre: string;
  esProvisional: boolean;
  estado: EstadoEntrada;
  origen: OrigenEntrada;
  metadataCaptura: string | null;
  confianza: number | null;
  confianzaCapa: number | null;
  etiquetas: string[];
  createdAt: string;
  updatedAt: string;
}

export interface PreguntaPendiente {
  preguntaId: string;
  entryId: string;
  orden: number;
  texto: string;
  opciones: string[];
  respuestaOpcion: number | null;
  respuestaTexto: string | null;
  resuelta: boolean;
  descartada: boolean;
  createdAt: string;
}

export interface TarjetaBandeja {
  entrada: Entrada;
  preguntas: PreguntaPendiente[];
}

export interface Contexto {
  espacioId: string;
  espacioNombre: string;
  espacioColor: string;
  proyectoId: string;
  proyectoNombre: string;
  origen: 'usuario' | 'aprendido' | 'sesion';
  etiqueta: string;
}

/** Espacios, proyectos y contexto activo leídos de SQLite en un solo viaje. */
export interface DestinosCaptura {
  espacios: Espacio[];
  proyectos: Proyecto[];
  contexto: Contexto | null;
}

export interface Stats {
  capturasHoy: number;
  sinTriage: number;
  latenciaMediaMs: number;
  perdidas: number;
  provisionales: number;
}

export interface CapturaConfirmada {
  entryId: string;
  latenciaMs: number;
}

export interface CapturaOcrResultado {
  entryId: string;
  texto: string;
  sourceApp: string | null;
}

export interface EstadoOnboarding {
  completado: boolean;
  permisoNotificaciones: boolean;
  permisoAccesibilidad: boolean;
  capturaVoz: boolean;
  atajo: string;
}

export interface InfoSistema {
  llmActivo: boolean;
  /** Si el ping falló: sin key, sin red u Ollama caído. */
  llmAviso: string | null;
  llmProveedor: 'ollama' | 'dashscope' | string;
  llmModelo: string;
  ocrDisponible: boolean;
  version: string;
}

export interface EspacioPropuesto {
  nombre: string;
  tipo: TipoEspacio;
  color: string;
  razon: string;
}

export interface TipoPropuesto {
  nombre: string;
  razon: string;
}

export interface PropuestaSetup {
  espacios: EspacioPropuesto[];
  tipos: TipoPropuesto[];
  recomendaciones?: string[];
  fuente: 'llm' | 'fallback' | string;
}

export interface Evento {
  eventoId: string;
  entryId: string | null;
  espacioId: string;
  espacioNombre: string;
  espacioColor: string;
  proyectoId: string | null;
  proyectoNombre: string | null;
  titulo: string;
  descripcion: string | null;
  inicioAt: string;
  finAt: string;
  allDay: boolean;
  ubicacion: string | null;
  rrule: string | null;
  color: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface EstadoPermisos {
  accesibilidad: boolean;
  notificaciones: boolean;
  microfono: boolean;
}

export interface FinalizarOnboardingResultado {
  espaciosCreados: number;
  tiposCreados: number;
}
