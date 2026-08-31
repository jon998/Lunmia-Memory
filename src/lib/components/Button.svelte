<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  type Variant = 'primary' | 'secondary' | 'ghost' | 'indigo' | 'danger';
  type Size = 'md' | 'sm';

  interface Props extends HTMLButtonAttributes {
    variant?: Variant;
    size?: Size;
    children?: Snippet;
  }

  let {
    variant = 'primary',
    size = 'md',
    children,
    class: klass = '',
    ...rest
  }: Props = $props();
</script>

<button {...rest} class="btn {variant} {size} {klass}">
  {@render children?.()}
</button>

<style>
  .btn {
    font-family: var(--font-ui);
    font-weight: 600;
    font-size: 13px;
    border-radius: var(--radio-m);
    padding: 7px 14px;
    border: 1px solid transparent;
    background: transparent;
    color: var(--tinta);
    line-height: 1.2;
    display: inline-flex;
    align-items: center;
    gap: 6px;
    white-space: nowrap;
    transition: background 0.12s ease, border-color 0.12s ease;
  }
  .sm { padding: 5px 10px; font-size: 12px; }

  .primary { background: var(--tinta); color: var(--papel); }
  .primary:hover:not(:disabled) { background: var(--tinta-hover); }

  .secondary {
    background: transparent;
    color: var(--texto-2);
    border-color: var(--borde-fuerte);
    font-weight: 500;
  }
  .secondary:hover:not(:disabled) { background: var(--superficie-2); }

  .ghost {
    color: var(--texto-2);
    font-weight: 500;
  }
  .ghost:hover:not(:disabled) { background: var(--superficie-2); }

  .indigo {
    background: var(--indigo-suave);
    color: var(--indigo);
    border-color: var(--indigo-borde);
  }
  .indigo:hover:not(:disabled) { background: #E2E0F6; }

  .danger {
    background: transparent;
    color: var(--error);
    border-color: var(--error-borde);
    font-weight: 500;
  }
  .danger:hover:not(:disabled) { background: var(--error-suave); }
</style>
