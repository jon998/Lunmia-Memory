<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import Kbd from './Kbd.svelte';
  import Button from './Button.svelte';
  import ContextSwitcher from './ContextSwitcher.svelte';
  import LimiteHint from './LimiteHint.svelte';
  import TipoChips from './TipoChips.svelte';
  import LlmStatus from './LlmStatus.svelte';
  import type { Contexto } from '$lib/types';
  import { LIMITES } from '$lib/limites';

  interface Props {
    contexto: Contexto | null;
    extrasTipos?: string[];
    consejos?: string[];
    llmActivo?: boolean | null;
    llmAviso?: string | null;
    onguardar: (texto: string, tipoNombre: string | null) => Promise<void> | void;
    oncambiarContexto?: (ctx: Contexto) => void;
  }

  let {
    contexto,
    extrasTipos = [],
    consejos = [
      "¿Qué no quieres olvidar? \"recordar llamar a Ana el lunes\"…",
      "Prueba: \"leer el libro que recomendó Marcos\"…",
      "Prueba: \"idea: modo oscuro automático al atardecer\"…",
      "Prueba: \"enviar propuesta al cliente el viernes a las 10\"…"
    ],
    llmActivo = null,
    llmAviso = null,
    onguardar,
    oncambiarContexto
  }: Props = $props();

  let borrador = $state('');
  let tipoNombre = $state<string | null>(null);
  let iConsejo = $state(0);
  let timer: ReturnType<typeof setInterval> | null = null;
  let textareaEl: HTMLTextAreaElement | null = null;

  onMount(() => {
    timer = setInterval(() => { iConsejo = (iConsejo + 1) % consejos.length; }, 4000);
  });
  onDestroy(() => { if (timer) clearInterval(timer); });

  async function capturar() {
    const texto = borrador.trim();
    if (!texto) return;
    const tipo = tipoNombre;
    borrador = '';
    tipoNombre = null;
    await onguardar(texto, tipo);
    textareaEl?.focus();
  }

  function tecla(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); capturar(); }
  }
</script>

<div class="box">
  <textarea
    bind:this={textareaEl}
    bind:value={borrador}
    onkeydown={tecla}
    placeholder={consejos[iConsejo]}
    maxlength={LIMITES.captura}
    rows="2"
  ></textarea>
  <div class="tipos">
    <TipoChips value={tipoNombre} extras={extrasTipos} onchange={(t) => tipoNombre = t} />
    <span class="hint-tipo">{tipoNombre ? `queda como ${tipoNombre}` : (llmActivo === false ? 'sin IA · se guarda tal cual' : 'si no eliges, la IA clasifica')}</span>
    <span class="llm"><LlmStatus activo={llmActivo} aviso={llmAviso} /></span>
  </div>
  <div class="pie">
    <ContextSwitcher {contexto} onchange={(c) => oncambiarContexto?.(c)} />
    <span class="hint">contexto activo · clic para cambiar</span>
    <LimiteHint valor={borrador} max={LIMITES.captura} />
    <span class="teclas">
      <Kbd label="guardar">↵</Kbd>
      <span class="sep"></span>
      <Kbd label="salto de línea">⇧↵</Kbd>
    </span>
    <Button variant="primary" onclick={capturar}>Capturar</Button>
  </div>
</div>

<style>
  .box {
    background: var(--superficie);
    border: 1.5px solid var(--tinta);
    border-radius: var(--radio-l);
    padding: 14px 16px;
    box-shadow: var(--sombra-captura);
  }
  textarea {
    width: 100%;
    box-sizing: border-box;
    font-family: var(--font-ui);
    font-size: 15px;
    line-height: 1.5;
    border: none;
    background: transparent;
    outline: none;
    resize: none;
    min-height: 44px;
  }
  .tipos {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 8px;
    flex-wrap: wrap;
  }
  .hint-tipo {
    font-size: 11px;
    color: var(--apagado);
  }
  .llm { margin-left: auto; }
  .pie {
    display: flex;
    gap: 8px;
    align-items: center;
    margin-top: 8px;
    flex-wrap: wrap;
  }
  .hint {
    font-size: 11px;
    color: var(--apagado);
    white-space: nowrap;
  }
  .teclas {
    margin-left: auto;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    color: var(--texto-2);
  }
  .sep { width: 8px; }
</style>
