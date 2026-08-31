# Fase 4 — Pulido

**Duración estimada:** 2 semanas
**Criterio de salida (PRD §15.1):** 21 días de uso continuo del autor sin que el inbox supere 20 elementos sin triage.

---

## Tarea 4.1: Perfiles automáticos de modelo por RAM (§9)

**Estado:** `[ ] Pendiente`

**Criterios de aceptación:**
- [ ] Detectar RAM total via `sysctl -n hw.memsize`
- [ ] Elegir Ligero (<= 8 GB), Equilibrado (16 GB), Completo (32 GB+)
- [ ] Descarga silenciosa desde onboarding, primera captura funcional antes de terminar

- [ ] **Code Review por Usuario**

---

## Tarea 4.2: OCR real con Vision (macOS)

**Estado:** `[ ] Andamiaje · FFI pendiente`

**Criterios de aceptación:**
- [x] Adaptador `MacOcr` toma screenshot con `screencapture -i`
- [ ] FFI a `VNRecognizeTextRequest` para extraer texto (~40 ms)
- [x] Imagen se descarta inmediatamente después (§6.4)
- [ ] D4: aviso al usuario cuando OCR no extrae texto útil

- [ ] **Code Review por Usuario**

---

## Tarea 4.3: Captura por voz con Speech (D1)

**Estado:** `[ ] Andamiaje`

**Criterios de aceptación:**
- [x] Trait `SpeechRecognizer` con implementación stub
- [ ] FFI a `SFSpeechRecognizer` para transcripción en dispositivo
- [ ] UI: barra de captura de voz de 10-20 s

- [ ] **Code Review por Usuario**

---

## Tarea 4.4: Bot de Telegram como canal móvil (D2)

**Estado:** `[ ] Pendiente`

**Criterios de aceptación:**
- [ ] Módulo `telegram/mod.rs` con long-polling
- [ ] Config `config.telegram_token`
- [ ] Al abrir la app, descargar mensajes retenidos (24h)
- [ ] Limitado a texto en MVP1

- [ ] **Code Review por Usuario**

---

## Tarea 4.5: Métricas de salud (§12.3)

**Estado:** `[x] Instrumentación entregada`

**Criterios de aceptación:**
- [x] Latencia por operación registrada
- [x] Sin triage, provisionales, latencia media en stats
- [ ] Panel de salud interno (`/ajustes/metricas`) para consulta rápida
- [ ] Alarma si "> 70% Entradas en proyectos por defecto" (§5.2)
- [ ] Alarma si "tasa de descarte > 40%" (§6.6)

- [ ] **Code Review por Usuario**
