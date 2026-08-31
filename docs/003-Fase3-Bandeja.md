# Fase 3 — Bandeja

**Duración estimada:** 2 semanas
**Criterio de salida (PRD §15.1):** tasa de descarte de preguntas por debajo del 40%.

---

## Tarea 3.1: Resolución asistida (§6.6)

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] Hasta 2 preguntas por Entrada, con botones fijos y texto libre
- [x] Teclas `1–3` para responder la primera pregunta pendiente
- [x] Presupuesto diario configurable (por defecto 10, `config.presupuesto_preguntas_diario`)
- [x] Descartar siempre disponible (vuelve al inbox)
- [ ] Umbral adaptativo por tasa de descarte del usuario

- [ ] **Code Review por Usuario**

---

## Tarea 3.2: Búsqueda semántica sobre `contenido` (§10)

**Estado:** `[x] UI listo · backend pendiente`

**Criterios de aceptación:**
- [x] Input de búsqueda en Home (`Fase 2` layout)
- [ ] `commands/buscar.rs` con embedding de la consulta + cosine sobre `entradas.embedding`
- [ ] Índice ANN opcional (deferido a Fase 4 según performance real)

- [ ] **Code Review por Usuario**

---

## Tarea 3.3: Panel post-guardado (§6.7)

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] Componente `SavePanel.svelte` con autocierre visible
- [x] Selectores prellenados de Espacio y Proyecto
- [x] Campo de prompt para corrección en lenguaje natural
- [x] Para capturas de pantalla, panel bloqueante (la imagen ya se borró)

- [ ] **Code Review por Usuario**
