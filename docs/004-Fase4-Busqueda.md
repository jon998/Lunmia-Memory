# Fase 4: Búsqueda Inteligente

**Duración Estimada:** 1 semana  
**Objetivo:** Embeddings generados, Búsqueda semántica funcional, Filtros y sorting

**Dependencias:** Fase 3 completada

---

## Tarea 4.1: Generación de Embeddings

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar generación de embeddings para tareas y capturas usando Ollama.

**Criterios de Aceptación:**
- [ ] Integración con modelo nomic-embed-text de Ollama
- [ ] Función para generar embedding de texto
- [ ] Almacenamiento de embeddings en tabla `embeddings`
- [ ] Embeddings generados al crear tarea/captura
- [ ] Batch processing para contenido existente
- [ ] Dimensionalidad consistente (768 dimensiones)

**Archivos Afectados:**
- `src-tauri/src/search/embeddings.rs`
- `src-tauri/src/search/mod.rs`
- `src-tauri/src/services/ollama/embeddings.rs`

**Decisiones Requeridas:**
- [ ] Modelo de embeddings (nomic-embed-text vs alternativas)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 4.2: Índice de Búsqueda

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar índice para búsqueda rápida por similitud de vectores.

**Criterios de Aceptación:**
- [ ] Algoritmo de similitud coseno implementado
- [ ] Índice en memoria para búsquedas rápidas
- [ ] Carga de índice al iniciar aplicación
- [ ] Actualización incremental del índice
- [ ] Performance < 100ms para 50k vectores

**Archivos Afectados:**
- `src-tauri/src/search/index.rs`
- `src-tauri/src/search/similarity.rs`

**Decisiones Requeridas:**
- [ ] Librería de vectores (hnsw, faiss-rs, o custom)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 4.3: Full-Text Search

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar búsqueda de texto completo usando SQLite FTS5.

**Criterios de Aceptación:**
- [ ] Tabla FTS5 creada para tasks y captures
- [ ] Triggers para mantener FTS sincronizado
- [ ] Búsqueda por keywords
- [ ] Ranking de resultados por relevancia
- [ ] Highlighting de términos encontrados
- [ ] Performance < 50ms para 50k documentos

**SQL para FTS:**
```sql
CREATE VIRTUAL TABLE tasks_fts USING fts5(
    title, 
    description, 
    content='tasks', 
    content_rowid='rowid'
);

CREATE TRIGGER tasks_ai AFTER INSERT ON tasks BEGIN
    INSERT INTO tasks_fts(rowid, title, description) 
    VALUES (new.rowid, new.title, new.description);
END;
```

**Archivos Afectados:**
- `src-tauri/src/db/migrations/002_fts.sql`
- `src-tauri/src/search/fulltext.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 4.4: Motor de Búsqueda Híbrido

**Estado:** `[ ] Pendiente`

**Descripción:**
Combinar búsqueda semántica y full-text para resultados óptimos.

**Criterios de Aceptación:**
- [ ] Función de búsqueda híbrida
- [ ] Pesos configurables (semántica vs keyword)
- [ ] Fusion de resultados con deduplicación
- [ ] Score combinado para ranking
- [ ] Fallback a keyword si embeddings no disponibles

**Algoritmo:**
```
1. Query llega
2. Generar embedding de query
3. Búsqueda semántica → resultados con score A
4. Búsqueda full-text → resultados con score B
5. Fusionar: score_final = (0.6 * A) + (0.4 * B)
6. Ordenar por score_final descendente
7. Retornar top N
```

**Archivos Afectados:**
- `src-tauri/src/search/hybrid.rs`
- `src-tauri/src/search/fusion.rs`

**Decisiones Requeridas:**
- [ ] Pesos para fusión (60/40, 70/30, configurable)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 4.5: API de Búsqueda

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar comandos Tauri para búsqueda desde frontend.

**Criterios de Aceptación:**
- [ ] Comando `search` con query string
- [ ] Parámetros de filtro: proyecto, tipo, fecha, estado
- [ ] Parámetro de límite de resultados
- [ ] Respuesta incluye snippets con highlighting
- [ ] Metadata de búsqueda (tiempo, total de resultados)

**Estructura de Respuesta:**
```json
{
  "results": [
    {
      "id": "task-123",
      "type": "task",
      "title": "Fix auth bug",
      "snippet": "...need to **fix** the **auth** issue...",
      "score": 0.92,
      "project": "lunmia-core",
      "created_at": 1705234567890
    }
  ],
  "meta": {
    "total": 42,
    "took_ms": 87,
    "query": "fix auth"
  }
}
```

**Archivos Afectados:**
- `src-tauri/src/commands/search.rs`
- `src-tauri/src/search/types.rs`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 4.6: UI - Vista Memory

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar la vista de Memory/Búsqueda completa.

**Criterios de Aceptación:**
- [ ] Barra de búsqueda prominente
- [ ] Resultados en tiempo real (debounced 300ms)
- [ ] Cards de resultados con highlighting
- [ ] Filtros laterales/superiores
- [ ] Estado vacío cuando no hay resultados
- [ ] Loading state mientras busca

**Archivos Afectados:**
- `src/routes/memory/+page.svelte`
- `src/lib/components/SearchBar.svelte`
- `src/lib/components/SearchResults.svelte`
- `src/lib/components/SearchResultCard.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 4.7: Filtros de Búsqueda

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar sistema de filtros para refinar búsquedas.

**Criterios de Aceptación:**
- [ ] Filtro por proyecto (dropdown multiselect)
- [ ] Filtro por tipo (task, meeting, commit, note)
- [ ] Filtro por rango de fechas (date picker)
- [ ] Filtro por estado (pending, in_progress, completed)
- [ ] Filtro por prioridad
- [ ] Filtros persisten en URL (query params)
- [ ] Botón "Clear filters"

**Archivos Afectados:**
- `src/lib/components/SearchFilters.svelte`
- `src/lib/components/DateRangePicker.svelte`
- `src/lib/components/MultiSelect.svelte`
- `src/routes/memory/+page.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 4.8: Vista de Detalle de Resultado

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar vista detallada al hacer click en un resultado de búsqueda.

**Criterios de Aceptación:**
- [ ] Modal o página de detalle
- [ ] Muestra información completa de tarea/captura
- [ ] Contexto relacionado (otras tareas del proyecto)
- [ ] Acciones: editar, eliminar, cambiar estado
- [ ] Link al proyecto padre
- [ ] Historial de cambios (si aplica)

**Archivos Afectados:**
- `src/lib/components/DetailModal.svelte`
- `src/lib/components/TaskDetail.svelte`
- `src/lib/components/CaptureDetail.svelte`

**Decisiones Requeridas:**
- [ ] Modal vs página separada para detalle

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Tarea 4.9: Búsqueda Rápida (Command Palette)

**Estado:** `[ ] Pendiente`

**Descripción:**
Implementar búsqueda rápida accesible desde cualquier pantalla.

**Criterios de Aceptación:**
- [ ] Atajo de teclado Cmd/Ctrl+K abre palette
- [ ] Búsqueda instantánea mientras escribe
- [ ] Navegación con flechas arriba/abajo
- [ ] Enter para ir al resultado
- [ ] Escape para cerrar
- [ ] Resultados agrupados por tipo

**Archivos Afectados:**
- `src/lib/components/CommandPalette.svelte`
- `src/lib/stores/commandPalette.ts`
- `src/routes/+layout.svelte`

**Decisiones Requeridas:**
- Ninguna

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
<!-- Completar después de implementar -->

---

## Resumen de Fase 4

| Tarea | Estado | Code Review |
|-------|--------|-------------|
| 4.1 Generación Embeddings | [ ] | [ ] |
| 4.2 Índice de Búsqueda | [ ] | [ ] |
| 4.3 Full-Text Search | [ ] | [ ] |
| 4.4 Motor Híbrido | [ ] | [ ] |
| 4.5 API de Búsqueda | [ ] | [ ] |
| 4.6 UI Vista Memory | [ ] | [ ] |
| 4.7 Filtros de Búsqueda | [ ] | [ ] |
| 4.8 Vista Detalle | [ ] | [ ] |
| 4.9 Command Palette | [ ] | [ ] |

**Progreso:** 0/9 tareas completadas
