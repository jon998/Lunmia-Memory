# Fase 3: IA y Procesamiento

**Duración Estimada:** 2 semanas  
**Objetivo:** Pipeline de procesamiento IA, Detección de info sensible, Creación automática de tareas, Categorización por proyecto

**Dependencias:** Fase 2 completada

---

## Tarea 3.1: Servicio Ollama - Cliente HTTP

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar cliente HTTP completo para comunicación con Ollama API.

**Criterios de Aceptación:**
- [ ] Cliente HTTP con reqwest
- [ ] Función `generate` para completions
- [ ] Función `chat` para conversaciones
- [ ] Función `embeddings` para vectores
- [ ] Timeouts configurables
- [ ] Retry logic con backoff exponencial
- [ ] Manejo de streaming responses

**Archivos Afectados:**
- `src-tauri/src/services/ollama/client.rs`
- `src-tauri/src/services/ollama/mod.rs`
- `src-tauri/src/services/ollama/types.rs`

**Decisiones Requeridas:**
- [ ] Timeout default para requests (30s, 60s, etc.)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.2: Gestión de Modelos Ollama

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar gestión de modelos: descarga, verificación, selección.

**Criterios de Aceptación:**
- [ ] Verificar si modelo existe localmente
- [ ] Descargar modelo si no existe (con progress)
- [ ] Listar modelos disponibles
- [ ] Seleccionar modelo activo
- [ ] Persistir modelo seleccionado en config
- [ ] UI para selección de modelo en settings

**Modelos Recomendados:**
- Qwen2.5-Coder (7b-int4) - procesamiento de código
- nomic-embed-text - embeddings para búsqueda

**Archivos Afectados:**
- `src-tauri/src/services/ollama/models.rs`
- `src-tauri/src/commands/ollama.rs`
- `src/routes/settings/+page.svelte`

**Decisiones Requeridas:**
- [ ] Modelo default a descargar en primera instalación

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.3: Cola de Procesamiento

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar sistema de cola para procesar capturas de forma asíncrona.

**Criterios de Aceptación:**
- [ ] Cola en memoria con persistencia opcional
- [ ] Procesamiento FIFO (First In First Out)
- [ ] Límite de concurrencia configurable
- [ ] Reintentos en caso de fallo
- [ ] Estado de cola visible en UI (pending, processing, done)
- [ ] Priorización de items urgentes

**Archivos Afectados:**
- `src-tauri/src/processing/queue.rs`
- `src-tauri/src/processing/mod.rs`
- `src-tauri/src/processing/worker.rs`

**Decisiones Requeridas:**
- [ ] Límite de concurrencia (1, 2, o más)
- [ ] Persistencia de cola entre reinicios

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.4: Detector de Información Sensible

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar sistema de detección de información sensible antes de almacenar.

**Criterios de Aceptación:**
- [ ] Detección de passwords (patrones comunes)
- [ ] Detección de API keys (AWS, OpenAI, etc.)
- [ ] Detección de tokens JWT
- [ ] Detección de tarjetas de crédito (Luhn algorithm)
- [ ] Detección de SSN, números de pasaporte
- [ ] Logging de cantidad de info descartada (sin contenido)
- [ ] Whitelist configurable de patrones

**Patrones a Detectar:**
```regex
# API Keys
/(?:api[_-]?key|apikey|api_secret)[=:]\s*['""]?[\w-]{20,}/i
# AWS Keys
/(?:AKIA|AGPA|AIDA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16}/
# JWT
/eyJ[a-zA-Z0-9_-]*\.eyJ[a-zA-Z0-9_-]*\.[a-zA-Z0-9_-]*/
# Credit Cards
/\b(?:\d{4}[- ]?){3}\d{4}\b/
```

**Archivos Afectados:**
- `src-tauri/src/security/sensitive.rs`
- `src-tauri/src/security/patterns.rs`
- `src-tauri/src/security/mod.rs`

**Decisiones Requeridas:**
- [ ] Acción cuando se detecta info sensible (descartar todo vs redactar)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.5: Prompts de Procesamiento

**Estado:** `[ ] Pendiente`

**Descripción:**
Diseñar e implementar prompts para diferentes tipos de procesamiento.

**Criterios de Aceptación:**
- [ ] Prompt para extracción de action items
- [ ] Prompt para categorización de tareas
- [ ] Prompt para resumen de reuniones
- [ ] Prompt para detección de prioridad
- [ ] Prompt para relación con proyectos
- [ ] Templates parametrizables
- [ ] Versioning de prompts

**Ejemplo de Prompt:**
```
Contexto: Usuario trabaja en proyecto "{project_name}"
Tecnologías: {tech_stack}

Información capturada:
---
{captured_text}
---

Extrae los action items de esta información.
Responde SOLO con JSON válido:
{
  "action_items": [
    {
      "title": "string",
      "priority": "low|medium|high|urgent",
      "type": "task|bug|feature|meeting_note",
      "estimated_time": "string (opcional)"
    }
  ]
}
```

**Archivos Afectados:**
- `src-tauri/src/processing/prompts/mod.rs`
- `src-tauri/src/processing/prompts/action_items.rs`
- `src-tauri/src/processing/prompts/categorization.rs`
- `src-tauri/src/processing/prompts/summary.rs`

**Decisiones Requeridas:**
- [ ] Formato de respuesta de IA (JSON strict vs flexible)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.6: Procesador de Action Items

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar el procesador que extrae action items de texto capturado.

**Criterios de Aceptación:**
- [ ] Recibe texto de captura (OCR, commit, etc.)
- [ ] Envía a Ollama con prompt apropiado
- [ ] Parsea respuesta JSON
- [ ] Valida estructura de action items
- [ ] Crea tareas en base de datos
- [ ] Relaciona con proyecto activo
- [ ] Manejo de errores de parsing

**Archivos Afectados:**
- `src-tauri/src/processing/action_items.rs`
- `src-tauri/src/processing/parser.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.7: Categorizador de Proyectos

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar lógica para asignar automáticamente capturas y tareas a proyectos.

**Criterios de Aceptación:**
- [ ] Análisis de contexto (URL, archivo activo, etc.)
- [ ] Matching por keywords en nombre de proyecto
- [ ] Matching por path de archivos
- [ ] Scoring de confianza para asignación
- [ ] Fallback a "Sin Proyecto" si confianza baja
- [ ] Opción de reasignar manualmente

**Archivos Afectados:**
- `src-tauri/src/processing/categorizer.rs`
- `src-tauri/src/processing/project_matcher.rs`

**Decisiones Requeridas:**
- [ ] Umbral de confianza mínimo para asignación automática

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.8: Generador de Resúmenes

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar generación automática de resúmenes de reuniones.

**Criterios de Aceptación:**
- [ ] Agrupa capturas de una misma reunión
- [ ] Genera resumen coherente con IA
- [ ] Extrae participantes si están visibles
- [ ] Extrae decisiones principales
- [ ] Extrae action items
- [ ] Almacena como entrada tipo "meeting_note"

**Archivos Afectados:**
- `src-tauri/src/processing/meeting_summary.rs`
- `src-tauri/src/processing/aggregator.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.9: Detector de Prioridades

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar detección automática de prioridad basada en contexto.

**Criterios de Aceptación:**
- [ ] Keywords de urgencia: "ASAP", "urgent", "critical", "blocker"
- [ ] Keywords de baja prioridad: "later", "eventually", "nice to have"
- [ ] Análisis de contexto con IA
- [ ] Detección de deadlines mencionados
- [ ] Asignación de prioridad: low, medium, high, urgent

**Archivos Afectados:**
- `src-tauri/src/processing/priority.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.10: Pipeline Completo de Procesamiento

**Estado:** `[ ] Pendiente`

**Descripción:**
Integrar todos los componentes en un pipeline unificado.

**Criterios de Aceptación:**
- [ ] Captura → Filtro Sensible → Cola
- [ ] Cola → Procesador Action Items → DB
- [ ] Cola → Categorizador → Asignación Proyecto
- [ ] Cola → Priorización → Actualización
- [ ] Eventos emitidos a frontend (WebSocket/IPC)
- [ ] Logging completo del pipeline
- [ ] Métricas de procesamiento

**Flujo:**
```
1. Captura llega (OCR, commit, actividad)
2. Filtro de info sensible
3. Si limpio → encolar
4. Worker toma de cola
5. Extrae action items con IA
6. Categoriza por proyecto
7. Asigna prioridad
8. Almacena en DB
9. Notifica a frontend
```

**Archivos Afectados:**
- `src-tauri/src/processing/pipeline.rs`
- `src-tauri/src/processing/mod.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.11: UI - Indicadores de Procesamiento

**Estado:** `[ ] Pendiente`

**Descripción:**
Mostrar estado de procesamiento IA en la UI.

**Criterios de Aceptación:**
- [ ] Indicador de cola (X items pendientes)
- [ ] Spinner cuando está procesando
- [ ] Notificación toast cuando se crea tarea
- [ ] Historial de procesamiento reciente
- [ ] Errores de procesamiento visibles

**Archivos Afectados:**
- `src/lib/components/ProcessingStatus.svelte`
- `src/lib/stores/processing.ts`
- `src/lib/components/Toast.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.12: Comandos Tauri para Tareas

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar comandos Tauri para CRUD de tareas.

**Criterios de Aceptación:**
- [ ] `create_task` - crear tarea manual
- [ ] `get_tasks` - listar tareas con filtros
- [ ] `get_task_by_id` - obtener tarea específica
- [ ] `update_task` - actualizar tarea
- [ ] `delete_task` - eliminar tarea
- [ ] `complete_task` - marcar como completada
- [ ] Filtros: por proyecto, por estado, por fecha

**Archivos Afectados:**
- `src-tauri/src/commands/tasks.rs`
- `src-tauri/src/main.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 3.13: Vista de Tareas por Proyecto

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar vista de tareas dentro de cada proyecto.

**Criterios de Aceptación:**
- [ ] Lista de tareas del proyecto
- [ ] Filtros por estado (pending, in_progress, completed)
- [ ] Ordenamiento por prioridad, fecha
- [ ] Click para ver/editar tarea
- [ ] Drag & drop para cambiar estado (opcional)
- [ ] Indicador de tareas automáticas vs manuales

**Archivos Afectados:**
- `src/routes/projects/[id]/+page.svelte`
- `src/lib/components/TaskList.svelte`
- `src/lib/components/TaskCard.svelte`
- `src/lib/components/TaskFilters.svelte`

**Decisiones Requeridas:**
- [ ] Implementar drag & drop en MVP o postergar

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Resumen de Fase 3

| Tarea | Estado | Code Review |
|-------|--------|-------------|
| 3.1 Cliente Ollama | [ ] | [ ] |
| 3.2 Gestión de Modelos | [ ] | [ ] |
| 3.3 Cola de Procesamiento | [ ] | [ ] |
| 3.4 Detector Info Sensible | [ ] | [ ] |
| 3.5 Prompts de Procesamiento | [ ] | [ ] |
| 3.6 Procesador Action Items | [ ] | [ ] |
| 3.7 Categorizador Proyectos | [ ] | [ ] |
| 3.8 Generador Resúmenes | [ ] | [ ] |
| 3.9 Detector Prioridades | [ ] | [ ] |
| 3.10 Pipeline Completo | [ ] | [ ] |
| 3.11 UI Indicadores | [ ] | [ ] |
| 3.12 Comandos Tauri Tareas | [ ] | [ ] |
| 3.13 Vista Tareas Proyecto | [ ] | [ ] |

**Progreso:** 0/13 tareas completadas
