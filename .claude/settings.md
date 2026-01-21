# Claude Code Settings - Lunmia Memory

Este archivo contiene las reglas específicas para el desarrollo de Lunmia Memory.

## Referencia Principal
Ver [CLAUDE.md](../CLAUDE.md) en la raíz del proyecto para las reglas completas.

## Resumen de Reglas

### Workflow Estricto
1. **NO avanzar** a otra tarea sin marcar la actual como completada `[x]`
2. **NO avanzar** a otra fase sin completar TODAS las tareas de la fase actual
3. Cada tarea tiene **Code Review por Usuario** - NO continuar sin esta marca

### Estructura de Tareas
```
docs/
├── 001-Fase1-SetupCore.md      (12 tareas)
├── 002-Fase2-AgentesMCP.md     (15 tareas)
├── 003-Fase3-IAProcesamiento.md (13 tareas)
├── 004-Fase4-Busqueda.md       (9 tareas)
├── 005-Fase5-UXOnboarding.md   (14 tareas)
└── 006-Fase6-TestingPolish.md  (16 tareas)
```

### Consultas Obligatorias
Antes de decisiones drásticas sobre:
- Arquitectura
- Dependencias principales
- Modelo de datos
- Seguridad

**DEBE consultarse al usuario.**

### Verificación Antes de Trabajar
1. Leer archivo de fase actual
2. Verificar tarea anterior completada `[x]`
3. Verificar Code Review marcado `[x]`
4. Solo entonces proceder

## Comandos Útiles
```bash
# Ver tareas pendientes
grep -r "\[ \]" docs/*.md | head -20

# Ver progreso por fase
grep "Progreso:" docs/*.md
```
