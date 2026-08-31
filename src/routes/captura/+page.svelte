<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import Kbd from '$lib/components/Kbd.svelte';
  import ContextSwitcher from '$lib/components/ContextSwitcher.svelte';
  import SavePanel from '$lib/components/SavePanel.svelte';
  import LimiteHint from '$lib/components/LimiteHint.svelte';
  import TipoChips from '$lib/components/TipoChips.svelte';
  import LlmStatus from '$lib/components/LlmStatus.svelte';
  import { actualizarContenido, capturarTexto, cerrarVentanaCaptura, isTauri, moverEntrada, recentrarVentanaCaptura } from '$lib/tauri';
  import { persistirAgenda, type AgendaCivil } from '$lib/agenda';
  import { capturarConInforme, type InformeCaptura } from '$lib/informe-captura';
  import { app } from '$lib/stores/app.svelte';
  import { LIMITES } from '$lib/limites';

  let borrador = $state('');
  let tipoNombre = $state<string | null>(null);
  let textareaEl: HTMLTextAreaElement | null = $state(null);
  let unlistenAbrir: (() => void) | undefined;

  const extrasTipos = $derived(app.tipos.map((t) => t.nombre));

  let resumen = $state<InformeCaptura | null>(null);
  let clasificando = $state(false);
  let capturaGen = 0;

  onMount(async () => {
    try {
      await app.hidratarDestinos();
    } catch (err) {
      console.error(err);
    }
    void app.comprobarLlm(true);
    setTimeout(() => textareaEl?.focus(), 20);

    if (isTauri()) {
      const { listen } = await import('@tauri-apps/api/event');
      unlistenAbrir = await listen('captura:abrir', () => {
        borrador = '';
        tipoNombre = null;
        resumen = null;
        clasificando = false;
        capturaGen += 1;
        void ajustarTamano('input');
        void app.hidratarDestinos().catch((err) => console.error(err));
        void app.comprobarLlm(true);
        setTimeout(() => textareaEl?.focus(), 20);
      });
    }

    document.addEventListener('keydown', teclaGlobal);
  });

  onDestroy(() => {
    unlistenAbrir?.();
    document.removeEventListener('keydown', teclaGlobal);
  });

  function teclaGlobal(e: KeyboardEvent) {
    if (e.key === 'Escape' && !resumen) {
      capturaGen += 1;
      clasificando = false;
      cerrar();
    }
  }

  async function ajustarTamano(modo: 'input' | 'resumen') {
    if (!isTauri()) return;
    try {
      const { getCurrentWindow, LogicalSize } = await import('@tauri-apps/api/window');
      const win = getCurrentWindow();
      if (modo === 'input') await win.setSize(new LogicalSize(620, 200));
      else await win.setSize(new LogicalSize(480, 560));
      await recentrarVentanaCaptura();
    } catch (err) {
      console.warn('no se pudo redimensionar', err);
    }
  }

  async function guardar() {
    const texto = borrador.trim();
    if (!texto || clasificando) return;
    if (!app.contexto || app.espacios.length === 0) {
      try { await app.hidratarDestinos(); } catch (err) { console.error(err); }
    }
    const espacioId = app.contexto?.espacioId ?? '';
    const proyectoId = app.contexto?.proyectoId ?? '';
    const tipo = tipoNombre;
    if (!espacioId || !proyectoId) {
      try {
        await capturarTexto(texto, { tipoNombre: tipo ?? undefined });
      } catch (err) { console.error(err); }
      borrador = '';
      tipoNombre = null;
      cerrar();
      return;
    }
    const gen = ++capturaGen;
    clasificando = true;
    borrador = '';
    tipoNombre = null;
    try {
      const informe = await capturarConInforme(texto, { espacioId, proyectoId, tipoNombre: tipo ?? undefined });
      if (gen !== capturaGen) return;
      resumen = {
        ...informe,
        espacioId: informe.espacioId || espacioId,
        proyectoId: informe.proyectoId || proyectoId
      };
      await ajustarTamano('resumen');
    } catch (err) {
      console.error(err);
      if (gen === capturaGen) cerrar();
    } finally {
      if (gen === capturaGen) clasificando = false;
    }
  }

  function tecla(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); guardar(); }
  }

  function cerrar() {
    if (isTauri()) {
      void ajustarTamano('input');
      void cerrarVentanaCaptura();
    } else {
      window.history.back();
    }
  }

  async function confirmarResumen(opts: {
    espacioId: string;
    proyectoId: string;
    prompt: string | null;
    agenda: AgendaCivil | null;
    agendaTocada: boolean;
  }) {
    if (!resumen) return;
    const { entryId } = resumen;
    if (opts.espacioId !== resumen.espacioId || opts.proyectoId !== resumen.proyectoId) {
      try { await moverEntrada(entryId, opts.espacioId, opts.proyectoId); }
      catch (err) { console.error(err); }
    }
    if (opts.prompt) {
      try {
        await actualizarContenido(entryId, `${resumen.contenido}\n\n[corrección]: ${opts.prompt}`);
      } catch (err) { console.error(err); }
    }
    try {
      await persistirAgenda({
        entryId,
        espacioId: opts.espacioId,
        proyectoId: opts.proyectoId,
        titulo: resumen.contenido.split('\n')[0]?.slice(0, 72) || 'Recordatorio',
        agenda: opts.agenda,
        borrarSiInactivo: opts.agendaTocada
      });
    } catch (err) { console.error(err); }
  }

  function cerrarResumen() {
    resumen = null;
    cerrar();
  }
</script>

<div class="wrap">
  {#if clasificando}
    <div class="flotante pensando">
      <div class="cabecera">
        <img src="/assets/logo.png" alt="Lunmia" width="20" height="20" />
        <span class="titulo">CAPTURA RÁPIDA</span>
      </div>
      <div class="espera">
        <span class="pulso"></span>
        {#if app.llmActivo === false}
          Guardada · sin IA, el texto queda tal cual
        {:else}
          Guardada · Lunmia está clasificando…
        {/if}
      </div>
    </div>
  {:else if !resumen}
    <div class="flotante">
      <div class="cabecera">
        <img src="/assets/logo.png" alt="Lunmia" width="20" height="20" />
        <span class="titulo">CAPTURA RÁPIDA</span>
        <span class="llm"><LlmStatus activo={app.llmActivo} aviso={app.llmAviso} /></span>
        <div class="switcher">
          <ContextSwitcher
            contexto={app.contexto}
            align="right"
            compact
            onchange={() => setTimeout(() => textareaEl?.focus(), 20)}
          />
        </div>
      </div>
      <textarea
        bind:this={textareaEl}
        bind:value={borrador}
        onkeydown={tecla}
        placeholder="Escribe y suéltalo. Lunmia lo guarda antes de pensar…"
        maxlength={LIMITES.captura}
        rows="2"
      ></textarea>
      <div class="tipos">
        <TipoChips value={tipoNombre} extras={extrasTipos} onchange={(t) => tipoNombre = t} />
      </div>
      <div class="pie">
        <span class="teclas">
          <Kbd label="guardar">↵</Kbd>
          <span class="sep"></span>
          <Kbd label="cancelar">esc</Kbd>
        </span>
        <LimiteHint valor={borrador} max={LIMITES.captura} />
        <span class="hint">{tipoNombre ? `queda como ${tipoNombre}` : (app.llmActivo === false ? 'sin IA · se guarda tal cual' : 'tipo opcional · si no eliges, Lunmia clasifica sola')}</span>
      </div>
    </div>
  {:else}
    <div class="wrap-panel">
      <SavePanel
        resumen={resumen.contenido}
        espacios={app.espacios}
        proyectosDe={(id) => app.proyectosDe(id)}
        espacioId={resumen.espacioId}
        proyectoId={resumen.proyectoId}
        tipoNombre={resumen.tipoNombre}
        etiquetas={resumen.etiquetas}
        agendaInicial={resumen.agenda}
        sinIa={resumen.sinIa}
        onguardar={confirmarResumen}
        onclose={cerrarResumen}
      />
    </div>
  {/if}
</div>

<style>
  :global(html), :global(body) {
    background: transparent !important;
  }
  /* Redefine los tokens SOLO para esta ventana, adaptándose al vibrancy
     del sistema: si macOS está en light, colores oscuros; si dark, claros.
     Los componentes hijos (SavePanel, AgendaField, chips…) heredan estas
     variables y quedan legibles sobre cristal claro u oscuro. */
  .wrap {
    --papel: transparent;
    --superficie: rgba(255, 255, 255, 0.55);
    --superficie-2: rgba(0, 0, 0, 0.04);
    --tinta: rgba(0, 0, 0, 0.90);
    --tinta-hover: rgba(0, 0, 0, 0.75);
    --texto-2: rgba(0, 0, 0, 0.62);
    --apagado: rgba(0, 0, 0, 0.48);
    --borde: rgba(0, 0, 0, 0.10);
    --borde-fuerte: rgba(0, 0, 0, 0.18);
    --indigo-suave: rgba(79, 70, 201, 0.20);
  }
  @media (prefers-color-scheme: dark) {
    .wrap {
      --papel: transparent;
      --superficie: rgba(255, 255, 255, 0.10);
      --superficie-2: rgba(255, 255, 255, 0.06);
      --tinta: rgba(255, 255, 255, 0.94);
      --tinta-hover: rgba(255, 255, 255, 0.82);
      --texto-2: rgba(255, 255, 255, 0.72);
      --apagado: rgba(255, 255, 255, 0.52);
      --borde: rgba(255, 255, 255, 0.10);
      --borde-fuerte: rgba(255, 255, 255, 0.22);
      --indigo-suave: rgba(140, 130, 255, 0.28);
    }
  }
  .wrap {
    position: fixed;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    background: transparent;
    color: var(--tinta);
  }
  .flotante {
    width: 100%;
    max-width: 100%;
    background: transparent;
    border: none;
    border-radius: 0;
    box-shadow: none;
    padding: 4px 4px;
    position: relative;
    animation: lm-pop 0.16s ease-out;
  }
  .cabecera { display: flex; align-items: center; gap: 8px; margin-bottom: 10px; }
  .titulo { font-family: var(--font-mono); font-size: 10px; color: var(--texto-2); letter-spacing: 0.1em; }
  .llm { margin-left: auto; }
  .switcher { margin-left: 8px; }
  textarea {
    width: 100%;
    box-sizing: border-box;
    font-family: var(--font-ui);
    font-size: 16px;
    line-height: 1.5;
    border: none;
    background: transparent;
    outline: none;
    resize: none;
    min-height: 52px;
  }
  .tipos { margin-top: 4px; }
  .pie {
    display: flex; align-items: center; gap: 10px;
    margin-top: 6px;
    flex-wrap: wrap;
  }
  .teclas { display: inline-flex; align-items: center; gap: 8px; font-size: 11px; color: var(--texto-2); }
  .sep { width: 8px; }
  .hint {
    margin-left: auto;
    font-size: 11px; color: var(--texto-2);
    white-space: nowrap;
  }
  .wrap-panel {
    width: 100%;
    max-width: 420px;
    max-height: 100%;
    overflow-x: hidden;
    overflow-y: auto;
    display: flex;
    justify-content: center;
    min-width: 0;
  }
  .wrap-panel :global(.panel) { width: 100%; max-width: 420px; }
  .espera {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: 52px;
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--texto-2);
  }
  .pulso {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--indigo);
    flex-shrink: 0;
    animation: lm-pulso 1s ease-in-out infinite;
  }
  @keyframes lm-pulso {
    0%, 100% { opacity: 0.35; transform: scale(0.85); }
    50% { opacity: 1; transform: scale(1); }
  }
</style>
