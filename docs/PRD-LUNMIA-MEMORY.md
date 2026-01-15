# PRD: Lunmia Memory

## Metadata
- **Autor:** Victor Yonatan, Maikol
- **Fecha:** 2026-01-14
- **Versión:** 1.0
- **Estado:** Aprobado
- **Metodología:** PRD Workflow System

---

## 1. Resumen Ejecutivo

**Lunmia Memory** es un gestor inteligente de tareas y proyectos que revoluciona la forma en que desarrolladores y profesionales organizan su trabajo mediante **orquestación automática de IAs locales**. A diferencia de herramientas tradicionales como Notion, Todoist o Linear, Lunmia Memory elimina completamente la necesidad de crear tareas manualmente: un sistema de agentes basados en MCP captura automáticamente información del contexto del usuario (navegadores, IDEs, reuniones) y una IA local procesa, organiza y estructura esta información en tareas, apuntes y recordatorios, todo sin conexión a internet garantizando máxima privacidad.

El producto opera 100% localmente, utilizando modelos de IA optimizados para TPU que procesan capturas de pantalla (mediante OCR), código fuente y actividad del usuario en tiempo real, organizando automáticamente la información en un dashboard intuitivo y moderno con capacidades de búsqueda inteligente.

---

## 2. Contexto y Motivación

### 2.1 Problema

Los desarrolladores y profesionales que manejan múltiples proyectos enfrentan tres problemas críticos:

1. **Sobrecarga cognitiva:** Deben recordar manualmente crear tareas, tomar apuntes y registrar el progreso en cada proyecto, interrumpiendo constantemente su flujo de trabajo.

2. **Pérdida de contexto:** La información valiosa de reuniones, conversaciones en Slack/Teams, cambios en código y decisiones técnicas se pierde porque no hay tiempo o disciplina para documentarla.

3. **Falta de privacidad:** Soluciones existentes en la nube (Notion, Asana, etc.) almacenan información sensible de proyectos, código fuente y datos empresariales en servidores externos.

### 2.2 Usuarios Objetivo

**Usuario Primario:** Desarrolladores de software que trabajan en múltiples proyectos simultáneamente (freelancers, tech leads, full-stack developers).

**Características:**
- Utilizan múltiples herramientas diariamente (VS Code, Cursor, Chrome, Slack, Teams)
- Valoran la privacidad y prefieren soluciones locales
- Tienen hardware moderno con capacidad de procesamiento IA (mínimo 16GB RAM, TPU)
- Buscan optimizar su productividad sin añadir fricción a su flujo de trabajo

**Usuario Secundario (Expansión):** Cualquier profesional que maneje múltiples proyectos (diseñadores, product managers, consultores), aunque el MVP se optimiza para desarrolladores.

### 2.3 Oportunidad

**Por qué ahora:**
- Modelos de IA local (Llama, Qwen, etc.) ahora son lo suficientemente potentes y eficientes para ejecutarse en hardware consumer.
- MCP (Model Context Protocol) permite integración estandarizada con múltiples herramientas.
- Creciente preocupación por privacidad de datos hace viable un producto 100% local.
- Hardware con TPU/NPU se está volviendo estándar en laptops modernas (Apple Silicon, Intel Core Ultra, etc.).

**Diferenciación clave:** Ninguna herramienta actual combina captura automática + procesamiento IA local + privacidad total. Competitors como Rewind.ai requieren internet y almacenan en la nube; herramientas tradicionales requieren entrada manual constante.

---

## 3. Objetivos y Métricas de Éxito

### 3.1 Objetivos del Proyecto

1. **Automatización radical:** Reducir al mínimo la intervención manual del usuario en la gestión de tareas y proyectos.
2. **Privacidad total:** Garantizar que ningún dato salga de la máquina del usuario.
3. **Performance local:** Demostrar que procesamiento IA local es viable y competitivo en velocidad.
4. **Experiencia fluida:** Integrar la captura de información de forma no intrusiva en el flujo de trabajo del usuario.

### 3.2 Criterios de Éxito

| Métrica | Objetivo | Cómo Medir |
|---------|----------|------------|
| **% Tareas Automáticas** | > 80% de tareas creadas automáticamente vs manualmente | Ratio en analytics local |
| **Satisfacción del Usuario** | Score > 8/10 en encontrar información rápidamente | Encuesta in-app post búsqueda |
| **Velocidad de Búsqueda** | < 1 segundo en 95% de búsquedas | Telemetría local de performance |
| **Precisión de Categorización IA** | > 85% de tareas correctamente categorizadas por proyecto | Feedback implícito (usuario no mueve tarea de proyecto) |
| **Adopción de Extensiones** | Usuario conecta al menos 2 de 4 extensiones disponibles | Logs de configuración MCP |

---

## 4. Alcance

### 4.1 En Alcance (MVP)

#### 4.1.1 Dashboard Principal
- Vista unificada de todos los proyectos activos (máximo 20 proyectos simultáneos)
- Estadísticas en tiempo real: porcentaje de código por lenguaje, archivos analizados, tokens indexados
- Indicador de estado de Ollama y modelo IA activo
- Visualización de Git worktrees y ramas activas
- Badge de "Local Mode" siempre visible
- Notificaciones push para tareas críticas/urgentes

#### 4.1.2 Sistema de Agentes MCP
**Extensiones/Integraciones:**
- **Chrome Extension:** Captura actividad en navegador, detecta reuniones en Teams/Slack/Meet
- **VS Code Extension:** Monitorea cambios en código, commits, archivos abiertos
- **Cursor Extension:** Igual que VS Code pero optimizado para Cursor
- **Safari Extension:** Captura actividad en navegador (macOS)

**Funcionalidad de Agentes:**
- MCP Server expone contexto de Lunmia a herramientas externas
- Bridge bidireccional: extensiones envían datos → Lunmia procesa → puede responder a extensiones
- Configuración individual por extensión (enable/disable, permisos)

#### 4.1.3 Gestión Automática de Tareas
- **Captura inteligente:**
  - OCR de capturas de pantalla (solo para extraer texto, imágenes se eliminan inmediatamente)
  - Parsing de código fuente y commits
  - Transcripción de reuniones (si disponible de Slack/Teams APIs)
  
- **Procesamiento IA Local:**
  - Modelo: Qwen2.5-Coder (u otros compatibles con Ollama)
  - Categorización automática de tareas por proyecto
  - Extracción de action items de reuniones
  - Detección de prioridades y deadlines
  - Relación automática entre tareas (dependencias)

- **Detección de Información Sensible:**
  - Algoritmos de pattern matching para detectar: passwords, API keys, tokens, tarjetas de crédito
  - Información sensible NO se almacena, se descarta inmediatamente
  - Logging de cuánta información fue descartada por seguridad (solo contadores, no contenido)

#### 4.1.4 Registro de Reuniones/Apuntes Automático
- Detección de ventanas activas (Teams, Slack, Meet, Zoom)
- Captura de pantalla automática cuando se detecta reunión
- OCR para extraer texto visible (agenda, decisiones, participantes)
- Eliminación de imagen post-OCR
- Generación automática de resumen de reunión por IA
- Vinculación de reunión a proyecto relevante

#### 4.1.5 Búsqueda y Filtrado Inteligente
- **Búsqueda por:**
  - Texto completo (full-text search)
  - Proyecto específico
  - Rango de fechas
  - Tipo de contenido (tarea, reunión, apunte, código)
  - Estado de tarea (pendiente, en progreso, completada)

- **Procesamiento IA:**
  - Búsqueda semántica (entender intención, no solo keywords)
  - Modelo local embebido en Ollama
  - Tiempo de respuesta garantizado < 1 segundo

- **Extensibilidad futura:** Preparar arquitectura para conectar APIs externas (Claude, GPT) sin refactorización mayor

#### 4.1.6 Modo Privado
- Toggle global para pausar completamente toda captura automática
- Activación manual por usuario
- Indicador visual claro cuando está activo
- Sugerencia automática de activar cuando se detectan keywords: "payment", "bank", "credit card", etc.

#### 4.1.7 Onboarding Completo
1. **Pantalla de Bienvenida**
2. **Guía de Configuración MCP:** Paso a paso por cada extensión (Chrome, VS Code, Cursor, Safari)
3. **Selección de Carpeta de Proyectos:** Usuario selecciona directorio raíz donde están sus proyectos
4. **Detección y Personalización de Proyectos:**
   - Escaneo automático de subcarpetas con Git repos
   - Usuario puede renombrar proyectos detectados
   - Opción de crear proyectos manualmente si no se detecta código
5. **Permisos del Sistema:**
   - **macOS:** Screen Recording, Accessibility
   - **Windows:** Administrator (si necesario), Notifications
   - **Linux:** X11 permissions o Wayland equivalents
6. **Tutorial Interactivo:** Tour rápido del dashboard y funciones clave

### 4.2 Fuera de Alcance (MVP 1.0)

- ❌ **Colaboración en equipo:** Compartir proyectos, tareas o dashboards con otros usuarios
- ❌ **Sincronización en la nube:** Backup automático, sync entre dispositivos
- ❌ **Soporte móvil:** Android/iOS apps (planeado para MVP 2.0)
- ❌ **Integraciones externas:** GitHub API, GitLab, Jira, Trello, etc.
- ❌ **Exportación de datos:** Formatos como CSV, JSON, Markdown, PDF
- ❌ **Calendario integrado:** Vista tipo Google Calendar
- ❌ **Time tracking:** Cuánto tiempo se dedica a cada tarea/proyecto
- ❌ **Webhooks/Automation Rules:** Reglas tipo "si X entonces Y"

### 4.3 Fases Futuras

**MVP 2.0 (Post-lanzamiento):**
- Soporte móvil (Android/iOS) con sync opcional
- Exportación de datos
- Integraciones con GitHub/GitLab/Linear

**MVP 3.0:**
- Colaboración en equipo (modo opcional, requiere servidor)
- Time tracking automático
- Analytics avanzados de productividad

---

## 5. Requisitos Funcionales

### 5.1 Gestión de Proyectos

**Descripción:** Sistema para detectar, crear y organizar proyectos.

**Flujo de usuario:**
1. Usuario selecciona carpeta raíz de proyectos en onboarding
2. Lunmia escanea recursivamente buscando Git repos
3. Lista de proyectos detectados se muestra con nombres inferidos (del `package.json`, `Cargo.toml`, nombre de carpeta)
4. Usuario confirma o renombra cada proyecto
5. Proyectos se indexan en base de datos local
6. Usuario puede añadir proyectos manualmente si no tienen Git

**Criterios de aceptación:**
- [ ] Detecta correctamente repos Git en subdirectorios hasta 5 niveles de profundidad
- [ ] Infiere nombres de proyecto de múltiples fuentes (package.json, Cargo.toml, go.mod, etc.)
- [ ] Permite renombrar proyectos antes de confirmar
- [ ] Permite crear proyectos sin Git (manual)
- [ ] Soporta hasta 20 proyectos simultáneos sin degradación de performance
- [ ] Detecta tech stack de cada proyecto (lenguajes principales)

### 5.2 Captura Automática de Contexto

**Descripción:** Sistema de agentes que captura información del usuario en tiempo real.

**Flujo de captura (navegador):**
1. Usuario navega en Chrome/Safari
2. Extensión detecta cambio de pestaña/URL
3. Si URL contiene "slack.com/call", "teams.microsoft.com", "meet.google.com" → trigger evento "reunión detectada"
4. Extensión envía metadata a Lunmia vía MCP (título, URL, timestamp)
5. Lunmia toma screenshot de ventana activa
6. OCR extrae texto
7. IA analiza texto, detecta si hay información sensible
8. Si NO hay info sensible: procesa y crea apunte/tarea
9. Si SÍ hay info sensible: descarta todo
10. Screenshot se elimina en ambos casos

**Flujo de captura (IDE):**
1. Usuario edita código en VS Code/Cursor
2. Extensión monitorea eventos: file_save, git_commit, terminal_command
3. Metadata se envía a Lunmia: nombre archivo, lenguaje, líneas modificadas, mensaje de commit
4. IA relaciona cambio con proyecto activo
5. Si commit message contiene action items ("TODO", "FIXME", "fix bug X") → crea tarea automáticamente
6. Actualiza estadísticas de proyecto (líneas de código, archivos tocados)

**Criterios de aceptación:**
- [ ] Extensión Chrome/Safari detecta 100% de URLs con keywords: slack, teams, meet, zoom
- [ ] Screenshot + OCR + procesamiento toma < 5 segundos en hardware mínimo (16GB RAM, TPU)
- [ ] Detección de información sensible con 99%+ accuracy (basado en regex patterns conocidos)
- [ ] Extensión VS Code/Cursor captura commits en tiempo real sin retrasos perceptibles
- [ ] MCP Server responde a pings de extensiones en < 100ms
- [ ] Modo privado bloquea 100% de capturas cuando está activo

### 5.3 Procesamiento de IA Local

**Descripción:** Motor de IA que transforma información cruda en tareas estructuradas.

**Modelo recomendado:** Qwen2.5-Coder (7b-int4, 4.2GB) vía Ollama

**Flujo de procesamiento:**
1. Información cruda llega a cola de procesamiento (texto de OCR, commits, etc.)
2. Lunmia envía prompt a Ollama con contexto:
   ```
   Contexto: Usuario está en proyecto "lunmia-core"
   Información capturada: "Fix authentication bug in login endpoint"
   Tarea: Extraer action items, categorizar, asignar prioridad
   ```
3. Ollama responde con JSON estructurado:
   ```json
   {
     "action_items": [
       {
         "title": "Fix authentication bug in login endpoint",
         "project": "lunmia-core",
         "priority": "high",
         "type": "bug",
         "estimated_time": "2h"
       }
     ]
   }
   ```
4. Lunmia almacena tarea en DB local
5. Dashboard se actualiza en tiempo real (websocket/IPC)

**Criterios de aceptación:**
- [ ] Ollama se instala y configura automáticamente en first-run
- [ ] Modelo Qwen2.5-Coder se descarga automáticamente (con progress bar)
- [ ] Procesamiento de un action item toma < 3 segundos en promedio
- [ ] Sistema funciona offline 100% (no hace requests HTTP externos)
- [ ] Si Ollama falla, se muestra error claro y opción de reintento
- [ ] IA correctamente categoriza > 85% de tareas al proyecto correcto (medido por tasa de re-categorización manual)

### 5.4 Dashboard y Visualización

**Descripción:** Interfaz principal para visualizar proyectos, tareas y estadísticas.

**Componentes (basado en UI/UX provisto):**

**Vista Principal:**
- Header: Logo, badge "Local Mode", icono settings, icono notificaciones
- Cards de estado:
  - Ollama Status: "Connected" + modelo activo
  - Active Model: Nombre + tamaño
- Project Insights:
  - Gráfico de barras por lenguaje (TypeScript, Rust, Go, etc.)
  - Porcentaje de cada lenguaje
  - Tokens indexados, Files analyzed
- Git Worktrees:
  - Lista de branches activas
  - Indicador de branch activo
  - Paths de worktrees
- Bottom Navigation:
  - Dash (vista actual)
  - Projects
  - Memory
  - Settings

**Flujo de interacción:**
1. Usuario abre Lunmia
2. Dashboard carga en < 1 segundo
3. Stats se actualizan en tiempo real conforme IA procesa información
4. Click en proyecto → navega a vista detallada de proyecto
5. Click en "Memory" → vista de búsqueda y timeline
6. Click en "Settings" → configuración de MCP, permisos, modo privado

**Criterios de aceptación:**
- [ ] Dashboard carga completamente en < 1 segundo
- [ ] Stats se actualizan sin flickering (smooth transitions)
- [ ] Responsive (funciona en ventana pequeña 800x600)
- [ ] Dark mode por defecto (tema claro futuro)
- [ ] Todas las interacciones responden en < 100ms
- [ ] Gráficos son accesibles (labels, contrast ratios WCAG AA)

### 5.5 Búsqueda Inteligente

**Descripción:** Sistema de búsqueda semántica sobre toda la información capturada.

**Flujo de usuario:**
1. Usuario navega a sección "Memory"
2. Barra de búsqueda prominente en top
3. Usuario escribe query: "meeting about authentication"
4. Sistema hace búsqueda semántica (no solo keyword matching)
5. Resultados aparecen en < 1 segundo:
   - Reuniones relacionadas
   - Tareas de autenticación
   - Commits con palabra "auth"
6. Click en resultado → vista detallada con contexto completo

**Criterios de aceptación:**
- [ ] Búsqueda responde en < 1 segundo para DB con 50k entradas
- [ ] Búsqueda semántica (entiende sinónimos: "fix" = "bug" = "issue")
- [ ] Soporta filtros: por proyecto, por fecha, por tipo
- [ ] Resultados se ordenan por relevancia (scoring IA)
- [ ] Highlighting de términos de búsqueda en resultados

### 5.6 Configuración de MCP

**Descripción:** Pantalla de setup para conectar extensiones y configurar MCP Server.

**Flujo (pantalla 1 de UI provisto):**
1. Usuario ve sección "MCP Configuration"
2. Toggle "Enable MCP Server" → activa servidor en localhost:4567
3. Badge "READY TO BRIDGE" aparece cuando servidor está activo
4. Dropdown "Target Environment" → selecciona entre:
   - VS Code Extension
   - Cursor Extension
   - Chrome Extension
   - Safari Extension
5. Para cada extensión, instrucciones específicas de instalación:
   - Link a Chrome Web Store / VS Code Marketplace
   - Código de configuración a copiar
   - Verificación de conexión
6. Sección "Tech Stack Detection" muestra stacks detectados automáticamente

**Criterios de aceptación:**
- [ ] MCP Server inicia automáticamente al abrir Lunmia
- [ ] Puerto configurable (default 4567, editable en settings)
- [ ] Instrucciones de instalación claras para cada extensión
- [ ] Verificación de conexión funciona (ping/pong test)
- [ ] Si extensión no conecta en 30s, troubleshooting tips se muestran

---

## 6. Requisitos No Funcionales

### 6.1 Performance

| Requisito | Objetivo |
|-----------|----------|
| **Tiempo de inicio** | App lista en < 3 segundos |
| **Tiempo de búsqueda** | < 1 segundo para 95% de queries |
| **Procesamiento IA** | < 5 segundos por action item |
| **Dashboard refresh** | Stats actualizan en < 500ms |
| **Tamaño de DB soportado** | Hasta 100k tareas/apuntes sin degradación |
| **Uso de RAM** | < 2GB en idle, < 4GB bajo carga |
| **Uso de CPU** | < 10% en idle, < 60% procesando IA |

### 6.2 Seguridad

- **Detección de información sensible:**
  - Passwords (patterns comunes)
  - API keys (formato AWS, OpenAI, etc.)
  - Tokens JWT
  - Tarjetas de crédito (regex Luhn algorithm)
  - SSN, números de pasaporte

- **Encriptación:**
  - Base de datos NO encriptada por defecto (para performance)
  - Opción futura de encriptación en reposo con password del usuario

- **Permisos:**
  - Solicitar permisos de OS solo cuando necesarios
  - Explicación clara de por qué cada permiso se requiere

### 6.3 Escalabilidad

- Arquitectura modular: agregar nuevos agentes MCP sin refactorizar core
- Sistema de plugins futuro: usuarios pueden escribir sus propios agentes
- DB schema con migraciones (versionado) para evolucionar sin romper datos

### 6.4 Compatibilidad

| Plataforma | Versión Mínima | Notas |
|------------|----------------|-------|
| **macOS** | 12.0 (Monterey) | Apple Silicon y Intel |
| **Windows** | 10 (build 19041+) | |
| **Linux** | Ubuntu 20.04, Fedora 36 | AppImage universal |
| **Chrome** | v120+ | Para extensión |
| **Safari** | 16+ | Solo macOS |
| **VS Code** | 1.85+ | |
| **Cursor** | 0.20+ | |

---

## 7. Arquitectura Técnica

### 7.1 Stack Tecnológico

| Componente | Tecnología | Justificación |
|------------|------------|---------------|
| **Frontend** | SvelteKit 2 + Svelte 5 | Reactivo, performante, tipado con TS |
| **Desktop Runtime** | Tauri 2 | Ligero (vs Electron), Rust security, acceso a OS APIs |
| **Backend (Rust)** | Tauri Commands + SQLite | Core de negocio en Rust para performance y seguridad |
| **Database** | SQLite (via rusqlite) | Local, rápido, sin servidor, probado |
| **IA Local** | Ollama + Qwen2.5-Coder | Open source, runs on TPU/CPU, API compatible con OpenAI |
| **MCP Server** | Rust (Actix/Tokio) | Servidor HTTP/WebSocket para comunicación con extensiones |
| **OCR** | Tesseract (via leptess) | Open source, alta accuracy, multiplataforma |
| **Embeddings** | nomic-embed-text (Ollama) | Para búsqueda semántica |
| **Extensiones** | TypeScript + WebExtension APIs | Cross-browser compatible |

### 7.2 Diagrama de Arquitectura

```
┌─────────────────────────────────────────────────────────────┐
│                     LUNMIA MEMORY                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         FRONTEND (SvelteKit + Svelte 5)              │  │
│  │  - Dashboard                                          │  │
│  │  - Memory (búsqueda)                                  │  │
│  │  - Projects                                           │  │
│  │  - Settings                                           │  │
│  └────────────────┬─────────────────────────────────────┘  │
│                   │ IPC (Tauri Commands)                    │
│  ┌────────────────▼─────────────────────────────────────┐  │
│  │         BACKEND (Rust/Tauri)                         │  │
│  │  ┌─────────────┐  ┌──────────────┐  ┌────────────┐  │  │
│  │  │  Project    │  │   Capture    │  │   Search   │  │  │
│  │  │  Manager    │  │   Engine     │  │   Engine   │  │  │
│  │  └─────────────┘  └──────────────┘  └────────────┘  │  │
│  │  ┌─────────────┐  ┌──────────────┐  ┌────────────┐  │  │
│  │  │   OCR       │  │   Security   │  │    MCP     │  │  │
│  │  │  Service    │  │   Filter     │  │   Server   │  │  │
│  │  └─────────────┘  └──────────────┘  └────────────┘  │  │
│  └────────────────┬─────────────────────────────────────┘  │
│                   │                                         │
│  ┌────────────────▼─────────────────────────────────────┐  │
│  │              SQLite Database                         │  │
│  │  - projects                                          │  │
│  │  - tasks                                             │  │
│  │  - captures                                          │  │
│  │  - embeddings                                        │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
└──────────────────────┬───────────────────────────────────────┘
                       │
        ┌──────────────┼──────────────┐
        │              │              │
   ┌────▼────┐    ┌────▼────┐   ┌────▼────┐
   │ Ollama  │    │  MCP    │   │   OS    │
   │ (Local  │    │ Clients │   │  APIs   │
   │  AI)    │    │         │   │         │
   └─────────┘    └────┬────┘   └─────────┘
                       │
         ┌─────────────┼─────────────┐
         │             │             │
    ┌────▼────┐   ┌────▼────┐  ┌────▼────┐
    │ Chrome  │   │   VS    │  │ Cursor  │
    │  Ext    │   │  Code   │  │   Ext   │
    └─────────┘   └─────────┘  └─────────┘
```

### 7.3 Modelo de Datos

**Entidades Principales:**

```sql
-- Proyectos
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    tech_stack JSON, -- ["typescript", "rust"]
    created_at INTEGER NOT NULL,
    last_activity INTEGER NOT NULL,
    is_active BOOLEAN DEFAULT 1
);

-- Tareas
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT, -- "low", "medium", "high", "urgent"
    status TEXT, -- "pending", "in_progress", "completed"
    type TEXT, -- "task", "bug", "feature", "meeting_note"
    source TEXT, -- "auto_chrome", "auto_vscode", "manual"
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Capturas (metadata, sin imágenes)
CREATE TABLE captures (
    id TEXT PRIMARY KEY,
    project_id TEXT,
    source TEXT NOT NULL, -- "chrome_extension", "vscode", etc.
    captured_text TEXT, -- resultado de OCR
    metadata JSON, -- {url, title, timestamp, etc.}
    created_at INTEGER NOT NULL,
    processed BOOLEAN DEFAULT 0,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Embeddings (para búsqueda semántica)
CREATE TABLE embeddings (
    id TEXT PRIMARY KEY,
    task_id TEXT,
    capture_id TEXT,
    embedding BLOB NOT NULL, -- vector de 768 dimensiones
    created_at INTEGER NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id),
    FOREIGN KEY (capture_id) REFERENCES captures(id)
);

-- Configuración
CREATE TABLE config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

---

## 8. Diseño y UX

### 8.1 Wireframes y Referencias

Se han proporcionado 2 diseños de referencia:

**Diseño 1: Project Setup & MCP Configuration**
- Dark theme con acentos en azul/violeta (#6366F1)
- Header con logo, título "Project Setup & MCP", settings icon
- Progress dots (paso 2 de 3)
- Drop zone para seleccionar repositorio local
- Sección "MCP Configuration" con toggle "Enable MCP Server"
- Dropdown "Target Environment" para seleccionar VS Code, Chrome, etc.
- Badge "READY TO BRIDGE" en verde cuando está configurado
- Status: "Listening on localhost:4567"
- Tech Stack Detection (SvelteKit, Rust Core)
- AI Insights con descripción de integración
- CTA principal: "Start Memory Sync" con icono de rayo

**Diseño 2: Dashboard Principal**
- Header: "Lunmia Memory", badge "LOCAL MODE" en verde, notification bell
- Cards de estado: Ollama Connected + Qwen2.5-Coder (7b-int4, 4.2GB)
- Project Insights: gráfico de barras horizontal
  - TypeScript: 64%
  - Rust: 28%
  - Go/Config: 8%
  - Stats: 1.2M tokens indexed, 442 files analyzed
- Git Worktrees: lista de branches con badges "ACTIVE"
- Bottom nav: Dash (activo), Projects, Memory, Settings
- Floating action button (chat icon) en bottom right

**Paleta de Colores:**
- Background: #0A0A0F (casi negro)
- Cards: #1A1A24 (gris oscuro)
- Accent: #6366F1 (azul violeta)
- Success: #10B981 (verde)
- Text: #E5E7EB (gris claro)
- Muted: #6B7280 (gris medio)

**Tipografía:**
- Sans-serif moderna (Inter, Geist, o similar)
- Weights: 400 (regular), 500 (medium), 600 (semibold)

### 8.2 Flujos de Usuario

**Flujo 1: First-Time Setup**
```
1. Bienvenida
   ↓
2. Instalar Ollama (si no está instalado)
   ↓
3. Descargar modelo Qwen2.5-Coder (progress bar)
   ↓
4. Configurar MCP Server
   ↓
5. Instalar extensiones (guía para cada una)
   ↓
6. Seleccionar carpeta de proyectos
   ↓
7. Escanear y detectar proyectos
   ↓
8. Personalizar nombres de proyectos
   ↓
9. Solicitar permisos del sistema
   ↓
10. Dashboard principal (listo)
```

**Flujo 2: Captura Automática en Reunión**
```
1. Usuario abre pestaña de Slack call
   ↓
2. Chrome extension detecta URL
   ↓
3. Envía notificación a Lunmia: "Reunión detectada"
   ↓
4. Lunmia toma screenshot cada 30 segundos
   ↓
5. OCR extrae texto
   ↓
6. Security filter verifica info sensible
   ↓
7. IA procesa texto, extrae action items
   ↓
8. Crea tareas automáticamente en proyecto relevante
   ↓
9. Screenshot se elimina
   ↓
10. Dashboard actualiza stats en tiempo real
```

**Flujo 3: Búsqueda de Información**
```
1. Usuario navega a "Memory"
   ↓
2. Escribe: "authentication meeting last week"
   ↓
3. Sistema genera embedding de query
   ↓
4. Búsqueda por similaridad en DB
   ↓
5. Resultados aparecen en < 1s:
   - Reunión "Auth Implementation" (hace 5 días)
   - Tarea "Fix JWT validation" (hace 4 días)
   - Commit "Update auth middleware" (hace 3 días)
   ↓
6. Usuario click en reunión → ve resumen completo con contexto
```

---

## 9. Plan de Implementación

### 9.1 Fases

| Fase | Duración Estimada | Entregables |
|------|-------------------|-------------|
| **Fase 1: Setup y Core** | 2 semanas | - Proyecto Tauri configurado<br>- SQLite integrado<br>- Dashboard básico funcional<br>- Ollama integrado |
| **Fase 2: Agentes MCP** | 3 semanas | - MCP Server funcional<br>- Chrome extension MVP<br>- VS Code extension MVP<br>- Captura de screenshots y OCR |
| **Fase 3: IA y Procesamiento** | 2 semanas | - Pipeline de procesamiento IA<br>- Detección de info sensible<br>- Creación automática de tareas<br>- Categorización por proyecto |
| **Fase 4: Búsqueda** | 1 semana | - Embeddings generados<br>- Búsqueda semántica funcional<br>- Filtros y sorting |
| **Fase 5: UX/Onboarding** | 2 semanas | - Pantalla de bienvenida<br>- Setup wizard completo<br>- Modo privado<br>- Notificaciones push |
| **Fase 6: Testing y Polish** | 2 semanas | - Bug fixes<br>- Performance optimization<br>- User testing<br>- Documentación |

**TOTAL:** ~12 semanas (3 meses)

### 9.2 Dependencias

**Críticas:**
- Ollama debe estar instalado y funcional antes de usar IA
- MCP Server debe estar activo antes de conectar extensiones
- Permisos de sistema deben estar otorgados antes de captura automática

**Técnicas:**
- Tauri 2 estable (ya disponible)
- Ollama stable (ya disponible)
- Tesseract binaries para cada OS

### 9.3 Riesgos

| Riesgo | Probabilidad | Impacto | Mitigación |
|--------|--------------|---------|------------|
| **Ollama no funciona en hardware antiguo** | Media | Alto | Detectar capacidades en onboarding, ofrecer modelos más pequeños (3b) |
| **OCR accuracy baja en screenshots complejos** | Media | Medio | Usar Tesseract 5.x con LSTM, pre-procesar imágenes (threshold, denoise) |
| **Extensiones bloqueadas por stores** | Baja | Alto | Seguir guidelines estrictas, ofrecer sideloading |
| **Performance degrada con 50k+ tareas** | Media | Alto | Indexar DB correctamente, implementar pagination, archiving de tareas viejas |
| **Detección de info sensible tiene falsos positivos** | Alta | Bajo | Preferir false positives sobre leaks, permitir whitelist de patterns |
| **MCP protocol cambia (es nuevo)** | Media | Medio | Abstraer MCP detrás de interface, fácil de reemplazar |

---

## 10. Apéndices

### 10.1 Glosario

- **MCP (Model Context Protocol):** Protocolo de Anthropic para exponer contexto de aplicaciones a modelos de IA.
- **Ollama:** Runtime local para ejecutar LLMs (similar a Docker pero para modelos IA).
- **TPU/NPU:** Tensor Processing Unit / Neural Processing Unit - hardware especializado para IA.
- **OCR:** Optical Character Recognition - extracción de texto de imágenes.
- **Embeddings:** Representación vectorial de texto para búsqueda semántica.
- **Worktree:** Feature de Git para trabajar en múltiples branches simultáneamente.

### 10.2 Referencias

- **MCP Docs:** https://modelcontextprotocol.io
- **Ollama:** https://ollama.ai
- **Tauri:** https://tauri.app
- **Qwen2.5-Coder:** https://huggingface.co/Qwen/Qwen2.5-Coder-7B-Instruct
- **Tesseract OCR:** https://github.com/tesseract-ocr/tesseract

### 10.3 UI/UX Assets

Referencia visual almacenada en: `/docs/ui/`
- Diseño 1: Setup y MCP Configuration
- Diseño 2: Dashboard principal

---

## 11. Notas de Cierre

Este PRD define un producto ambicioso pero viable que aprovecha tecnologías emergentes (IA local, MCP) para resolver un problema real: la sobrecarga cognitiva de gestión de proyectos. La propuesta de valor única (automatización + privacidad) lo diferencia claramente de competidores.

**Próximos Pasos:**
1. Revisión y aprobación de este PRD
2. Generación de Task List detallado (FASE 2 del PRD Workflow)
3. Inicio de implementación (FASE 3 del PRD Workflow)

---

*Documento PRD v1.0 - Lunmia Memory*
*Metodología: PRD Workflow System by Oleksiy Kovyrin*
*Fecha: 2026-01-14*
