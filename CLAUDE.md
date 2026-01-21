# Lunmia Memory - Reglas de Desarrollo

## Workflow de Tareas

### Estructura de Archivos de Tareas
Las tareas se encuentran en `/docs/` con el formato:
- `001-Fase1-SetupCore.md`
- `002-Fase2-AgentesMCP.md`
- `003-Fase3-IAProcesamiento.md`
- `004-Fase4-Busqueda.md`
- `005-Fase5-UXOnboarding.md`
- `006-Fase6-TestingPolish.md`

### Reglas Obligatorias

#### 1. Flujo Secuencial Estricto
- **NUNCA** avances a otra tarea sin marcar la actual como completada `[x]`
- **NUNCA** avances a otra fase sin completar TODAS las tareas de la fase actual
- Cada tarea tiene una casilla de **Code Review** `[ ] Code Review por Usuario` que DEBE ser marcada por el usuario antes de continuar

#### 2. Casilla de Code Review
Cada tarea incluye:
```markdown
- [ ] **Code Review por Usuario** (NO continuar sin esta marca)
```
**IMPORTANTE:** Esta casilla la marca ÚNICAMENTE el usuario. Si no está marcada `[x]`, NO procedas con las siguientes tareas.

#### 3. Consultas en Decisiones Drásticas
Antes de tomar decisiones que afecten:
- Arquitectura del proyecto
- Dependencias principales
- Cambios en el modelo de datos
- Integraciones con servicios externos
- Cambios de configuración de seguridad

**DEBES** consultar al usuario utilizando preguntas claras y esperar confirmación.

#### 4. Verificación de Estado
Antes de trabajar en cualquier tarea:
1. Lee el archivo de la fase actual
2. Verifica que la tarea anterior esté completada `[x]`
3. Verifica que el Code Review de la tarea anterior esté marcado `[x]`
4. Solo entonces procede con la nueva tarea

### Formato de Tareas

Cada tarea sigue este formato:
```markdown
### Tarea X.Y: [Nombre de la Tarea]

**Estado:** `[ ] Pendiente` | `[x] Completada`

**Descripción:**
[Descripción detallada]

**Criterios de Aceptación:**
- [ ] Criterio 1
- [ ] Criterio 2

**Archivos Afectados:**
- `ruta/archivo.ts`

**Decisiones Requeridas:**
- [ ] Decisión 1 (consultar usuario)

- [ ] **Code Review por Usuario** (NO continuar sin esta marca)

**Notas de Implementación:**
[Notas post-implementación]
```

### Comandos Útiles

Para verificar el estado actual del proyecto:
```bash
# Ver tareas pendientes
grep -r "\[ \]" docs/*.md

# Ver tareas completadas
grep -r "\[x\]" docs/*.md
```

## Referencias
- PRD Principal: `/docs/PRD-LUNMIA-MEMORY.md`
- UI/UX Assets: `/docs/ui/`

## Stack Tecnológico
- Frontend: SvelteKit 2 + Svelte 5
- Desktop: Tauri 2
- Backend: Rust
- Database: SQLite
- IA: Ollama + Qwen2.5-Coder
- OCR: Tesseract
