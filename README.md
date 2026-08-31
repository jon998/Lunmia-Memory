# Lunmia Memory

App de escritorio para **macOS** que captura ideas, notas, recordatorios y capturas de pantalla en **menos de 3 segundos** y los clasifica con IA que corre en la Mac del usuario.

Promesa: **nada se pierde**.

Diferenciador: captura sin fricción sobre cualquier aplicación · procesamiento local (nada sale a la nube) · taxonomía que evoluciona con el uso real, no impuesta desde el día uno.

## Documentación

- **[PRD MVP 1](docs/PRD-lunmia-memory-mvp1.md)** — especificación cerrada del producto
- **[CLAUDE.md](CLAUDE.md)** — reglas de desarrollo
- **Fases del roadmap** (docs/):
  - [Fase 0 — Scheduler](docs/000-Fase0-Scheduler.md) *(prototipo de riesgo, innegociable)*
  - [Fase 1 — Captura](docs/001-Fase1-Captura.md)
  - [Fase 2 — Clasificación](docs/002-Fase2-Clasificacion.md)
  - [Fase 3 — Bandeja](docs/003-Fase3-Bandeja.md)
  - [Fase 4 — Pulido](docs/004-Fase4-Pulido.md)

## Stack

- **Shell:** Tauri 2
- **Backend:** Rust (adaptadores de plataforma en traits)
- **Frontend:** SvelteKit 2 + Svelte 5 (runes) sobre Tauri
- **Persistencia:** SQLite con UUIDv7, `updated_at` y tombstones (§5.6 del PRD)
- **IA local:** Ollama · `bge-m3` (embeddings multilingües) · `qwen2.5:3b` (LLM pequeño)
- **OCR:** Vision framework de macOS (imagen se descarta al terminar)
- **Voz:** Speech framework de macOS *(D1 — pendiente)*

## Arranque

```bash
pnpm install
pnpm tauri dev
```

Requiere:
- macOS 12+
- Rust 1.75+
- Node 20+
- Un proveedor LLM (ver abajo)

### Proveedor por defecto: Ollama local

```bash
brew install ollama && ollama serve
ollama pull bge-m3       # embeddings multilingües
ollama pull qwen2.5:3b   # clasificador
```

### Proveedor de pruebas: Alibaba DashScope (Qwen cloud)

```bash
export LUNMIA_LLM_PROVIDER=dashscope
export LUNMIA_LLM_API_KEY=sk-...        # tu key de DashScope
# opcionales:
export LUNMIA_LLM_MODEL=qwen-flash      # default: qwen-flash
export LUNMIA_EMBED_MODEL=text-embedding-v3
export LUNMIA_LLM_BASE_URL=https://dashscope-intl.aliyuncs.com/compatible-mode/v1
pnpm tauri dev
```

> Nota: usar la nube contradice el "todo local" del PRD §1. Sólo para pruebas.

En modo dev sin proveedor la clasificación cae a la Capa 1 (reglas) y las entradas quedan en `pendiente_resolucion`.

## Resetear datos de desarrollo

Cierra la app (`Ctrl+C` en `tauri dev`) y borra la SQLite de desarrollo. Al volver a abrir, empieza el onboarding de cero.

```bash
pnpm reset:dev
pnpm tauri:dev
```

Equivale a:

```bash
rm -rf "$HOME/Library/Application Support/Lunmia Memory (dev)"
```

La carpeta incluye `lunmia.db` y los archivos WAL. No toca la BD de producción (`~/Library/Application Support/Lunmia Memory/`).

## Estructura

```
src/                       Frontend SvelteKit
  routes/                    /, /onboarding, /bandeja, /entrada/[id], /captura
  lib/
    components/              Design system (Button, Chip, EntryRow, …)
    stores/                  Estado global (Svelte 5 runes)
    styles/tokens.css        Tokens de "Papel Lunar"
    tauri.ts                 IPC tipado
    mock-adapter.ts          Datos falsos para dev en navegador

src-tauri/                 Backend Rust
  src/
    lib.rs                   Setup Tauri, tray, atajo, scheduler
    platform/                Traits + implementación macOS (§7.4)
    db/                      Esquema + queries + seed
    ai/                      Pipeline en cascada (§6.2)
    scheduler/               Recordatorios que disparan con la app cerrada
    ocr/                     Wrapper de Vision
    commands/                Comandos Tauri (IPC)

docs/                      PRD + fases + notas
```

## Autor

Yonatan
