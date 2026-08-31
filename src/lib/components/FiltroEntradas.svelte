<script lang="ts">
  import Search from '@lucide/svelte/icons/search';
  import TipoChips from './TipoChips.svelte';
  import { LIMITES } from '$lib/limites';

  interface Props {
    busqueda: string;
    tipo: string | null;
    extras?: string[];
    placeholder?: string;
    onbusqueda: (q: string) => void;
    ontipo: (nombre: string | null) => void;
  }

  let {
    busqueda,
    tipo,
    extras = [],
    placeholder = 'Filtrar por título…',
    onbusqueda,
    ontipo
  }: Props = $props();
</script>

<div class="barra">
  <div class="buscar">
    <span class="icono"><Search size={14} strokeWidth={1.5} /></span>
    <input
      value={busqueda}
      maxlength={LIMITES.busqueda}
      {placeholder}
      oninput={(e) => onbusqueda((e.currentTarget as HTMLInputElement).value)}
    />
  </div>
  <TipoChips value={tipo} nullLabel="todos" {extras} onchange={ontipo} />
</div>

<style>
  .barra {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
  }
  .buscar {
    position: relative;
    flex: 1;
    min-width: 180px;
    max-width: 320px;
  }
  .icono {
    position: absolute;
    left: 10px;
    top: 8px;
    color: var(--apagado);
    display: flex;
  }
  input {
    width: 100%;
    box-sizing: border-box;
    font-family: var(--font-ui);
    font-size: 13px;
    padding: 7px 12px 7px 32px;
    border: 1px solid var(--borde-fuerte);
    border-radius: var(--radio-m);
    background: var(--superficie);
    outline: none;
  }
</style>
