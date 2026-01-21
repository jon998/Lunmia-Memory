# Fase 2: Agentes MCP

**Duración Estimada:** 3 semanas  
**Objetivo:** MCP Server funcional, Chrome extension MVP, VS Code extension MVP, Captura de screenshots y OCR

**Dependencias:** Fase 1 completada

---

## Tarea 2.1: MCP Server - Estructura Base

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar el servidor MCP base usando Rust con Actix/Tokio para comunicación con extensiones.

**Criterios de Aceptación:**
- [ ] Servidor HTTP/WebSocket iniciado en puerto configurable (default 4567)
- [ ] Endpoint `/health` para verificación de estado
- [ ] Endpoint `/api/v1/ping` para pruebas de conexión
- [ ] Manejo de CORS para extensiones de navegador
- [ ] Logging de conexiones y eventos
- [ ] Inicio automático con la aplicación Tauri

**Archivos Afectados:**
- `src-tauri/src/mcp/mod.rs`
- `src-tauri/src/mcp/server.rs`
- `src-tauri/src/mcp/routes.rs`
- `src-tauri/Cargo.toml` (dependencias actix-web, tokio)

**Decisiones Requeridas:**
- [ ] Framework HTTP (Actix-web vs Axum vs Warp)
- [ ] Estructura de endpoints API

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.2: MCP Server - Protocolo de Comunicación

**Estado:** `[ ] Pendiente`

**Descripción:**
Definir e implementar el protocolo de comunicación entre extensiones y Lunmia.

**Criterios de Aceptación:**
- [ ] Estructura de mensajes JSON definida
- [ ] Tipos de eventos: `capture`, `activity`, `meeting_detected`, `commit`
- [ ] Autenticación por token local (generado en primera conexión)
- [ ] Validación de mensajes entrantes
- [ ] Respuestas estructuradas con códigos de estado

**Estructura de Mensaje:**
```json
{
  "type": "capture" | "activity" | "meeting" | "commit",
  "source": "chrome" | "vscode" | "cursor" | "safari",
  "timestamp": 1705234567890,
  "data": { ... },
  "token": "local-auth-token"
}
```

**Archivos Afectados:**
- `src-tauri/src/mcp/protocol.rs`
- `src-tauri/src/mcp/handlers.rs`
- `src-tauri/src/mcp/auth.rs`

**Decisiones Requeridas:**
- [ ] Estrategia de autenticación (token fijo vs rotativo)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.3: MCP Server - Endpoints de Captura

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar endpoints para recibir datos de captura de las extensiones.

**Criterios de Aceptación:**
- [ ] `POST /api/v1/capture` - recibe metadata de captura
- [ ] `POST /api/v1/activity` - recibe actividad de usuario
- [ ] `POST /api/v1/meeting` - notificación de reunión detectada
- [ ] `POST /api/v1/commit` - información de commits
- [ ] Almacenamiento en cola de procesamiento
- [ ] Respuesta con ID de captura para tracking

**Archivos Afectados:**
- `src-tauri/src/mcp/routes.rs`
- `src-tauri/src/mcp/handlers/capture.rs`
- `src-tauri/src/mcp/handlers/activity.rs`
- `src-tauri/src/mcp/handlers/meeting.rs`
- `src-tauri/src/mcp/handlers/commit.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.4: UI - Configuración MCP

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar la pantalla de configuración MCP según el diseño proporcionado.

**Criterios de Aceptación:**
- [ ] Toggle "Enable MCP Server" funcional
- [ ] Indicador de estado del servidor (listening, stopped, error)
- [ ] Dropdown para seleccionar Target Environment
- [ ] Badge "READY TO BRIDGE" cuando está activo
- [ ] Mostrar puerto activo (localhost:XXXX)
- [ ] Instrucciones de instalación por extensión

**Archivos Afectados:**
- `src/routes/settings/mcp/+page.svelte`
- `src/lib/components/MCPConfig.svelte`
- `src/lib/components/ExtensionGuide.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.5: Chrome Extension - Estructura Base

**Estado:** `[ ] Pendiente`

**Descripción:**
Crear la estructura base de la extensión de Chrome usando Manifest V3.

**Criterios de Aceptación:**
- [ ] Manifest V3 configurado correctamente
- [ ] Background service worker funcional
- [ ] Popup básico con estado de conexión
- [ ] Content script para detección de páginas
- [ ] Permisos mínimos necesarios declarados
- [ ] Build script con webpack/vite

**Archivos Afectados:**
- `extensions/chrome/manifest.json`
- `extensions/chrome/src/background.ts`
- `extensions/chrome/src/popup/`
- `extensions/chrome/src/content.ts`
- `extensions/chrome/package.json`

**Decisiones Requeridas:**
- [ ] Bundler para extensión (Vite vs Webpack)
- [ ] Estructura de carpetas de extensiones

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.6: Chrome Extension - Conexión con MCP

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar la comunicación entre la extensión de Chrome y el MCP Server.

**Criterios de Aceptación:**
- [ ] Función para conectar con MCP Server
- [ ] Almacenamiento de token de autenticación
- [ ] Reconexión automática si se pierde conexión
- [ ] Indicador visual de estado de conexión en popup
- [ ] Manejo de errores de conexión

**Archivos Afectados:**
- `extensions/chrome/src/services/mcp-client.ts`
- `extensions/chrome/src/background.ts`
- `extensions/chrome/src/popup/App.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.7: Chrome Extension - Detección de Reuniones

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar detección automática de reuniones en navegador.

**Criterios de Aceptación:**
- [ ] Detecta URLs de Slack calls (slack.com/call)
- [ ] Detecta URLs de Microsoft Teams (teams.microsoft.com)
- [ ] Detecta URLs de Google Meet (meet.google.com)
- [ ] Detecta URLs de Zoom (zoom.us)
- [ ] Envía evento `meeting_detected` a MCP Server
- [ ] Incluye metadata: URL, título de pestaña, timestamp

**Archivos Afectados:**
- `extensions/chrome/src/services/meeting-detector.ts`
- `extensions/chrome/src/content.ts`
- `extensions/chrome/src/background.ts`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.8: Chrome Extension - Captura de Actividad

**Estado:** `[ ] Pendiente`

**Descripción:**
Capturar actividad del navegador y enviar a Lunmia.

**Criterios de Aceptación:**
- [ ] Captura cambios de pestaña activa
- [ ] Captura título y URL (sin datos sensibles)
- [ ] Throttle de eventos (máximo 1 por segundo)
- [ ] Filtro de URLs privadas/sensibles
- [ ] Opción de pausar captura manualmente

**Archivos Afectados:**
- `extensions/chrome/src/services/activity-tracker.ts`
- `extensions/chrome/src/background.ts`

**Decisiones Requeridas:**
- [ ] Lista de dominios a excluir por defecto

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.9: VS Code Extension - Estructura Base

**Estado:** `[ ] Pendiente`

**Descripción:**
Crear la estructura base de la extensión de VS Code.

**Criterios de Aceptación:**
- [ ] Extension configurada con TypeScript
- [ ] Activation events definidos
- [ ] Comando para conectar/desconectar MCP
- [ ] Status bar item mostrando estado de conexión
- [ ] Settings de extensión (puerto MCP, enable/disable)

**Archivos Afectados:**
- `extensions/vscode/package.json`
- `extensions/vscode/src/extension.ts`
- `extensions/vscode/src/statusbar.ts`
- `extensions/vscode/tsconfig.json`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.10: VS Code Extension - Monitoreo de Archivos

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar monitoreo de cambios en archivos y eventos del editor.

**Criterios de Aceptación:**
- [ ] Detecta guardado de archivos
- [ ] Captura nombre de archivo y lenguaje
- [ ] Captura líneas modificadas (count, no contenido)
- [ ] Envía eventos a MCP Server
- [ ] Respeta .gitignore para exclusiones

**Archivos Afectados:**
- `extensions/vscode/src/services/file-watcher.ts`
- `extensions/vscode/src/services/mcp-client.ts`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.11: VS Code Extension - Monitoreo de Git

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar monitoreo de commits y operaciones Git.

**Criterios de Aceptación:**
- [ ] Detecta commits en repositorio activo
- [ ] Captura mensaje de commit
- [ ] Detecta keywords de action items (TODO, FIXME, etc.)
- [ ] Envía información a MCP Server
- [ ] No captura diff completo (solo metadata)

**Archivos Afectados:**
- `extensions/vscode/src/services/git-watcher.ts`
- `extensions/vscode/src/extension.ts`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.12: Sistema de Screenshots - Captura

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar captura de screenshots de ventana activa.

**Criterios de Aceptación:**
- [ ] Captura de ventana activa (no pantalla completa)
- [ ] Soporte para macOS, Windows, Linux
- [ ] Formato PNG con compresión
- [ ] Almacenamiento temporal seguro
- [ ] Permisos de sistema solicitados correctamente

**Archivos Afectados:**
- `src-tauri/src/capture/mod.rs`
- `src-tauri/src/capture/screenshot.rs`
- `src-tauri/Cargo.toml` (dependencia screenshots)

**Decisiones Requeridas:**
- [ ] Librería de captura (screenshots-rs vs alternativas)
- [ ] Resolución/calidad de captura

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.13: Sistema de OCR - Integración Tesseract

**Estado:** `[ ] Pendiente`

**Descripción:**
Integrar Tesseract OCR para extracción de texto de screenshots.

**Criterios de Aceptación:**
- [ ] Tesseract integrado via leptess o similar
- [ ] Función para extraer texto de imagen
- [ ] Preprocesamiento de imagen (threshold, denoise)
- [ ] Soporte para múltiples idiomas (inglés, español)
- [ ] Manejo de errores cuando OCR falla

**Archivos Afectados:**
- `src-tauri/src/capture/ocr.rs`
- `src-tauri/Cargo.toml` (dependencia leptess)

**Decisiones Requeridas:**
- [ ] Idiomas de OCR a soportar por defecto
- [ ] Estrategia de instalación de Tesseract (bundled vs system)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.14: Pipeline de Captura Automática

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar el pipeline completo de captura: trigger → screenshot → OCR → almacenamiento.

**Criterios de Aceptación:**
- [ ] Trigger desde evento de reunión detectada
- [ ] Captura de screenshot
- [ ] Procesamiento OCR
- [ ] Almacenamiento de texto en tabla captures
- [ ] Eliminación inmediata de imagen
- [ ] Logging del proceso completo

**Archivos Afectados:**
- `src-tauri/src/capture/pipeline.rs`
- `src-tauri/src/capture/mod.rs`
- `src-tauri/src/mcp/handlers/meeting.rs`

**Decisiones Requeridas:**
- [ ] Intervalo de captura durante reuniones (cada X segundos)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 2.15: Modo Privado - Implementación

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar el modo privado que pausa toda captura automática.

**Criterios de Aceptación:**
- [ ] Toggle global accesible desde UI
- [ ] Bloquea 100% de capturas cuando activo
- [ ] Indicador visual claro en header
- [ ] Notifica a extensiones conectadas
- [ ] Persistencia de estado entre sesiones
- [ ] Atajo de teclado para activar/desactivar

**Archivos Afectados:**
- `src-tauri/src/services/privacy.rs`
- `src/lib/stores/privacy.ts`
- `src/lib/components/PrivacyToggle.svelte`
- `src/lib/components/Header.svelte`

**Decisiones Requeridas:**
- [ ] Atajo de teclado preferido

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Resumen de Fase 2

| Tarea | Estado | Code Review |
|-------|--------|-------------|
| 2.1 MCP Server Base | [ ] | [ ] |
| 2.2 Protocolo de Comunicación | [ ] | [ ] |
| 2.3 Endpoints de Captura | [ ] | [ ] |
| 2.4 UI Configuración MCP | [ ] | [ ] |
| 2.5 Chrome Extension Base | [ ] | [ ] |
| 2.6 Chrome MCP Conexión | [ ] | [ ] |
| 2.7 Chrome Detección Reuniones | [ ] | [ ] |
| 2.8 Chrome Captura Actividad | [ ] | [ ] |
| 2.9 VS Code Extension Base | [ ] | [ ] |
| 2.10 VS Code Monitoreo Archivos | [ ] | [ ] |
| 2.11 VS Code Monitoreo Git | [ ] | [ ] |
| 2.12 Sistema Screenshots | [ ] | [ ] |
| 2.13 Integración Tesseract | [ ] | [ ] |
| 2.14 Pipeline de Captura | [ ] | [ ] |
| 2.15 Modo Privado | [ ] | [ ] |

**Progreso:** 0/15 tareas completadas
