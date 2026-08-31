<script lang="ts">
  interface Props { valor: string | number; etiqueta: string; color?: string }
  let { valor, etiqueta, color = 'var(--tinta)' }: Props = $props();
</script>

<div class="card stat">
  <div class="valor" style:color>{valor}</div>
  <div class="etiqueta">{etiqueta}</div>
</div>

<style>
  .stat { padding: 10px 14px; }
  .valor { font-family: var(--font-mono); font-size: 20px; line-height: 1.1; }
  .etiqueta { font-size: 11px; color: var(--apagado); margin-top: 3px; }
</style>
