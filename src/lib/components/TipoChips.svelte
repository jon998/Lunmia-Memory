<script lang="ts">
  import Plus from '@lucide/svelte/icons/plus';
  import { fusionarTipos, estiloTipo } from '$lib/tipos';
  import { app } from '$lib/stores/app.svelte';
  import { LIMITES } from '$lib/limites';

  interface Props {
    value: string | null;
    nullLabel?: string;
    extras?: string[];
    allowCreate?: boolean;
    onchange: (nombre: string | null) => void;
  }

  let {
    value,
    nullLabel = 'IA',
    extras = [],
    allowCreate = true,
    onchange
  }: Props = $props();

  const tipos = $derived(fusionarTipos(extras));

  let creando = $state(false);
  let nuevo = $state('');
  let error = $state('');
  let inputEl: HTMLInputElement | null = $state(null);

  $effect(() => {
    if (creando) inputEl?.focus();
  });

  function abrir() {
    creando = true;
    nuevo = '';
    error = '';
  }

  function cancelar() {
    creando = false;
    nuevo = '';
    error = '';
  }

  async function confirmar() {
    const nombre = nuevo.trim().toLowerCase();
    if (!nombre) {
      cancelar();
      return;
    }
    if (tipos.includes(nombre)) {
      creando = false;
      nuevo = '';
      onchange(nombre);
      return;
    }
    try {
      const t = await app.crearTipo(nombre);
      creando = false;
      nuevo = '';
      error = '';
      onchange(t.nombre);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    }
  }

  function tecla(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      e.stopPropagation();
      void confirmar();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      cancelar();
    }
  }
</script>

<div class="chips" role="listbox" aria-label="Tipo de entrada">
  <button
    type="button"
    class="chip auto"
    class:sel={value === null}
    role="option"
    aria-selected={value === null}
    title="Si no eliges, Lunmia clasifica sola"
    onclick={() => onchange(null)}
  >
    {nullLabel}
  </button>
  {#each tipos as t (t)}
    <button
      type="button"
      class="chip {estiloTipo(t)}"
      class:sel={value === t}
      role="option"
      aria-selected={value === t}
      onclick={() => onchange(value === t ? null : t)}
    >
      {t}
    </button>
  {/each}
  {#if allowCreate}
    {#if creando}
      <input
        bind:this={inputEl}
        bind:value={nuevo}
        class="nuevo"
        class:error={!!error}
        maxlength={LIMITES.nombre}
        placeholder="ej. cita"
        aria-label="Nombre del tipo nuevo"
        onkeydown={tecla}
        onblur={() => { if (!nuevo.trim()) cancelar(); }}
      />
    {:else}
      <button
        type="button"
        class="chip add"
        title="Añadir un tipo de clasificación"
        aria-label="Añadir tipo"
        onclick={abrir}
      >
        <Plus size={11} strokeWidth={2} />
      </button>
    {/if}
  {/if}
</div>
{#if error}
  <span class="err">{error}</span>
{/if}

<style>
  .chips {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
    align-items: center;
  }
  .chip {
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.04em;
    border-radius: 999px;
    padding: 3px 9px;
    background: transparent;
    border: 1px dashed var(--borde-fuerte);
    color: var(--apagado);
    cursor: pointer;
    line-height: 1.4;
  }
  .chip:hover { color: var(--tinta); border-color: var(--tinta); border-style: solid; }
  .chip.sel {
    border-style: solid;
    font-weight: 600;
  }
  .add {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    padding: 0;
    border-radius: 999px;
  }
  .add:hover { color: var(--indigo); border-color: var(--indigo); }
  .nuevo {
    width: 92px;
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.04em;
    padding: 3px 8px;
    border: 1px solid var(--indigo);
    border-radius: 999px;
    background: var(--superficie);
    color: var(--tinta);
    outline: none;
  }
  .nuevo.error { border-color: var(--error); }
  .err {
    display: block;
    margin-top: 4px;
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--error);
  }
  .auto.sel {
    background: var(--superficie-2);
    color: var(--tinta);
    border-color: var(--borde-fuerte);
  }
  .indigo.sel {
    background: var(--indigo-suave);
    color: var(--indigo);
    border-color: var(--indigo-borde);
  }
  .ok.sel {
    background: var(--ok-suave);
    color: var(--ok-texto);
    border-color: var(--ok-borde);
  }
  .error.sel {
    background: var(--error-suave);
    color: var(--error);
    border-color: var(--error-borde);
  }
  .alerta.sel {
    background: var(--alerta-suave);
    color: var(--alerta-texto);
    border-color: var(--alerta-borde);
  }
  .neutral.sel {
    background: var(--superficie-2);
    color: var(--tinta);
    border-color: var(--borde-fuerte);
  }
</style>
