<script lang="ts">
  import { parsearContenido } from '$lib/contenido';

  interface Props {
    contenido: string;
  }
  let { contenido }: Props = $props();

  const bloques = $derived(parsearContenido(contenido));
</script>

<div class="vista">
  {#each bloques as b, i (i)}
    {#if b.kind === 'codigo'}
      <figure class="code">
        <figcaption>{b.lang}</figcaption>
        <pre><code>{b.valor}</code></pre>
      </figure>
    {:else}
      <div class="prosa">{b.valor}</div>
    {/if}
  {/each}
</div>

<style>
  .vista { display: flex; flex-direction: column; gap: 12px; min-width: 0; }
  .prosa {
    font-size: 15px;
    line-height: 1.6;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    word-break: break-word;
  }
  .code {
    margin: 0;
    background: var(--tinta);
    color: #EDEDE8;
    border-radius: var(--radio-l);
    overflow: hidden;
    min-width: 0;
  }
  .code figcaption {
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.08em;
    color: #B8B6AE;
    padding: 8px 12px 0;
    text-transform: lowercase;
  }
  .code pre {
    margin: 0;
    padding: 8px 12px 12px;
    overflow-x: auto;
    overflow-y: hidden;
  }
  .code code {
    font-family: var(--font-mono);
    font-size: 12px;
    line-height: 1.55;
    white-space: pre;
  }
</style>
