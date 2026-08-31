# Fase 0 — Prototipo de riesgo: scheduler de recordatorios

**Duración estimada:** 1 semana
**Criterio de salida (PRD §15.1):** un recordatorio dispara de forma fiable 20 veces seguidas con la aplicación cerrada.

> Esta fase es innegociable y va primero. El riesgo **R1** (§13) es existencial: si los recordatorios no disparan con la aplicación cerrada, no hay producto que construir.

---

## Tarea 0.1: Prototipo del scheduler con la app cerrada

**Estado:** `[x] Entregado en el refactor MVP1`

**Descripción:**
Programar recordatorios en el sistema (macOS `UNNotificationRequest` via `tauri-plugin-notification`) de forma que dispare aunque el proceso de Tauri no esté vivo.

**Criterios de aceptación:**
- [x] Tabla `recordatorios` persistida en SQLite (`db/schema.sql`)
- [x] Worker en `scheduler/mod.rs` que hace tick cada 30 s cuando la app está viva
- [x] Reconciliación al arrancar: recordatorios vencidos con `disparado_at IS NULL` disparan una vez
- [ ] **Pruebas manuales — 20 disparos con la app cerrada** (usuario)
- [x] Registrar `disparado_at` para no repetir

**Archivos afectados:**
- `src-tauri/src/scheduler/mod.rs`
- `src-tauri/src/db/schema.sql` (tabla `recordatorios`)
- `src-tauri/src/db/queries.rs` (`recordatorios_por_disparar`, `marcar_recordatorio_disparado`)

**Decisiones tomadas:**
- Se usa `tauri-plugin-notification` (delega en `UNUserNotificationCenter`) porque programa la notificación en el sistema, no en el proceso Tauri.
- Backoff: si el sistema tarda en entregar, el próximo tick reconcilia.

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

---

## Tarea 0.2: Instrumentación mínima desde el día uno

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] Tabla `metricas_operacion` que registra latencias por operación
- [x] Tabla `metricas_clasificacion` que registra capa/confianza/correcciones
- [x] Log con `tracing` inicializado con `EnvFilter`

- [ ] **Code Review por Usuario**

---

## Notas de implementación

El scheduler queda listo, pero la validación real (§13 R1) requiere pruebas manuales en un macOS con la app cerrada. Ese es el gate para salir a Fase 1.
