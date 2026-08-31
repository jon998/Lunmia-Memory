<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Option {
    value: string;
    label: string;
    icon?: Snippet;
  }

  interface Props {
    options: Option[];
    value: string;
    onchange: (value: string) => void;
  }

  let { options, value = $bindable(), onchange }: Props = $props();
</script>

<div class="wrap" role="tablist">
  {#each options as opt (opt.value)}
    <button
      type="button"
      role="tab"
      aria-selected={value === opt.value}
      class:active={value === opt.value}
      onclick={() => { value = opt.value; onchange(opt.value); }}
    >
      {#if opt.icon}{@render opt.icon()}{/if}
      {opt.label}
    </button>
  {/each}
</div>

<style>
  .wrap {
    display: inline-flex;
    background: var(--superficie-2);
    border: 1px solid var(--borde);
    border-radius: var(--radio-m);
    padding: 2px;
    gap: 2px;
    flex-shrink: 0;
  }
  button {
    background: transparent;
    color: var(--apagado);
    border: none;
    border-radius: var(--radio-s);
    padding: 4px 12px;
    font-family: var(--font-ui);
    font-weight: 500;
    font-size: 12px;
    display: inline-flex;
    align-items: center;
    gap: 6px;
    white-space: nowrap;
  }
  button.active {
    background: var(--superficie);
    color: var(--tinta);
  }
</style>
