// Adaptador de datos en memoria — permite desarrollar la UI en el navegador
// sin ejecutar Tauri. NO se usa en producción (isTauri() lo filtra).

import type {
  CapturaConfirmada,
  Contexto,
  Entrada,
  Espacio,
  Evento,
  InfoSistema,
  Proyecto,
  Stats,
  TarjetaBandeja,
  Tipo
} from './types';

const espacios: Espacio[] = [
  { espacioId: 'e1', nombre: 'Trabajo', tipo: 'trabajo', color: '#4F46C9', fijado: true, createdAt: '', updatedAt: '' },
  { espacioId: 'e2', nombre: 'Personal', tipo: 'personal', color: '#C08A3E', fijado: false, createdAt: '', updatedAt: '' },
  { espacioId: 'e3', nombre: 'Estudio', tipo: 'estudio', color: '#4C8C68', fijado: false, createdAt: '', updatedAt: '' }
];

const proyectos: Proyecto[] = [
  { proyectoId: 'p1', espacioId: 'e1', nombre: 'General', esPorDefecto: true, fijado: false, createdAt: '', updatedAt: '' },
  { proyectoId: 'p2', espacioId: 'e1', nombre: 'Lunmia App', esPorDefecto: false, fijado: true, createdAt: '', updatedAt: '' },
  { proyectoId: 'p3', espacioId: 'e2', nombre: 'General', esPorDefecto: true, fijado: false, createdAt: '', updatedAt: '' },
  { proyectoId: 'p4', espacioId: 'e3', nombre: 'General', esPorDefecto: true, fijado: false, createdAt: '', updatedAt: '' },
  { proyectoId: 'p5', espacioId: 'e3', nombre: 'ML nocturno', esPorDefecto: false, fijado: false, createdAt: '', updatedAt: '' }
];

const tipos: Tipo[] = [
  { tipoId: 't-bug', nombre: 'bug', descripcion: null, usosTotal: 4, archivado: false, ultimaUsoAt: null, createdAt: '', updatedAt: '' },
  { tipoId: 't-script', nombre: 'script', descripcion: null, usosTotal: 2, archivado: false, ultimaUsoAt: null, createdAt: '', updatedAt: '' },
  { tipoId: 't2', nombre: 'idea', descripcion: null, usosTotal: 8, archivado: false, ultimaUsoAt: null, createdAt: '', updatedAt: '' },
  { tipoId: 't5', nombre: 'nota', descripcion: null, usosTotal: 6, archivado: false, ultimaUsoAt: null, createdAt: '', updatedAt: '' },
  { tipoId: 't4', nombre: 'recordatorio', descripcion: null, usosTotal: 5, archivado: false, ultimaUsoAt: null, createdAt: '', updatedAt: '' },
  { tipoId: 't1', nombre: 'tarea', descripcion: null, usosTotal: 3, archivado: false, ultimaUsoAt: null, createdAt: '', updatedAt: '' }
];

const iso = (offsetMin: number) => new Date(Date.now() - offsetMin * 60_000).toISOString();
let zonaMock: string | null = null;

function isoAllDay(day: number): string {
  const n = new Date();
  const m = String(n.getMonth() + 1).padStart(2, '0');
  const d = String(day).padStart(2, '0');
  return `${n.getFullYear()}-${m}-${d}T00:00:00.000Z`;
}

function isoHora(day: number, hour: number, minute = 0): string {
  const n = new Date();
  const d = new Date(n.getFullYear(), n.getMonth(), day, hour, minute, 0);
  return d.toISOString();
}

const entradas: Entrada[] = [
  {
    entryId: 'x1', contenido: 'Enviar la propuesta de presupuesto a Elena antes del viernes',
    contenidoOriginal: 'Enviar la propuesta de presupuesto a Elena antes del viernes',
    tipoId: 't1', tipoNombre: 'tarea',
    espacioId: 'e1', espacioNombre: 'Trabajo', espacioColor: '#4F46C9',
    proyectoId: 'p2', proyectoNombre: 'Lunmia App',
    esProvisional: false, estado: 'activo', origen: 'texto',
    metadataCaptura: null, confianza: 0.87, confianzaCapa: 2, etiquetas: ['propuesta', 'urgente'],
    createdAt: iso(20), updatedAt: iso(20)
  },
  {
    entryId: 'x2', contenido: 'Idea: que el contexto activo se proponga solo según la hora',
    contenidoOriginal: 'Idea: que el contexto activo se proponga solo según la hora',
    tipoId: 't2', tipoNombre: 'idea',
    espacioId: 'e1', espacioNombre: 'Trabajo', espacioColor: '#4F46C9',
    proyectoId: 'p2', proyectoNombre: 'Lunmia App',
    esProvisional: false, estado: 'activo', origen: 'texto',
    metadataCaptura: null, confianza: 0.92, confianzaCapa: 1, etiquetas: ['ux'],
    createdAt: iso(60), updatedAt: iso(60)
  },
  {
    entryId: 'x3', contenido: 'Comprar el libro que recomendó Marcos sobre sistemas',
    contenidoOriginal: 'Comprar el libro que recomendó Marcos sobre sistemas',
    tipoId: 't3', tipoNombre: 'recomendación',
    espacioId: 'e2', espacioNombre: 'Personal', espacioColor: '#C08A3E',
    proyectoId: 'p3', proyectoNombre: 'General',
    esProvisional: false, estado: 'activo', origen: 'voz',
    metadataCaptura: null, confianza: 0.75, confianzaCapa: 2, etiquetas: ['libros'],
    createdAt: iso(120), updatedAt: iso(120)
  },
  {
    entryId: 'x4', contenido: 'Reunión de seguimiento — miércoles 10:00, sala 3',
    contenidoOriginal: 'Reunión de seguimiento — miércoles 10:00, sala 3\nTrae los números del sprint',
    tipoId: null, tipoNombre: null,
    espacioId: 'e1', espacioNombre: 'Trabajo', espacioColor: '#4F46C9',
    proyectoId: 'p1', proyectoNombre: 'General',
    esProvisional: true, estado: 'pendiente_resolucion', origen: 'captura_pantalla',
    metadataCaptura: null, confianza: 0.41, confianzaCapa: 3, etiquetas: [],
    createdAt: iso(180), updatedAt: iso(180)
  },
  {
    entryId: 'x5', contenido: 'Repasar backpropagation antes del examen del jueves',
    contenidoOriginal: 'Repasar backpropagation antes del examen del jueves',
    tipoId: 't4', tipoNombre: 'recordatorio',
    espacioId: 'e3', espacioNombre: 'Estudio', espacioColor: '#4C8C68',
    proyectoId: 'p5', proyectoNombre: 'ML nocturno',
    esProvisional: false, estado: 'activo', origen: 'texto',
    metadataCaptura: null, confianza: 0.9, confianzaCapa: 1, etiquetas: ['examen'],
    createdAt: iso(1440), updatedAt: iso(1440)
  }
];

const bandeja: TarjetaBandeja[] = [
  {
    entrada: entradas[3],
    preguntas: [
      {
        preguntaId: 'q1', entryId: 'x4', orden: 0,
        texto: '¿Qué miércoles?',
        opciones: ['Este (20 ago)', 'El próximo (27 ago)'],
        respuestaOpcion: null, respuestaTexto: null,
        resuelta: false, descartada: false, createdAt: ''
      },
      {
        preguntaId: 'q2', entryId: 'x4', orden: 1,
        texto: '¿A qué proyecto va?',
        opciones: ['Lunmia App', 'General'],
        respuestaOpcion: null, respuestaTexto: null,
        resuelta: false, descartada: false, createdAt: ''
      }
    ]
  }
];

const contexto: Contexto = {
  espacioId: 'e1', espacioNombre: 'Trabajo', espacioColor: '#4F46C9',
  proyectoId: 'p2', proyectoNombre: 'Lunmia App', origen: 'aprendido',
  etiqueta: 'Trabajo / Lunmia App'
};

const hoy = new Date().getDate();
const diaRango = Math.min(Math.max(hoy, 8), 24);

const eventos: Evento[] = [
  {
    eventoId: 'ev-rango',
    entryId: 'x1',
    espacioId: 'e1', espacioNombre: 'Trabajo', espacioColor: '#4F46C9',
    proyectoId: 'p2', proyectoNombre: 'Lunmia App',
    titulo: 'Sprint con Elena',
    descripcion: 'Enviar la propuesta de presupuesto a Elena antes del viernes',
    inicioAt: isoAllDay(diaRango),
    finAt: isoAllDay(diaRango + 4),
    allDay: true,
    ubicacion: null, rrule: null, color: null,
    createdAt: iso(20), updatedAt: iso(20)
  },
  {
    eventoId: 'ev-ml',
    entryId: 'x5',
    espacioId: 'e3', espacioNombre: 'Estudio', espacioColor: '#4C8C68',
    proyectoId: 'p5', proyectoNombre: 'ML nocturno',
    titulo: 'Repaso backprop',
    descripcion: 'Repasar backpropagation antes del examen del jueves',
    inicioAt: isoHora(Math.min(diaRango + 1, 28), 18, 0),
    finAt: isoHora(Math.min(diaRango + 1, 28), 19, 0),
    allDay: false,
    ubicacion: null, rrule: null, color: null,
    createdAt: iso(1440), updatedAt: iso(1440)
  },
  {
    eventoId: 'ev-pasaporte',
    entryId: 'x3',
    espacioId: 'e2', espacioNombre: 'Personal', espacioColor: '#C08A3E',
    proyectoId: 'p3', proyectoNombre: 'General',
    titulo: 'Comprar el libro de Marcos',
    descripcion: null,
    inicioAt: isoAllDay(Math.min(diaRango + 6, 28)),
    finAt: isoAllDay(Math.min(diaRango + 7, 29)),
    allDay: true,
    ubicacion: null, rrule: null, color: null,
    createdAt: iso(120), updatedAt: iso(120)
  }
];

export async function mockAdapter<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  await new Promise((r) => setTimeout(r, 40));
  switch (cmd) {
    case 'listar_espacios': return espacios as unknown as T;
    case 'listar_proyectos': {
      const p = (args?.params as { espacioId: string | null } | undefined)?.espacioId;
      return (p ? proyectos.filter((x) => x.espacioId === p) : proyectos) as unknown as T;
    }
    case 'obtener_contexto_activo': return contexto as unknown as T;
    case 'destinos_captura':
      return { espacios, proyectos, contexto } as unknown as T;
    case 'fijar_contexto': return undefined as unknown as T;
    case 'listar_entradas': return entradas as unknown as T;
    case 'listar_tipos': return tipos as unknown as T;
    case 'crear_tipo': {
      const p = args?.params as { nombre: string };
      const nombre = (p.nombre ?? '').trim().toLowerCase();
      const existente = tipos.find((t) => t.nombre === nombre);
      if (existente) return existente as unknown as T;
      const nuevo: Tipo = {
        tipoId: `t-${Math.random().toString(36).slice(2, 8)}`,
        nombre,
        descripcion: null,
        usosTotal: 0,
        archivado: false,
        ultimaUsoAt: null,
        createdAt: iso(0),
        updatedAt: iso(0)
      };
      tipos.push(nuevo);
      return nuevo as unknown as T;
    }
    case 'obtener_entrada': {
      const id = args?.entryId as string;
      return (entradas.find((e) => e.entryId === id) ?? null) as unknown as T;
    }
    case 'listar_bandeja': return bandeja as unknown as T;
    case 'obtener_stats':
      return {
        capturasHoy: entradas.filter((e) => e.createdAt > iso(1440)).length,
        sinTriage: bandeja.length + 3,
        latenciaMediaMs: 1800,
        perdidas: 0,
        provisionales: entradas.filter((e) => e.esProvisional).length
      } as unknown as T;
    case 'capturar_texto': {
      const p = args?.params as { contenido: string; tipoNombre?: string | null };
      const tipo = p.tipoNombre?.trim() || null;
      const tipoRow = tipo ? tipos.find((t) => t.nombre === tipo) : null;
      const nueva: Entrada = {
        entryId: `n${Math.random().toString(36).slice(2, 8)}`,
        contenido: p.contenido, contenidoOriginal: p.contenido,
        tipoId: tipoRow?.tipoId ?? (tipo ? `t-${tipo}` : null),
        tipoNombre: tipo,
        espacioId: contexto.espacioId, espacioNombre: contexto.espacioNombre, espacioColor: contexto.espacioColor,
        proyectoId: contexto.proyectoId, proyectoNombre: contexto.proyectoNombre,
        esProvisional: false, estado: tipo ? 'activo' : 'inbox', origen: 'texto',
        metadataCaptura: null, confianza: tipo ? 1 : null, confianzaCapa: tipo ? 0 : null, etiquetas: [],
        createdAt: iso(0), updatedAt: iso(0)
      };
      entradas.unshift(nueva);
      return { entryId: nueva.entryId, latenciaMs: 40 } as unknown as CapturaConfirmada as unknown as T;
    }
    case 'estado_ocr': return { disponible: false } as unknown as T;
    case 'estado_onboarding':
      return {
        completado: false, permisoNotificaciones: false,
        permisoAccesibilidad: false, capturaVoz: false,
        atajo: 'CmdOrCtrl+Shift+Space'
      } as unknown as T;
    case 'proponer_setup_inicial':
      return {
        espacios: [
          { nombre: 'Trabajo', tipo: 'trabajo', color: '#4F46C9', razon: 'Todo lo del trabajo principal.' },
          { nombre: 'Personal', tipo: 'personal', color: '#C08A3E', razon: 'La vida fuera del trabajo.' },
          { nombre: 'Estudio', tipo: 'estudio', color: '#4C8C68', razon: 'Aprender por las noches.' }
        ],
        tipos: [
          { nombre: 'idea', razon: 'Chispazos por atrapar.' },
          { nombre: 'nota', razon: 'Fragmentos y contexto.' },
          { nombre: 'recordatorio', razon: 'Cosas con hora o fecha.' },
          { nombre: 'bug', razon: 'Fallos para no perder de vista.' },
          { nombre: 'script', razon: 'Trozos de código o automatizaciones.' }
        ],
        recomendaciones: [
          '⌘⇧Space captura desde cualquier app sin robar foco.',
          'Los espacios se ordenan por color: elígelos reconocibles al vuelo.',
          'Todo lo dudoso va a Bandeja para revisión rápida.'
        ],
        fuente: 'fallback'
      } as unknown as T;
    case 'verificar_permisos':
      return { accesibilidad: false, notificaciones: false, microfono: false } as unknown as T;
    case 'solicitar_permiso_notificaciones':
    case 'solicitar_permiso_accesibilidad':
    case 'solicitar_permiso_microfono':
      return true as unknown as T;
    case 'ruta_log_llm':
      return '/tmp/llm-mock.jsonl' as unknown as T;
    case 'fijar_autostart':
    case 'abrir_log_llm':
    case 'eliminar_espacio':
    case 'eliminar_proyecto':
    case 'renombrar_espacio':
    case 'renombrar_proyecto':
    case 'fijar_espacio':
    case 'fijar_proyecto':
    case 'cambiar_color_espacio':
      return undefined as unknown as T;
    case 'mover_entrada': {
      const p = args?.params as { entryId: string; espacioId: string; proyectoId: string };
      const idx = entradas.findIndex((x) => x.entryId === p.entryId);
      if (idx >= 0) {
        const esp = espacios.find((x) => x.espacioId === p.espacioId);
        const proy = proyectos.find((x) => x.proyectoId === p.proyectoId);
        if (esp && proy) {
          entradas[idx] = {
            ...entradas[idx],
            espacioId: p.espacioId,
            espacioNombre: esp.nombre,
            espacioColor: esp.color,
            proyectoId: p.proyectoId,
            proyectoNombre: proy.nombre
          };
        }
      }
      return undefined as unknown as T;
    }
    case 'crear_proyecto': {
      const p = args?.params as { espacioId: string; nombre: string };
      const nuevo: Proyecto = {
        proyectoId: `p${Math.random().toString(36).slice(2, 6)}`,
        espacioId: p.espacioId,
        nombre: p.nombre,
        esPorDefecto: false,
        fijado: false,
        createdAt: iso(0),
        updatedAt: iso(0)
      };
      proyectos.push(nuevo);
      return nuevo as unknown as T;
    }
    case 'finalizar_onboarding': {
      const p = args?.params as { zonaHoraria?: string } | undefined;
      if (p?.zonaHoraria) zonaMock = p.zonaHoraria;
      return { espaciosCreados: 0, tiposCreados: 0 } as unknown as T;
    }
    case 'get_config': {
      const clave = args?.clave as string;
      if (clave === 'zona_horaria') return (zonaMock ?? 'America/Mexico_City') as unknown as T;
      return null as unknown as T;
    }
    case 'set_config': {
      const clave = args?.clave as string;
      const valor = args?.valor as string;
      if (clave === 'zona_horaria') zonaMock = valor;
      return undefined as unknown as T;
    }
    case 'info_sistema':
      return {
        llmActivo: false,
        llmAviso: 'Sin conexión con la IA. La nota se guardará tal cual.',
        llmProveedor: 'ollama',
        llmModelo: 'qwen2.5:3b',
        ocrDisponible: false,
        version: '0.1.0'
      } as unknown as InfoSistema as unknown as T;
    case 'listar_eventos': {
      const p = (args?.params as { desde: string; hasta: string; espacioId: string | null } | undefined);
      let lista = eventos;
      if (p?.espacioId) lista = lista.filter((e) => e.espacioId === p.espacioId);
      if (p?.desde && p?.hasta) {
        lista = lista.filter((e) => e.inicioAt < p.hasta && e.finAt > p.desde);
      }
      return lista as unknown as T;
    }
    case 'obtener_evento': {
      const id = args?.eventoId as string;
      return (eventos.find((e) => e.eventoId === id) ?? null) as unknown as T;
    }
    case 'obtener_evento_por_entrada': {
      const id = args?.entryId as string;
      return (eventos.find((e) => e.entryId === id) ?? null) as unknown as T;
    }
    case 'crear_evento': {
      const p = args?.params as {
        entryId?: string | null;
        espacioId: string;
        proyectoId?: string | null;
        titulo: string;
        descripcion?: string | null;
        inicioAt: string;
        finAt: string;
        allDay: boolean;
      };
      const esp = espacios.find((x) => x.espacioId === p.espacioId);
      const proy = p.proyectoId ? proyectos.find((x) => x.proyectoId === p.proyectoId) : undefined;
      const nuevo: Evento = {
        eventoId: `ev${Math.random().toString(36).slice(2, 8)}`,
        entryId: p.entryId ?? null,
        espacioId: p.espacioId,
        espacioNombre: esp?.nombre ?? '',
        espacioColor: esp?.color ?? '#4F46C9',
        proyectoId: p.proyectoId ?? null,
        proyectoNombre: proy?.nombre ?? null,
        titulo: p.titulo,
        descripcion: p.descripcion ?? null,
        inicioAt: p.inicioAt,
        finAt: p.finAt,
        allDay: p.allDay,
        ubicacion: null,
        rrule: null,
        color: null,
        createdAt: iso(0),
        updatedAt: iso(0)
      };
      eventos.push(nuevo);
      return nuevo.eventoId as unknown as T;
    }
    case 'actualizar_evento': {
      const p = args?.params as {
        eventoId: string;
        titulo: string;
        descripcion?: string | null;
        inicioAt: string;
        finAt: string;
        allDay: boolean;
        espacioId: string;
        proyectoId?: string | null;
      };
      const i = eventos.findIndex((e) => e.eventoId === p.eventoId);
      if (i >= 0) {
        const esp = espacios.find((x) => x.espacioId === p.espacioId);
        const proy = p.proyectoId ? proyectos.find((x) => x.proyectoId === p.proyectoId) : undefined;
        eventos[i] = {
          ...eventos[i],
          titulo: p.titulo,
          descripcion: p.descripcion ?? eventos[i].descripcion,
          inicioAt: p.inicioAt,
          finAt: p.finAt,
          allDay: p.allDay,
          espacioId: p.espacioId,
          espacioNombre: esp?.nombre ?? eventos[i].espacioNombre,
          espacioColor: esp?.color ?? eventos[i].espacioColor,
          proyectoId: p.proyectoId ?? null,
          proyectoNombre: proy?.nombre ?? null,
          updatedAt: iso(0)
        };
      }
      return undefined as unknown as T;
    }
    case 'eliminar_evento': {
      const id = args?.eventoId as string;
      const i = eventos.findIndex((e) => e.eventoId === id);
      if (i >= 0) eventos.splice(i, 1);
      return undefined as unknown as T;
    }
    case 'eliminar_entrada': {
      const id = args?.entryId as string;
      const i = entradas.findIndex((e) => e.entryId === id);
      if (i >= 0) entradas.splice(i, 1);
      for (let j = eventos.length - 1; j >= 0; j--) {
        if (eventos[j].entryId === id) eventos.splice(j, 1);
      }
      return undefined as unknown as T;
    }
    case 'fijar_recordatorio':
      return undefined as unknown as T;
    case 'obtener_recordatorio_por_entrada':
      return null as unknown as T;
    default: {
      console.warn('mock: comando no implementado', cmd, args);
      return undefined as unknown as T;
    }
  }
}
