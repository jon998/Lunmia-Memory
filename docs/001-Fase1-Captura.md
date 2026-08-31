# Fase 1 — Captura

**Duración estimada:** 2 semanas
**Criterio de salida (PRD §15.1):** captura funcional en menos de 3 s desde cualquier aplicación.

Referencias UI: `Fase 1 - Onboarding.dc.html`, `Fase 2 - Home.dc.html`, `Fase 3 - Captura Flotante.dc.html`.

---

## Tarea 1.1: Atajo global del sistema (`⌘⇧Space`)

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] Registrar `Cmd+Shift+Space` con `tauri-plugin-global-shortcut`
- [x] Al disparar: mostrar y enfocar la ventana `captura` sin robar el foco del resto
- [x] Configurable desde `config.atajo_captura`
- [ ] Alternativa "doble Option" (fuera del MVP1 inmediato)

**Archivos afectados:**
- `src-tauri/src/lib.rs` (`registrar_atajo_predeterminado`, `toggle_ventana_captura`)
- `src-tauri/tauri.conf.json` (ventana `captura` con `alwaysOnTop`, sin decoración)

- [ ] **Code Review por Usuario**

---

## Tarea 1.2: Ventana flotante de captura (§7.1)

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] Ventana `captura` sin decoración, siempre encima, no en el Dock
- [x] Ruta `/captura` que reproduce el diseño de `Fase 3 - Captura Flotante.dc.html`
- [x] Selector de contexto con filtro (`Selector.svelte`)
- [x] `Enter` guarda, `Shift+Enter` salta línea, `Esc` cancela

**Archivos afectados:**
- `src/routes/captura/+page.svelte`
- `src/lib/components/Selector.svelte`

- [ ] **Code Review por Usuario**

---

## Tarea 1.3: Icono en la barra de menú con contador (§7.1)

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] Tray icon (template) con menú: Abrir Lunmia · Capturar · Bandeja · Salir
- [x] Click izquierdo abre la ventana principal
- [x] Emit `navegar` para navegar a `/bandeja` desde el menú

- [ ] **Code Review por Usuario**

---

## Tarea 1.4: Persistencia de la Entrada (§5.3)

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] `contenido_original` inmutable, `contenido` editable
- [x] UUIDv7 en `entry_id`, `created_at`/`updated_at`, borrado suave
- [x] Comando `capturar_texto` guarda primero y clasifica en background (§6.3)

- [ ] **Code Review por Usuario**

---

## Tarea 1.5: Onboarding generativo (§8)

**Estado:** `[x] Entregado en el refactor MVP1`

**Criterios de aceptación:**
- [x] 4 pasos: bienvenida, oficio, atajo, permisos + autostart
- [x] Ventana `onboarding` dedicada
- [x] Termina con `finalizar_onboarding` que persiste tipos propuestos
- [x] Fallback local `proponer_tipos_iniciales` para no bloquear por descarga de Ollama (§9 R4)

- [ ] **Code Review por Usuario**
