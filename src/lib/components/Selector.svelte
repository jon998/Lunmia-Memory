<script lang="ts">
  // Selector con filtro y scroll — patrón "los más usados, primero" (readme §Patrones).
  interface Option {
    value: string;
    label: string;
    color?: string;
    grupo?: string;
    subtitle?: string;
  }

  interface Props {
    options: Option[];
    value: string | null;
    placeholder?: string;
    filterPlaceholder?: string;
    footerHint?: string;
    onselect: (value: string) => void;
    onclose?: () => void;
  }

  let {
    options,
    value,
    placeholder = 'Filtrar…',
    filterPlaceholder = placeholder,
    footerHint = 'los más usados, primero',
    onselect,
    onclose
  }: Props = $props();

  let filtro = $state('');
  let inputEl: HTMLInputElement | null = $state(null);

  const filtradas = $derived.by(() => {
    const q = filtro.trim().toLowerCase();
    if (!q) return options;
    return options.filter((o) => o.label.toLowerCase().includes(q));
  });

  $effect(() => {
    inputEl?.focus();
  });
</script>

<div
  class="wrap"
  onkeydown={(e) => { if (e.key === 'Escape') onclose?.(); }}
  role="listbox"
  tabindex="-1"
>
  <input
    bind:this={inputEl}
    bind:value={filtro}
    placeholder={filterPlaceholder}
  />
  <div class="lista">
    {#each filtradas as opt (opt.value)}
      <button
        type="button"
        class="opt"
        class:sel={value === opt.value}
        onclick={() => onselect(opt.value)}
      >
        {#if opt.color}<span class="dot" style:background={opt.color}></span>{/if}
        <span class="lbl">{opt.label}</span>
        {#if opt.subtitle}<span class="sub">{opt.subtitle}</span>{/if}
        {#if value === opt.value}<span class="check">✓</span>{/if}
      </button>
    {/each}
    {#if filtradas.length === 0}
      <div class="vacio">{options.length === 0 ? 'No hay espacios todavía.' : 'Sin coincidencias.'}</div>
    {/if}
  </div>
  {#if footerHint}<div class="footer">{footerHint}</div>{/if}
</div>

<style>
  .wrap {
    background: var(--superficie);
    border: 1px solid var(--borde);
    border-radius: var(--radio-l);
    box-shadow: var(--sombra-flotante);
    padding: 4px;
    width: 260px;
    display: flex;
    flex-direction: column;
  }
  input {
    font-family: var(--font-ui);
    font-size: 12px;
    padding: 6px 9px;
    border: none;
    border-bottom: 1px solid var(--borde);
    background: transparent;
    outline: none;
    margin-bottom: 4px;
  }
  .lista { max-height: 220px; overflow-y: auto; display: flex; flex-direction: column; gap: 1px; }
  .opt {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 9px;
    border-radius: var(--radio-m);
    font-size: 12px;
    background: transparent;
    color: var(--tinta);
    border: none;
    text-align: left;
    width: 100%;
    cursor: pointer;
  }
  .opt:hover { background: var(--superficie-2); }
  .opt.sel { background: var(--indigo-suave); color: var(--indigo); }
  .dot { width: 8px; height: 8px; border-radius: 2px; flex-shrink: 0; }
  .lbl { flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .sub { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); }
  .check { font-family: var(--font-mono); font-size: 10px; color: var(--indigo); }
  .vacio { padding: 10px 9px; font-size: 11px; color: var(--apagado); }
  .footer {
    padding: 5px 9px;
    border-top: 1px solid var(--borde);
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--apagado);
  }
</style>
