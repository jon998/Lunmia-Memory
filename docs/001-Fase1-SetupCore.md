# Fase 1: Setup y Core

**Duración Estimada:** 2 semanas  
**Objetivo:** Proyecto Tauri configurado, SQLite integrado, Dashboard básico funcional, Ollama integrado

---

## Tarea 1.1: Configuración Inicial del Proyecto Tauri

**Estado:** `[ ] Pendiente`

**Descripción:**
Configurar el proyecto base de Tauri 2 con SvelteKit y todas las dependencias necesarias para el desarrollo.

**Criterios de Aceptación:**
- [ ] Proyecto Tauri 2 inicializado correctamente
- [ ] SvelteKit 2 + Svelte 5 configurado
- [ ] TypeScript configurado con strict mode
- [ ] Estructura de carpetas definida según arquitectura
- [ ] Scripts de desarrollo funcionando (`pnpm tauri dev`)
- [ ] Build de producción genera ejecutable

**Archivos Afectados:**
- `package.json`
- `svelte.config.js`
- `vite.config.ts`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

**Decisiones Requeridas:**
- [ ] Confirmar estructura de carpetas del proyecto
- [ ] Confirmar configuración de TypeScript (strict vs standard)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.2: Configuración de SQLite con rusqlite

**Estado:** `[ ] Pendiente`

**Descripción:**
Integrar SQLite como base de datos local usando rusqlite en el backend de Rust.

**Criterios de Aceptación:**
- [ ] rusqlite añadido como dependencia en Cargo.toml
- [ ] Módulo de conexión a DB implementado
- [ ] Sistema de migraciones configurado
- [ ] Esquema inicial de base de datos creado (projects, tasks, captures, embeddings, config)
- [ ] Tests unitarios para operaciones CRUD básicas
- [ ] DB se crea automáticamente en primer inicio

**Archivos Afectados:**
- `src-tauri/Cargo.toml`
- `src-tauri/src/db/mod.rs`
- `src-tauri/src/db/migrations/`
- `src-tauri/src/db/models.rs`

**Decisiones Requeridas:**
- [ ] Ubicación del archivo de base de datos (app_data vs custom)
- [ ] Estrategia de migraciones (embedded vs archivos externos)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.3: Esquema de Base de Datos

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar el esquema completo de base de datos según el PRD.

**Criterios de Aceptación:**
- [ ] Tabla `projects` creada con todos los campos
- [ ] Tabla `tasks` creada con relación a projects
- [ ] Tabla `captures` creada para metadata de capturas
- [ ] Tabla `embeddings` creada para búsqueda semántica
- [ ] Tabla `config` creada para configuración
- [ ] Índices creados para campos frecuentemente consultados
- [ ] Foreign keys configuradas correctamente

**Esquema SQL Requerido:**
```sql
-- projects
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    tech_stack JSON,
    created_at INTEGER NOT NULL,
    last_activity INTEGER NOT NULL,
    is_active BOOLEAN DEFAULT 1
);

-- tasks
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT,
    status TEXT,
    type TEXT,
    source TEXT,
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- captures
CREATE TABLE captures (
    id TEXT PRIMARY KEY,
    project_id TEXT,
    source TEXT NOT NULL,
    captured_text TEXT,
    metadata JSON,
    created_at INTEGER NOT NULL,
    processed BOOLEAN DEFAULT 0,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- embeddings
CREATE TABLE embeddings (
    id TEXT PRIMARY KEY,
    task_id TEXT,
    capture_id TEXT,
    embedding BLOB NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id),
    FOREIGN KEY (capture_id) REFERENCES captures(id)
);

-- config
CREATE TABLE config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

**Archivos Afectados:**
- `src-tauri/src/db/migrations/001_initial_schema.sql`
- `src-tauri/src/db/models.rs`

**Decisiones Requeridas:**
- [ ] Confirmar tipos de datos para campos JSON (TEXT vs nativo)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.4: Comandos Tauri para CRUD de Proyectos

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar comandos Tauri para operaciones CRUD de proyectos accesibles desde el frontend.

**Criterios de Aceptación:**
- [ ] Comando `create_project` implementado
- [ ] Comando `get_projects` implementado
- [ ] Comando `get_project_by_id` implementado
- [ ] Comando `update_project` implementado
- [ ] Comando `delete_project` implementado
- [ ] Manejo de errores con tipos Result apropiados
- [ ] Serialización/deserialización con serde

**Archivos Afectados:**
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/projects.rs`
- `src-tauri/src/main.rs` (registro de comandos)

**Decisiones Requeridas:**
- [ ] Formato de respuesta de errores (JSON structure)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.5: Dashboard Básico - Layout Principal

**Estado:** `[ ] Pendiente`

**Descripción:**
Crear el layout principal del dashboard según el diseño UI/UX proporcionado.

**Criterios de Aceptación:**
- [ ] Layout base con header, content area, bottom navigation
- [ ] Header con logo, badge "LOCAL MODE", iconos de settings/notificaciones
- [ ] Bottom navigation con 4 tabs: Dash, Projects, Memory, Settings
- [ ] Dark theme implementado con paleta de colores del PRD
- [ ] Responsive (mínimo 800x600)
- [ ] Transiciones suaves entre vistas

**Paleta de Colores:**
- Background: #0A0A0F
- Cards: #1A1A24
- Accent: #6366F1
- Success: #10B981
- Text: #E5E7EB
- Muted: #6B7280

**Archivos Afectados:**
- `src/routes/+layout.svelte`
- `src/lib/components/Header.svelte`
- `src/lib/components/BottomNav.svelte`
- `src/app.css` (variables globales)

**Decisiones Requeridas:**
- [ ] Librería de iconos a utilizar (Lucide, Heroicons, etc.)
- [ ] Sistema de routing (file-based vs programmatic)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.6: Dashboard - Cards de Estado

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar las cards de estado que muestran información de Ollama y modelo activo.

**Criterios de Aceptación:**
- [ ] Card "Ollama Status" con indicador de conexión
- [ ] Card "Active Model" con nombre y tamaño del modelo
- [ ] Estados visuales: connected (verde), disconnected (rojo), loading (amarillo)
- [ ] Actualización en tiempo real del estado
- [ ] Diseño acorde al mockup proporcionado

**Archivos Afectados:**
- `src/lib/components/StatusCard.svelte`
- `src/lib/components/OllamaStatus.svelte`
- `src/routes/+page.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.7: Integración con Ollama - Verificación de Conexión

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar la verificación de conexión con Ollama y detección de modelos disponibles.

**Criterios de Aceptación:**
- [ ] Función para verificar si Ollama está corriendo
- [ ] Función para listar modelos disponibles
- [ ] Función para obtener información del modelo activo
- [ ] Comando Tauri `check_ollama_status`
- [ ] Comando Tauri `list_ollama_models`
- [ ] Manejo de errores cuando Ollama no está disponible

**Archivos Afectados:**
- `src-tauri/src/services/ollama.rs`
- `src-tauri/src/commands/ollama.rs`

**Decisiones Requeridas:**
- [ ] Puerto default de Ollama (11434 estándar)
- [ ] Timeout para verificación de conexión

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.8: Dashboard - Project Insights

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar la sección de Project Insights con gráfico de barras de lenguajes.

**Criterios de Aceptación:**
- [ ] Gráfico de barras horizontal mostrando % por lenguaje
- [ ] Colores distintos por lenguaje (TypeScript, Rust, Go, etc.)
- [ ] Stats: tokens indexed, files analyzed
- [ ] Datos se cargan desde backend
- [ ] Animación de carga del gráfico

**Archivos Afectados:**
- `src/lib/components/ProjectInsights.svelte`
- `src/lib/components/LanguageBar.svelte`
- `src/routes/+page.svelte`

**Decisiones Requeridas:**
- [ ] Librería de gráficos (Chart.js, custom SVG, etc.)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.9: Dashboard - Git Worktrees

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar la sección de Git Worktrees mostrando branches activas.

**Criterios de Aceptación:**
- [ ] Lista de worktrees/branches detectadas
- [ ] Badge "ACTIVE" para branch actual
- [ ] Path de cada worktree visible
- [ ] Comando Tauri para obtener worktrees de un proyecto
- [ ] Integración con libgit2 o comandos git

**Archivos Afectados:**
- `src/lib/components/GitWorktrees.svelte`
- `src-tauri/src/services/git.rs`
- `src-tauri/src/commands/git.rs`

**Decisiones Requeridas:**
- [ ] Usar libgit2-rs vs comandos git shell

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.10: Vista de Projects

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar la vista básica de listado de proyectos.

**Criterios de Aceptación:**
- [ ] Lista de proyectos con cards
- [ ] Cada card muestra: nombre, path, tech stack, última actividad
- [ ] Click en proyecto navega a detalle (placeholder)
- [ ] Botón para agregar proyecto manualmente
- [ ] Estado vacío cuando no hay proyectos

**Archivos Afectados:**
- `src/routes/projects/+page.svelte`
- `src/lib/components/ProjectCard.svelte`
- `src/lib/components/EmptyState.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.11: Vista de Settings (Básica)

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar la vista básica de configuración.

**Criterios de Aceptación:**
- [ ] Sección "General" con información de la app
- [ ] Sección "Ollama" con estado y configuración
- [ ] Toggle para modo privado (placeholder funcional)
- [ ] Información de versión de la app
- [ ] Link a documentación/ayuda

**Archivos Afectados:**
- `src/routes/settings/+page.svelte`
- `src/lib/components/SettingsSection.svelte`
- `src/lib/components/Toggle.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 1.12: Vista de Memory (Placeholder)

**Estado:** `[ ] Pendiente`

**Descripción:**
Crear placeholder para la vista de Memory/Búsqueda.

**Criterios de Aceptación:**
- [ ] Barra de búsqueda visible (sin funcionalidad)
- [ ] Mensaje de "Próximamente" o estado vacío
- [ ] Diseño consistente con el resto de la app

**Archivos Afectados:**
- `src/routes/memory/+page.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Resumen de Fase 1

| Tarea | Estado | Code Review |
|-------|--------|-------------|
| 1.1 Configuración Inicial Tauri | [ ] | [ ] |
| 1.2 SQLite con rusqlite | [ ] | [ ] |
| 1.3 Esquema de Base de Datos | [ ] | [ ] |
| 1.4 Comandos CRUD Proyectos | [ ] | [ ] |
| 1.5 Dashboard Layout | [ ] | [ ] |
| 1.6 Cards de Estado | [ ] | [ ] |
| 1.7 Integración Ollama | [ ] | [ ] |
| 1.8 Project Insights | [ ] | [ ] |
| 1.9 Git Worktrees | [ ] | [ ] |
| 1.10 Vista Projects | [ ] | [ ] |
| 1.11 Vista Settings | [ ] | [ ] |
| 1.12 Vista Memory Placeholder | [ ] | [ ] |

**Progreso:** 0/12 tareas completadas
