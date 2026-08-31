-- Lunmia Memory — esquema SQLite (PRD MVP1 §5)
-- Convenciones:
--   * UUIDv7 en TEXT como PK en todas las tablas (§5.6)
--   * updated_at por fila (§5.6)
--   * Borrado suave con deleted_at (§5.6)
--   * Timestamps en ISO-8601 UTC

PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;

-- ────────────────────────────────────────────────────────────────────
-- ESPACIOS
CREATE TABLE IF NOT EXISTS espacios (
  espacio_id TEXT PRIMARY KEY,
  nombre TEXT NOT NULL,
  tipo TEXT NOT NULL CHECK (tipo IN ('trabajo', 'personal', 'cliente', 'estudio')),
  color TEXT NOT NULL,
  fijado INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_espacios_deleted ON espacios(deleted_at);

-- ────────────────────────────────────────────────────────────────────
-- CALENDARIOS (1 por espacio, §5.2)
CREATE TABLE IF NOT EXISTS calendarios (
  calendario_id TEXT PRIMARY KEY,
  espacio_id TEXT NOT NULL UNIQUE REFERENCES espacios(espacio_id),
  nombre TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);

-- ────────────────────────────────────────────────────────────────────
-- PROYECTOS (por defecto no borrable, §5.2)
CREATE TABLE IF NOT EXISTS proyectos (
  proyecto_id TEXT PRIMARY KEY,
  espacio_id TEXT NOT NULL REFERENCES espacios(espacio_id),
  nombre TEXT NOT NULL,
  es_por_defecto INTEGER NOT NULL DEFAULT 0,
  fijado INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_proyectos_espacio ON proyectos(espacio_id, deleted_at);

-- ────────────────────────────────────────────────────────────────────
-- TIPOS (taxonomía, §5.4)
CREATE TABLE IF NOT EXISTS tipos (
  tipo_id TEXT PRIMARY KEY,
  nombre TEXT NOT NULL UNIQUE,
  descripcion TEXT,
  centroide_embedding BLOB,
  centroide_norm REAL,
  ultima_uso_at TEXT,
  usos_total INTEGER NOT NULL DEFAULT 0,
  archivado INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_tipos_archivado ON tipos(archivado, deleted_at);

-- ────────────────────────────────────────────────────────────────────
-- ENTRADAS (§5.3)
CREATE TABLE IF NOT EXISTS entradas (
  entry_id TEXT PRIMARY KEY,
  contenido_original TEXT NOT NULL,
  contenido TEXT NOT NULL,
  tipo_id TEXT REFERENCES tipos(tipo_id),
  espacio_id TEXT NOT NULL REFERENCES espacios(espacio_id),
  proyecto_id TEXT NOT NULL REFERENCES proyectos(proyecto_id),
  es_provisional INTEGER NOT NULL DEFAULT 0,
  estado TEXT NOT NULL DEFAULT 'inbox'
    CHECK (estado IN ('inbox', 'pendiente_resolucion', 'triaged', 'activo', 'archivado')),
  origen TEXT NOT NULL
    CHECK (origen IN ('texto', 'voz', 'captura_pantalla', 'telegram')),
  metadata_captura TEXT,           -- JSON
  confianza REAL,
  confianza_capa INTEGER,          -- 0..3
  embedding BLOB,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_entradas_estado ON entradas(estado, deleted_at, created_at);
CREATE INDEX IF NOT EXISTS idx_entradas_espacio_proyecto ON entradas(espacio_id, proyecto_id, deleted_at);
CREATE INDEX IF NOT EXISTS idx_entradas_provisional ON entradas(es_provisional, deleted_at);

-- ────────────────────────────────────────────────────────────────────
-- ETIQUETAS (libres, §5.4)
CREATE TABLE IF NOT EXISTS etiquetas (
  etiqueta_id TEXT PRIMARY KEY,
  nombre TEXT NOT NULL UNIQUE,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);

CREATE TABLE IF NOT EXISTS entrada_etiquetas (
  entry_id TEXT NOT NULL REFERENCES entradas(entry_id) ON DELETE CASCADE,
  etiqueta_id TEXT NOT NULL REFERENCES etiquetas(etiqueta_id) ON DELETE CASCADE,
  created_at TEXT NOT NULL,
  PRIMARY KEY (entry_id, etiqueta_id)
);

-- ────────────────────────────────────────────────────────────────────
-- RECORDATORIOS (dispara con la app cerrada — R1, §13)
CREATE TABLE IF NOT EXISTS recordatorios (
  recordatorio_id TEXT PRIMARY KEY,
  entry_id TEXT REFERENCES entradas(entry_id) ON DELETE CASCADE,
  titulo TEXT NOT NULL,
  cuerpo TEXT,
  dispara_at TEXT NOT NULL,            -- ISO-8601 UTC
  disparado_at TEXT,
  cancelado_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_recordatorios_dispara ON recordatorios(disparado_at, dispara_at)
  WHERE disparado_at IS NULL AND cancelado_at IS NULL AND deleted_at IS NULL;

-- ────────────────────────────────────────────────────────────────────
-- PREGUNTAS PENDIENTES (bandeja, §6.6)
CREATE TABLE IF NOT EXISTS preguntas_pendientes (
  pregunta_id TEXT PRIMARY KEY,
  entry_id TEXT NOT NULL REFERENCES entradas(entry_id) ON DELETE CASCADE,
  orden INTEGER NOT NULL,                       -- 0..1 (máximo 2 por entrada)
  texto TEXT NOT NULL,
  opciones TEXT NOT NULL,                       -- JSON array de strings
  respuesta_opcion INTEGER,
  respuesta_texto TEXT,
  resuelta_at TEXT,
  descartada_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_preguntas_pendientes ON preguntas_pendientes(entry_id)
  WHERE resuelta_at IS NULL AND descartada_at IS NULL;

-- ────────────────────────────────────────────────────────────────────
-- CONTEXTO ACTIVO (§5.5): tabla de frecuencias determinista, NO IA
CREATE TABLE IF NOT EXISTS contexto_frecuencias (
  frecuencia_id TEXT PRIMARY KEY,
  dia_semana INTEGER NOT NULL,           -- 0=Dom..6=Sáb
  franja_hora INTEGER NOT NULL,          -- 0..23
  espacio_id TEXT NOT NULL REFERENCES espacios(espacio_id),
  proyecto_id TEXT NOT NULL REFERENCES proyectos(proyecto_id),
  cuenta INTEGER NOT NULL DEFAULT 1,
  updated_at TEXT NOT NULL,
  UNIQUE (dia_semana, franja_hora, espacio_id, proyecto_id)
);

CREATE TABLE IF NOT EXISTS contexto_sesion (
  sesion_id TEXT PRIMARY KEY,
  espacio_id TEXT NOT NULL REFERENCES espacios(espacio_id),
  proyecto_id TEXT NOT NULL REFERENCES proyectos(proyecto_id),
  origen TEXT NOT NULL CHECK (origen IN ('usuario', 'aprendido')),
  activo INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_contexto_activo ON contexto_sesion(activo, created_at);

-- ────────────────────────────────────────────────────────────────────
-- EVENTOS (calendario, modelo tipo Google/iCal — soporta rangos y all_day)
CREATE TABLE IF NOT EXISTS eventos (
  evento_id TEXT PRIMARY KEY,
  entry_id TEXT REFERENCES entradas(entry_id) ON DELETE SET NULL,
  espacio_id TEXT NOT NULL REFERENCES espacios(espacio_id),
  proyecto_id TEXT REFERENCES proyectos(proyecto_id),
  titulo TEXT NOT NULL,
  descripcion TEXT,
  inicio_at TEXT NOT NULL,                 -- ISO-8601 UTC (inicio del evento)
  fin_at TEXT NOT NULL,                    -- ISO-8601 UTC (fin exclusivo; para all_day = medianoche del día siguiente al último día)
  all_day INTEGER NOT NULL DEFAULT 0,      -- 1 = evento de todo el día (ignora la hora)
  ubicacion TEXT,
  rrule TEXT,                              -- iCal RRULE opcional para recurrencia (FREQ=DAILY;...)
  color TEXT,                              -- opcional; si null se hereda del espacio
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_eventos_rango ON eventos(inicio_at, fin_at, deleted_at);
CREATE INDEX IF NOT EXISTS idx_eventos_espacio ON eventos(espacio_id, deleted_at);
CREATE INDEX IF NOT EXISTS idx_eventos_entry ON eventos(entry_id);

-- ────────────────────────────────────────────────────────────────────
-- COLECCIONES (vistas no destructivas generadas por IA, §5.1)
CREATE TABLE IF NOT EXISTS colecciones (
  coleccion_id TEXT PRIMARY KEY,
  espacio_id TEXT NOT NULL REFERENCES espacios(espacio_id),
  nombre TEXT NOT NULL,
  descripcion TEXT,
  consulta TEXT NOT NULL,                  -- JSON: filtros generadores
  promovida_a_proyecto INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);

-- ────────────────────────────────────────────────────────────────────
-- REFERENCIAS EXTERNAS (§5.7 — vacío en MVP1)
CREATE TABLE IF NOT EXISTS referencias_externas (
  referencia_id TEXT PRIMARY KEY,
  entry_id TEXT REFERENCES entradas(entry_id) ON DELETE CASCADE,
  proyecto_id TEXT REFERENCES proyectos(proyecto_id) ON DELETE CASCADE,
  tipo TEXT NOT NULL CHECK (tipo IN ('url', 'repo', 'ticket', 'archivo')),
  valor TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);

-- ────────────────────────────────────────────────────────────────────
-- INSTRUMENTACIÓN (§12.4)
CREATE TABLE IF NOT EXISTS metricas_operacion (
  metrica_id TEXT PRIMARY KEY,
  operacion TEXT NOT NULL,               -- 'capturar', 'clasificar', 'preguntar', ...
  latencia_ms INTEGER NOT NULL,
  contexto TEXT,                          -- JSON
  created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_metricas_op ON metricas_operacion(operacion, created_at);

CREATE TABLE IF NOT EXISTS metricas_clasificacion (
  metrica_id TEXT PRIMARY KEY,
  entry_id TEXT NOT NULL REFERENCES entradas(entry_id) ON DELETE CASCADE,
  capa INTEGER NOT NULL,
  confianza REAL NOT NULL,
  corregido INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL
);

-- ────────────────────────────────────────────────────────────────────
-- CONFIGURACIÓN
CREATE TABLE IF NOT EXISTS config (
  clave TEXT PRIMARY KEY,
  valor TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- Semilla mínima: config por defecto
INSERT OR IGNORE INTO config (clave, valor, updated_at) VALUES
  ('atajo_captura', 'CmdOrCtrl+Shift+Space', datetime('now')),
  ('presupuesto_preguntas_diario', '10', datetime('now')),
  ('umbral_confianza_pregunta', '0.65', datetime('now')),
  ('perfil_modelo', 'equilibrado', datetime('now')),
  ('onboarding_completado', '0', datetime('now')),
  ('permiso_notificaciones', '0', datetime('now')),
  ('permiso_accesibilidad', '0', datetime('now')),
  ('permiso_microfono', '0', datetime('now')),
  ('captura_voz_habilitada', '0', datetime('now')),
  ('autostart_habilitado', '0', datetime('now'));
