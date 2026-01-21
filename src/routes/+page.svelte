<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let searchQuery = $state("");
    let memories: {
        id: number;
        title: string;
        preview: string;
        date: string;
    }[] = $state([
        {
            id: 1,
            title: "Primera nota",
            preview: "Esta es una nota de ejemplo...",
            date: "Hoy",
        },
        {
            id: 2,
            title: "Ideas del proyecto",
            preview: "Implementar sistema de tags...",
            date: "Ayer",
        },
        {
            id: 3,
            title: "Recordatorio",
            preview: "Revisar documentacion de Tauri...",
            date: "15 Ene",
        },
    ]);

    let selectedMemory = $state<number | null>(null);

    function selectMemory(id: number) {
        selectedMemory = id;
    }
</script>

<div class="app">
    <aside class="sidebar">
        <div class="logo">
            <span class="logo-icon">L</span>
            <span class="logo-text">Lunmia Memory</span>
        </div>

        <div class="search-box">
            <input
                type="text"
                placeholder="Buscar memorias..."
                bind:value={searchQuery}
            />
        </div>

        <button class="new-btn">+ Nueva Memoria</button>

        <nav class="nav-section">
            <h3>Recientes</h3>
            <ul class="memory-list">
                {#each memories as memory}
                    <li
                        class="memory-item"
                        class:active={selectedMemory === memory.id}
                        onclick={() => selectMemory(memory.id)}
                    >
                        <div class="memory-title">{memory.title}</div>
                        <div class="memory-meta">
                            <span class="memory-preview">{memory.preview}</span>
                            <span class="memory-date">{memory.date}</span>
                        </div>
                    </li>
                {/each}
            </ul>
        </nav>
    </aside>

    <main class="content">
        {#if selectedMemory}
            {@const current = memories.find((m) => m.id === selectedMemory)}
            <header class="content-header">
                <h1>{current?.title}</h1>
                <span class="date-badge">{current?.date}</span>
            </header>
            <div class="editor-area">
                <p class="placeholder-text">{current?.preview}</p>
            </div>
        {:else}
            <div class="empty-state">
                <div class="empty-icon">
                    <span>L</span>
                </div>
                <h2>Bienvenido a Lunmia Memory</h2>
                <p>Selecciona una memoria o crea una nueva para comenzar</p>
            </div>
        {/if}
    </main>
</div>

<style>
    :root {
        font-family:
            "Inter",
            -apple-system,
            BlinkMacSystemFont,
            "Segoe UI",
            sans-serif;
        font-size: 14px;
        line-height: 1.5;
        --primary: #6366f1;
        --primary-hover: #4f46e5;
        --bg-main: #fafafa;
        --bg-sidebar: #ffffff;
        --bg-hover: #f3f4f6;
        --text-primary: #111827;
        --text-secondary: #6b7280;
        --border: #e5e7eb;
    }

    * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    .app {
        display: flex;
        height: 100vh;
        background: var(--bg-main);
    }

    .sidebar {
        width: 280px;
        background: var(--bg-sidebar);
        border-right: 1px solid var(--border);
        display: flex;
        flex-direction: column;
        padding: 16px;
    }

    .logo {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 8px 0 20px;
    }

    .logo-icon {
        width: 32px;
        height: 32px;
        background: linear-gradient(135deg, var(--primary), #8b5cf6);
        border-radius: 8px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: white;
        font-weight: 700;
        font-size: 16px;
    }

    .logo-text {
        font-weight: 600;
        font-size: 16px;
        color: var(--text-primary);
    }

    .search-box input {
        width: 100%;
        padding: 10px 12px;
        border: 1px solid var(--border);
        border-radius: 8px;
        font-size: 13px;
        background: var(--bg-main);
        color: var(--text-primary);
        outline: none;
        transition: border-color 0.2s;
    }

    .search-box input:focus {
        border-color: var(--primary);
    }

    .new-btn {
        margin-top: 12px;
        padding: 10px 16px;
        background: var(--primary);
        color: white;
        border: none;
        border-radius: 8px;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .new-btn:hover {
        background: var(--primary-hover);
    }

    .nav-section {
        margin-top: 24px;
        flex: 1;
        overflow-y: auto;
    }

    .nav-section h3 {
        font-size: 11px;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        color: var(--text-secondary);
        margin-bottom: 8px;
        padding: 0 4px;
    }

    .memory-list {
        list-style: none;
    }

    .memory-item {
        padding: 10px 12px;
        border-radius: 8px;
        cursor: pointer;
        transition: background 0.15s;
        margin-bottom: 2px;
    }

    .memory-item:hover {
        background: var(--bg-hover);
    }

    .memory-item.active {
        background: #eef2ff;
    }

    .memory-title {
        font-weight: 500;
        color: var(--text-primary);
        margin-bottom: 4px;
        font-size: 13px;
    }

    .memory-meta {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .memory-preview {
        font-size: 12px;
        color: var(--text-secondary);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 150px;
    }

    .memory-date {
        font-size: 11px;
        color: var(--text-secondary);
    }

    .content {
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .content-header {
        padding: 20px 32px;
        border-bottom: 1px solid var(--border);
        display: flex;
        align-items: center;
        justify-content: space-between;
        background: var(--bg-sidebar);
    }

    .content-header h1 {
        font-size: 20px;
        font-weight: 600;
        color: var(--text-primary);
    }

    .date-badge {
        font-size: 12px;
        color: var(--text-secondary);
        background: var(--bg-hover);
        padding: 4px 10px;
        border-radius: 12px;
    }

    .editor-area {
        flex: 1;
        padding: 32px;
        overflow-y: auto;
    }

    .placeholder-text {
        color: var(--text-secondary);
        font-size: 15px;
        line-height: 1.7;
    }

    .empty-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: var(--text-secondary);
    }

    .empty-icon {
        width: 64px;
        height: 64px;
        background: linear-gradient(135deg, var(--primary), #8b5cf6);
        border-radius: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        margin-bottom: 20px;
    }

    .empty-icon span {
        color: white;
        font-size: 28px;
        font-weight: 700;
    }

    .empty-state h2 {
        color: var(--text-primary);
        font-size: 20px;
        margin-bottom: 8px;
    }

    .empty-state p {
        font-size: 14px;
    }

    @media (prefers-color-scheme: dark) {
        :root {
            --bg-main: #111827;
            --bg-sidebar: #1f2937;
            --bg-hover: #374151;
            --text-primary: #f9fafb;
            --text-secondary: #9ca3af;
            --border: #374151;
        }

        .memory-item.active {
            background: #312e81;
        }

        .search-box input {
            background: #111827;
        }
    }
</style>
