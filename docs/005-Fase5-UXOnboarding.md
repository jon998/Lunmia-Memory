# Fase 5: UX y Onboarding

**Duración Estimada:** 2 semanas  
**Objetivo:** Pantalla de bienvenida, Setup wizard completo, Modo privado, Notificaciones push

**Dependencias:** Fase 4 completada

---

## Tarea 5.1: Pantalla de Bienvenida

**Estado:** `[ ] Pendiente`

**Descripción:**
Crear pantalla de bienvenida para nuevos usuarios.

**Criterios de Aceptación:**
- [ ] Logo de Lunmia Memory prominente
- [ ] Mensaje de bienvenida atractivo
- [ ] Breve descripción de funcionalidades
- [ ] Botón "Comenzar Setup" como CTA principal
- [ ] Animaciones sutiles de entrada
- [ ] Se muestra solo en primera ejecución

**Archivos Afectados:**
- `src/routes/onboarding/+page.svelte`
- `src/routes/onboarding/welcome/+page.svelte`
- `src/lib/components/Logo.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.2: Verificación e Instalación de Ollama

**Estado:** `[ ] Pendiente`

**Descripción:**
Guiar al usuario en la instalación de Ollama si no está presente.

**Criterios de Aceptación:**
- [ ] Detección automática de Ollama instalado
- [ ] Si no está: mostrar instrucciones de instalación
- [ ] Link directo a descarga de Ollama
- [ ] Botón "Verificar instalación" para recheck
- [ ] Indicador visual de estado (instalado/no instalado)
- [ ] Si está: pasar al siguiente paso automáticamente

**Archivos Afectados:**
- `src/routes/onboarding/ollama/+page.svelte`
- `src/lib/components/OllamaInstallGuide.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.3: Descarga de Modelo IA

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar descarga automática del modelo de IA con progress bar.

**Criterios de Aceptación:**
- [ ] Verificar si modelo ya existe
- [ ] Si no: iniciar descarga automática
- [ ] Progress bar con porcentaje y velocidad
- [ ] Tamaño total y descargado visible
- [ ] Botón cancelar descarga
- [ ] Retry automático en caso de fallo
- [ ] Mensaje de éxito al completar

**Archivos Afectados:**
- `src/routes/onboarding/model/+page.svelte`
- `src/lib/components/DownloadProgress.svelte`
- `src-tauri/src/commands/ollama.rs` (comando pull_model)

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.4: Configuración MCP en Onboarding

**Estado:** `[ ] Pendiente`

**Descripción:**
Paso de onboarding para configurar y conectar extensiones MCP.

**Criterios de Aceptación:**
- [ ] Toggle para habilitar MCP Server
- [ ] Selección de extensiones a instalar
- [ ] Instrucciones paso a paso por extensión
- [ ] Links a Chrome Web Store / VS Code Marketplace
- [ ] Código de configuración copiable
- [ ] Verificación de conexión con indicador
- [ ] Opción de "Skip" para configurar después

**Archivos Afectados:**
- `src/routes/onboarding/mcp/+page.svelte`
- `src/lib/components/ExtensionSetupStep.svelte`
- `src/lib/components/CopyableCode.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.5: Selección de Carpeta de Proyectos

**Estado:** `[ ] Pendiente`

**Descripción:**
Permitir al usuario seleccionar directorio raíz de proyectos.

**Criterios de Aceptación:**
- [ ] Botón para abrir diálogo de selección de carpeta
- [ ] Mostrar path seleccionado
- [ ] Botón "Escanear" para buscar proyectos
- [ ] Indicador de escaneo en progreso
- [ ] Manejo de permisos si no tiene acceso
- [ ] Sugerencias de ubicaciones comunes (~/Projects, ~/Developer)

**Archivos Afectados:**
- `src/routes/onboarding/projects/+page.svelte`
- `src/lib/components/FolderPicker.svelte`
- `src-tauri/src/commands/filesystem.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.6: Detección Automática de Proyectos

**Estado:** `[ ] Pendiente`

**Descripción:**
Escanear carpeta seleccionada y detectar proyectos Git.

**Criterios de Aceptación:**
- [ ] Escaneo recursivo hasta 5 niveles de profundidad
- [ ] Detección de repos Git (.git folder)
- [ ] Inferencia de nombre desde package.json, Cargo.toml, etc.
- [ ] Detección de tech stack (lenguajes principales)
- [ ] Lista de proyectos encontrados con preview
- [ ] Contador de proyectos detectados

**Archivos Afectados:**
- `src-tauri/src/services/project_scanner.rs`
- `src-tauri/src/commands/projects.rs`
- `src/routes/onboarding/projects/scan/+page.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.7: Personalización de Proyectos Detectados

**Estado:** `[ ] Pendiente`

**Descripción:**
Permitir al usuario renombrar y confirmar proyectos antes de importar.

**Criterios de Aceptación:**
- [ ] Lista editable de proyectos detectados
- [ ] Campo de nombre editable por proyecto
- [ ] Checkbox para incluir/excluir proyectos
- [ ] Mostrar tech stack detectado
- [ ] Opción de crear proyecto manual
- [ ] Botón "Confirmar e Importar"
- [ ] Límite visual de 20 proyectos

**Archivos Afectados:**
- `src/routes/onboarding/projects/customize/+page.svelte`
- `src/lib/components/ProjectListEditor.svelte`
- `src/lib/components/ProjectEditRow.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.8: Solicitud de Permisos del Sistema

**Estado:** `[ ] Pendiente`

**Descripción:**
Solicitar permisos necesarios del sistema operativo.

**Criterios de Aceptación:**
- [ ] Detección de OS (macOS, Windows, Linux)
- [ ] Lista de permisos necesarios por OS
- [ ] Explicación clara de por qué cada permiso
- [ ] Botón para solicitar cada permiso
- [ ] Indicador de estado (granted, denied, pending)
- [ ] Link a System Preferences si está denegado

**Permisos por OS:**
- **macOS:** Screen Recording, Accessibility
- **Windows:** Administrator (si necesario), Notifications
- **Linux:** X11/Wayland permissions

**Archivos Afectados:**
- `src/routes/onboarding/permissions/+page.svelte`
- `src/lib/components/PermissionRequest.svelte`
- `src-tauri/src/services/permissions.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.9: Tour Interactivo del Dashboard

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar tour guiado de las funcionalidades principales.

**Criterios de Aceptación:**
- [ ] Overlay con spotlight en elementos
- [ ] Tooltips explicativos por sección
- [ ] Navegación: Siguiente, Anterior, Omitir
- [ ] Progress dots del tour
- [ ] Pasos: Header, Stats, Projects, Memory, Settings
- [ ] Opción de repetir tour desde Settings

**Archivos Afectados:**
- `src/lib/components/Tour.svelte`
- `src/lib/components/TourStep.svelte`
- `src/lib/stores/tour.ts`

**Decisiones Requeridas:**
- [ ] Librería de tours (shepherd.js, custom, etc.)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.10: Sistema de Notificaciones Push

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar notificaciones del sistema para eventos importantes.

**Criterios de Aceptación:**
- [ ] Notificación nativa del OS
- [ ] Eventos: tarea urgente creada, reunión detectada
- [ ] Click en notificación abre la app
- [ ] Toggle para habilitar/deshabilitar en Settings
- [ ] Quiet hours configurable
- [ ] Límite de frecuencia (no spam)

**Archivos Afectados:**
- `src-tauri/src/services/notifications.rs`
- `src-tauri/src/commands/notifications.rs`
- `src/routes/settings/notifications/+page.svelte`

**Decisiones Requeridas:**
- [ ] Eventos que disparan notificaciones

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.11: Notificaciones In-App (Toast)

**Estado:** `[ ] Pendiente`

**Descripción:**
Sistema de notificaciones dentro de la aplicación.

**Criterios de Aceptación:**
- [ ] Toast notifications en esquina
- [ ] Tipos: success, error, warning, info
- [ ] Auto-dismiss configurable
- [ ] Botón de cerrar
- [ ] Queue de múltiples notificaciones
- [ ] Animaciones de entrada/salida

**Archivos Afectados:**
- `src/lib/components/ToastContainer.svelte`
- `src/lib/components/Toast.svelte`
- `src/lib/stores/toasts.ts`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.12: Modo Privado - UI Completa

**Estado:** `[ ] Pendiente`

**Descripción:**
Completar la implementación de UI para modo privado.

**Criterios de Aceptación:**
- [ ] Toggle prominente en header
- [ ] Indicador visual claro (icono candado)
- [ ] Cambio de color en header cuando activo
- [ ] Tooltip explicando funcionalidad
- [ ] Confirmación al activar por primera vez
- [ ] Accesible desde Settings también

**Archivos Afectados:**
- `src/lib/components/PrivacyModeToggle.svelte`
- `src/lib/components/Header.svelte`
- `src/routes/settings/privacy/+page.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.13: Sugerencia Automática de Modo Privado

**Estado:** `[ ] Pendiente`

**Descripción:**
Detectar keywords sensibles y sugerir activar modo privado.

**Criterios de Aceptación:**
- [ ] Detección de keywords: "payment", "bank", "credit card", etc.
- [ ] Modal de sugerencia cuando se detecta
- [ ] Opciones: Activar, Ignorar esta vez, No mostrar más
- [ ] Logging de sugerencias (no contenido)
- [ ] Configurable en Settings

**Keywords de Detección:**
- payment, bank, credit, card, password, secret
- cvv, ssn, passport, account number
- Y variantes en español

**Archivos Afectados:**
- `src-tauri/src/services/privacy_detector.rs`
- `src/lib/components/PrivacySuggestionModal.svelte`

**Decisiones Requeridas:**
- [ ] Lista completa de keywords a detectar

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 5.14: Estado de Primera Ejecución

**Estado:** `[ ] Pendiente`

**Descripción:**
Gestionar estado de onboarding y redirecciones.

**Criterios de Aceptación:**
- [ ] Flag en DB para onboarding completado
- [ ] Redirección automática si no completado
- [ ] Redirección a dashboard si ya completado
- [ ] Opción de repetir onboarding desde Settings
- [ ] Tracking de paso actual del onboarding

**Archivos Afectados:**
- `src-tauri/src/services/app_state.rs`
- `src/routes/+layout.ts`
- `src/lib/stores/onboarding.ts`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Resumen de Fase 5

| Tarea | Estado | Code Review |
|-------|--------|-------------|
| 5.1 Pantalla Bienvenida | [ ] | [ ] |
| 5.2 Verificación Ollama | [ ] | [ ] |
| 5.3 Descarga Modelo | [ ] | [ ] |
| 5.4 Config MCP Onboarding | [ ] | [ ] |
| 5.5 Selección Carpeta | [ ] | [ ] |
| 5.6 Detección Proyectos | [ ] | [ ] |
| 5.7 Personalización Proyectos | [ ] | [ ] |
| 5.8 Permisos Sistema | [ ] | [ ] |
| 5.9 Tour Interactivo | [ ] | [ ] |
| 5.10 Notificaciones Push | [ ] | [ ] |
| 5.11 Toasts In-App | [ ] | [ ] |
| 5.12 Modo Privado UI | [ ] | [ ] |
| 5.13 Sugerencia Privacidad | [ ] | [ ] |
| 5.14 Estado Primera Ejecución | [ ] | [ ] |

**Progreso:** 0/14 tareas completadas
