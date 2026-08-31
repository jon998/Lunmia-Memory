<script lang="ts">
  import type { Snippet } from 'svelte';

  type Variant = 'default' | 'indigo' | 'ok' | 'alerta' | 'error' | 'neutral';

  interface Props {
    variant?: Variant;
    mono?: boolean;
    dot?: string;
    onclick?: (e: MouseEvent) => void;
    title?: string;
    class?: string;
    children?: Snippet;
  }

  let {
    variant = 'default',
    mono = false,
    dot,
    onclick,
    title,
    class: klass = '',
    children
  }: Props = $props();
</script>

<span
  role={onclick ? 'button' : undefined}
  tabindex={onclick ? 0 : undefined}
  {onclick}
  {title}
  class="chip {variant} {klass}"
  class:mono
  class:clic={!!onclick}
>
  {#if dot}<span class="dot" style:background={dot}></span>{/if}
  {@render children?.()}
</span>

<style>
  .chip {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    border-radius: var(--radio-s);
    padding: 3px 8px;
    background: var(--superficie-2);
    border: 1px solid var(--borde);
    color: var(--tinta);
    white-space: nowrap;
    line-height: 1.4;
  }
  .mono { font-family: var(--font-mono); }
  .clic { cursor: pointer; }
  .dot {
    width: 8px;
    height: 8px;
    border-radius: 2px;
    display: inline-block;
    flex-shrink: 0;
  }
  .indigo {
    background: var(--indigo-suave);
    color: var(--indigo);
    border-color: var(--indigo-borde);
  }
  .ok {
    background: var(--ok-suave);
    color: var(--ok-texto);
    border-color: var(--ok-borde);
    font-weight: 500;
  }
  .alerta {
    background: var(--alerta-suave);
    color: var(--alerta-texto);
    border-color: var(--alerta-borde);
    font-weight: 500;
  }
  .error {
    background: var(--error-suave);
    color: var(--error);
    border-color: var(--error-borde);
    font-weight: 500;
  }
  .neutral {
    background: var(--superficie-2);
    color: var(--texto-2);
    border-color: var(--borde);
  }
</style>
