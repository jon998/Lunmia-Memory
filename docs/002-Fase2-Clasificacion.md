# Fase 2 — Clasificación

**Duración estimada:** 3 semanas
**Criterio de salida (PRD §15.1):** 80% de precisión tras 200 entradas reales.

---

## Tarea 2.1: Cascada de capas 0-3 (§6.2)

**Estado:** `[x] Andamiaje entregado`

**Criterios de aceptación:**
- [x] Capa 0: contexto activo desde tabla de frecuencias (`ai/context.rs`)
- [x] Capa 1: reglas ES en `ai/rules.rs` (30 patrones de fecha + verbos)
- [x] Capa 2: embeddings vs centroides con `bge-m3` (`ai/embeddings.rs`)
- [x] Capa 3: LLM pequeño con `qwen2.5:3b` (`ai/classifier.rs`)
- [x] Orquestador en `ai/pipeline.rs` con `UMBRAL_ALTA_CONFIANZA = 0.72`
- [ ] Validar precisión >= 80% sobre corpus real (requiere uso real del usuario)

- [ ] **Code Review por Usuario**

---

## Tarea 2.2: Taxonomía dinámica (§5.4)

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] Tabla `tipos` con `centroide_embedding`, `usos_total`, `archivado`
- [x] `upsert_tipo` idempotente
- [ ] Fusión de centroides que se solapan (Fase 2.3)
- [ ] Archivado automático de tipos sin uso (Fase 2.3)
- [ ] Alarma cuando supera 12 tipos activos

- [ ] **Code Review por Usuario**

---

## Tarea 2.3: Contexto activo aprendido (§5.5)

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] Tabla `contexto_frecuencias` (día_semana, franja_hora, espacio, proyecto)
- [x] `registrar_frecuencia_contexto` en cada captura confirmada
- [x] `proponer_contexto_por_hora` como capa 0
- [ ] UI para "cambiar contexto activo" desde el sidebar (visible pero no editable)

- [ ] **Code Review por Usuario**

---

## Tarea 2.4: Colecciones no destructivas (§5.1)

**Estado:** `[ ] Pendiente`

**Criterios de aceptación:**
- [ ] Tabla `colecciones` con consulta JSON
- [ ] Componente `ColeccionCard.svelte`
- [ ] Botón "Convertir en proyecto"

- [ ] **Code Review por Usuario**
