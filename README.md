# Lunmia Memory

Gestor inteligente de tareas y proyectos con orquestación automática de IAs locales. 100% privado, 100% local.

## Documentación

- **[PRD Completo](docs/PRD-LUNMIA-MEMORY.md)** - Especificación del producto
- **[CLAUDE.md](CLAUDE.md)** - Reglas de desarrollo para Claude
- **Tasklists por Fase:**
  - [Fase 1: Setup y Core](docs/001-Fase1-SetupCore.md) - 12 tareas
  - [Fase 2: Agentes MCP](docs/002-Fase2-AgentesMCP.md) - 15 tareas
  - [Fase 3: IA y Procesamiento](docs/003-Fase3-IAProcesamiento.md) - 13 tareas
  - [Fase 4: Búsqueda](docs/004-Fase4-Busqueda.md) - 9 tareas
  - [Fase 5: UX y Onboarding](docs/005-Fase5-UXOnboarding.md) - 14 tareas
  - [Fase 6: Testing y Polish](docs/006-Fase6-TestingPolish.md) - 16 tareas

**Total: 79 tareas | Stack: SvelteKit + Tauri + Rust + Ollama**

---

# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


# Install
Template created! To get started run:
  cd "Lunmia Memory"
  pnpm install
  pnpm tauri android init
  pnpm tauri ios init

For Desktop development, run:
  pnpm tauri dev

For Android development, run:
  pnpm tauri android dev

For iOS development, run:
  pnpm tauri ios dev
by: Victor Yonatan, Maikol
