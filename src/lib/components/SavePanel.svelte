<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import Button from './Button.svelte';
  import Chip from './Chip.svelte';
  import AgendaField from './AgendaField.svelte';
  import type { Espacio, Proyecto } from '$lib/types';
  import { agendaVacia, type AgendaCivil } from '$lib/agenda';
  import { estiloTipo } from '$lib/tipos';
  import { LIMITES } from '$lib/limites';

  interface Props {
    resumen: string;
    espacios: Espacio[];
    proyectosDe: (espacioId: string) => Proyecto[];
    espacioId: string;
    proyectoId: string;
    tipoNombre?: string | null;
    etiquetas?: string[];
    agendaInicial?: AgendaCivil | null;
    sinIa?: boolean;
    esCapturaPantalla?: boolean;
    autocierre?: number;
    onguardar: (opts: {
      espacioId: string;
      proyectoId: string;
      prompt: string | null;
      agenda: AgendaCivil | null;
      agendaTocada: boolean;
    }) => void | Promise<void>;
    onclose: () => void;
  }

  let {
    resumen,
    espacios,
    proyectosDe,
    espacioId = $bindable(),
    proyectoId = $bindable(),
    tipoNombre = null,
    etiquetas = [],
    agendaInicial = null,
    sinIa = false,
    esCapturaPantalla = false,
    autocierre = 10,
    onguardar,
    onclose
  }: Props = $props();

  const duracionMs = Math.max(1, autocierre) * 1000;
  let prompt = $state('');
  let restanteMs = $state(duracionMs);
  let hover = $state(false);
  let foco = $state(false);
  let agenda = $state<AgendaCivil>(agendaInicial ?? agendaVacia());
  let agendaTocada = $state(false);

  const pausado = $derived(hover || foco);
  const fraccion = $derived(Math.max(0, restanteMs / duracionMs));
  const segundos = $derived(Math.max(0, Math.ceil(restanteMs / 1000)));

  let raf = 0;
  let ultimo = 0;
  let cerrado = false;
  let reanudarT: ReturnType<typeof setTimeout> | null = null;

  onMount(() => {
    if (esCapturaPantalla) return;
    ultimo = performance.now();
    raf = requestAnimationFrame(tick);
  });

  onDestroy(() => {
    cerrado = true;
    if (raf) cancelAnimationFrame(raf);
    if (reanudarT) clearTimeout(reanudarT);
  });

  function tick(now: number) {
    if (cerrado) return;
    if (!pausado) {
      restanteMs = Math.max(0, restanteMs - (now - ultimo));
      if (restanteMs <= 0) {
        void confirmar();
        return;
      }
    }
    ultimo = now;
    raf = requestAnimationFrame(tick);
  }

  function entrar() {
    if (reanudarT) { clearTimeout(reanudarT); reanudarT = null; }
    hover = true;
  }

  function salir() {
    // Gracia breve: el <select> nativo dispara mouseleave al abrir.
    reanudarT = setTimeout(() => { hover = false; reanudarT = null; }, 180);
  }

  function focoDentro() { foco = true; }
  function focoFuera(e: FocusEvent) {
    const dest = e.relatedTarget as Node | null;
    if (dest && (e.currentTarget as HTMLElement).contains(dest)) return;
    foco = false;
  }

  async function confirmar() {
    if (cerrado) return;
    cerrado = true;
    if (raf) cancelAnimationFrame(raf);
    await onguardar({
      espacioId,
      proyectoId,
      prompt: prompt.trim() || null,
      agenda: agendaTocada ? agenda : null,
      agendaTocada
    });
    onclose();
  }

  function descartar() {
    cerrado = true;
    if (raf) cancelAnimationFrame(raf);
    onclose();
  }

  $effect(() => {
    const proyectos = proyectosDe(espacioId);
    if (!proyectos.some((p) => p.proyectoId === proyectoId) && proyectos.length > 0) {
      proyectoId = proyectos[0].proyectoId;
    }
  });
</script>

<div
  class="panel"
  onmouseenter={entrar}
  onmouseleave={salir}
  onfocusin={focoDentro}
  onfocusout={focoFuera}
>
  {#if !esCapturaPantalla}
    <div class="mecha" class:pausado aria-hidden="true">
      <div class="mecha-fill" style:transform="scaleX({fraccion})"></div>
    </div>
  {/if}
  <div class="cabecera">
    <Chip variant="ok">Guardada ✓</Chip>
    {#if sinIa}
      <Chip variant="alerta" title="El modelo no respondió. El texto quedó como lo escribiste.">sin IA</Chip>
    {/if}
    {#if tipoNombre}
      <Chip variant={estiloTipo(tipoNombre)}>{tipoNombre}</Chip>
    {/if}
    {#if esCapturaPantalla}
      <Chip variant="alerta">espera tu OK — la imagen ya se borró</Chip>
    {:else if pausado}
      <Chip variant="alerta">pausa</Chip>
    {/if}
    <button type="button" class="cerrar" onclick={descartar} aria-label="Cerrar">✕</button>
  </div>
  <div class="resumen">{resumen}</div>
  {#if sinIa}
    <div class="aviso-ia">Sin conexión con el modelo. El texto quedó como lo escribiste — sin clasificar ni reescribir.</div>
  {/if}
  {#if etiquetas.length > 0}
    <div class="tags">
      {#each etiquetas as t}
        <Chip variant="neutral" mono>#{t}</Chip>
      {/each}
    </div>
  {/if}
  {#if esCapturaPantalla}
    <div class="ocr-nota">texto extraído por OCR · Vision · 40ms</div>
  {/if}
    <AgendaField bind:value={agenda} compact oninteract={() => (agendaTocada = true)} />
  <div class="selectores">
    <select bind:value={espacioId}>
      {#each espacios as e (e.espacioId)}
        <option value={e.espacioId}>{e.nombre}</option>
      {/each}
    </select>
    <select bind:value={proyectoId}>
      {#each proyectosDe(espacioId) as p (p.proyectoId)}
        <option value={p.proyectoId}>{p.nombre}</option>
      {/each}
    </select>
  </div>
  <input
    bind:value={prompt}
    class="prompt"
    maxlength={LIMITES.prompt}
    placeholder="…o corrígelo con tus palabras: 'esto es personal, para el sábado'"
  />
  {#if esCapturaPantalla}
    <div class="acciones">
      <Button variant="primary" onclick={confirmar}>Confirmar</Button>
      <Button variant="danger" onclick={descartar}>Descartar</Button>
    </div>
  {:else}
    <div class="pie">
      <span class="nota">
        {#if pausado}
          cursor encima — el cierre se detiene
        {:else}
          se guarda solo en {segundos}s
        {/if}
      </span>
      <Button variant="primary" onclick={confirmar}>Listo ✓</Button>
    </div>
  {/if}
</div>

<style>
  .panel {
    position: relative;
    width: 100%;
    max-width: 100%;
    max-height: min(80vh, 620px);
    overflow-x: hidden;
    overflow-y: auto;
    background: transparent;
    border: none;
    border-radius: 0;
    box-shadow: none;
    padding: 8px 6px 4px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    min-width: 0;
    animation: lm-slide 0.2s ease-out;
  }
  @keyframes lm-slide {
    from { opacity: 0; transform: translateY(6px); }
    to   { opacity: 1; transform: translateY(0); }
  }
  .mecha {
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 3px;
    background: var(--borde);
    overflow: hidden;
    border-radius: var(--radio-xl) var(--radio-xl) 0 0;
  }
  .mecha-fill {
    height: 100%;
    width: 100%;
    background: var(--indigo);
    transform-origin: left center;
    will-change: transform;
  }
  .mecha.pausado .mecha-fill { background: var(--alerta); }
  .cabecera { display: flex; align-items: center; gap: 8px; min-width: 0; }
  .cerrar {
    margin-left: auto;
    background: transparent;
    border: none;
    color: var(--apagado);
    font-size: 14px;
    cursor: pointer;
    line-height: 1;
    flex-shrink: 0;
  }
  .cerrar:hover { color: var(--tinta); }
  .resumen {
    font-size: 13px;
    line-height: 1.5;
    color: var(--tinta);
    max-height: 7.5em;
    overflow-x: hidden;
    overflow-y: auto;
    padding-right: 4px;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    word-break: break-word;
    min-width: 0;
    max-width: 100%;
  }
  .ocr-nota { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); }
  .aviso-ia {
    font-family: var(--font-mono);
    font-size: 11px;
    line-height: 1.4;
    color: var(--alerta-texto);
    background: var(--alerta-suave);
    border: 1px solid var(--alerta-borde);
    border-radius: var(--radio-m);
    padding: 7px 10px;
  }
  .tags { display: flex; flex-wrap: wrap; gap: 4px; }
  .selectores { display: flex; gap: 8px; min-width: 0; }
  select {
    flex: 1;
    min-width: 0;
    font-family: var(--font-ui);
    font-size: 12px;
    padding: 6px 8px;
    border: 1px solid var(--borde-fuerte);
    border-radius: var(--radio-m);
    background: var(--superficie);
    color: var(--tinta);
    outline: none;
  }
  .prompt {
    width: 100%;
    box-sizing: border-box;
    font-family: var(--font-ui);
    font-size: 12px;
    padding: 7px 10px;
    border: 1px solid var(--borde);
    border-radius: var(--radio-m);
    background: var(--superficie);
    color: var(--tinta);
    outline: none;
  }
  .acciones { display: flex; gap: 8px; }
  .pie {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }
  .nota {
    flex: 1;
    min-width: 0;
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--apagado);
    line-height: 1.35;
  }
</style>
