<script lang="ts">
  interface Item {
    label: string;
    action: () => void;
    danger?: boolean;
    disabled?: boolean;
    separator?: boolean;
  }
  interface Props {
    items: Item[];
    x: number;
    y: number;
    onclose: () => void;
  }
  let { items, x, y, onclose }: Props = $props();

  let menuEl: HTMLDivElement | null = $state(null);
  let posX = $state(x);
  let posY = $state(y);

  $effect(() => {
    if (!menuEl) return;
    const rect = menuEl.getBoundingClientRect();
    if (x + rect.width > window.innerWidth) posX = window.innerWidth - rect.width - 8;
    if (y + rect.height > window.innerHeight) posY = window.innerHeight - rect.height - 8;
  });

  function onDoc(e: MouseEvent) {
    if (!menuEl?.contains(e.target as Node)) onclose();
  }
  function onKey(e: KeyboardEvent) {
    if (e.key === 'Escape') onclose();
  }
  $effect(() => {
    document.addEventListener('mousedown', onDoc);
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('mousedown', onDoc);
      document.removeEventListener('keydown', onKey);
    };
  });
</script>

<div class="menu" bind:this={menuEl} style:left="{posX}px" style:top="{posY}px" role="menu">
  {#each items as it, i (i)}
    {#if it.separator}
      <div class="sep"></div>
    {:else}
      <button
        type="button"
        class="item"
        class:danger={it.danger}
        disabled={it.disabled}
        onclick={() => { it.action(); onclose(); }}
      >
        {it.label}
      </button>
    {/if}
  {/each}
</div>

<style>
  .menu {
    position: fixed;
    z-index: 100;
    min-width: 180px;
    background: var(--superficie);
    border: 1px solid var(--borde);
    border-radius: var(--radio-m);
    box-shadow: var(--sombra-flotante);
    padding: 4px;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .item {
    background: transparent;
    border: none;
    text-align: left;
    padding: 6px 10px;
    border-radius: var(--radio-s);
    font-family: var(--font-ui);
    font-size: 12px;
    color: var(--tinta);
    cursor: pointer;
  }
  .item:hover:not(:disabled) { background: var(--superficie-2); }
  .item:disabled { color: var(--apagado); cursor: default; }
  .item.danger { color: var(--error, #C05246); }
  .item.danger:hover:not(:disabled) { background: var(--error-suave, #F8EEEC); }
  .sep { height: 1px; background: var(--borde); margin: 4px 0; }
</style>
