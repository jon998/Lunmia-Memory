# Claude Code Settings — Lunmia Memory

Reglas específicas del proyecto. Ver [`CLAUDE.md`](../CLAUDE.md) para el conjunto completo.

## Fases (roadmap del PRD §15.1)

```
docs/
├── PRD-lunmia-memory-mvp1.md   Especificación cerrada
├── 000-Fase0-Scheduler.md      Prototipo de riesgo — innegociable
├── 001-Fase1-Captura.md        Atajo global, ventana flotante, persistencia
├── 002-Fase2-Clasificacion.md  Cascada 0-3, taxonomía, contexto
├── 003-Fase3-Bandeja.md        Resolución asistida, búsqueda
└── 004-Fase4-Pulido.md         Modelos, OCR real, voz, Telegram, métricas
```

## Reglas duras

1. **NO avanzar** a otra tarea sin marcar la actual como completada `[x]`
2. **NO avanzar** a otra fase sin completar todas las tareas de la fase actual
3. Cada tarea tiene **Code Review por Usuario** — NO continuar sin esta marca
4. **Nunca** modificar `contenido_original` (§5.3)
5. La IA **propone, nunca aplica** (§5.1)
6. **UUIDv7 + updated_at + tombstones** en toda tabla nueva (§5.6)

## Consultas obligatorias

Antes de decisiones sobre:
- Arquitectura del proyecto (adaptadores de plataforma, esquema)
- Dependencias principales del stack
- Cambios al modelo de datos
- Configuración de seguridad
- Cambios al pipeline de clasificación

**DEBE consultarse al usuario.**

## Comandos rápidos

```bash
# Estado del proyecto
grep -r "\[ \]" docs/*.md | head -20   # tareas pendientes
grep -r "\[x\]" docs/*.md | head -20   # completadas

# Desarrollo
pnpm tauri dev
cd src-tauri && cargo check
```
