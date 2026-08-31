<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import Kbd from '$lib/components/Kbd.svelte';
  import Chip from '$lib/components/Chip.svelte';
  import Button from '$lib/components/Button.svelte';
  import { app } from '$lib/stores/app.svelte';
  import { descartarPregunta, responderPregunta } from '$lib/tauri';
  import type { TarjetaBandeja, PreguntaPendiente } from '$lib/types';

  const PRESUPUESTO = 10;
  let usadas = $state(4);

  let tarjetas = $state<TarjetaBandeja[]>([]);
  let estados = $state<Record<string, 'activa' | 'validando' | 'resuelta'>>({});
  let libres = $state<Record<string, { texto: string; confirmado: boolean }>>({});

  onMount(async () => {
    await app.cargarTodo();
    tarjetas = app.bandeja;
    for (const t of tarjetas) estados[t.entrada.entryId] = 'activa';
  });

  function marcarRespuesta(entryId: string, pregunta: PreguntaPendiente, opcion: number) {
    tarjetas = tarjetas.map((t) => {
      if (t.entrada.entryId !== entryId) return t;
      return {
        ...t,
        preguntas: t.preguntas.map((p) =>
          p.preguntaId === pregunta.preguntaId ? { ...p, respuestaOpcion: opcion, respuestaTexto: null } : p
        )
      };
    });
    void responderPregunta(pregunta.preguntaId, opcion, null);
  }

  function marcarLibre(entryId: string, pregunta: PreguntaPendiente, texto: string) {
    if (!texto.trim()) return;
    libres[pregunta.preguntaId] = { texto: texto.trim(), confirmado: true };
    void responderPregunta(pregunta.preguntaId, null, texto.trim());
  }

  function todasContestadas(t: TarjetaBandeja): boolean {
    return t.preguntas.every((p) => p.respuestaOpcion !== null || libres[p.preguntaId]?.confirmado);
  }

  async function procesar(t: TarjetaBandeja) {
    if (!todasContestadas(t)) return;
    estados[t.entrada.entryId] = 'validando';
    usadas = Math.min(usadas + t.preguntas.length, PRESUPUESTO);
    await new Promise((r) => setTimeout(r, 700));
    estados[t.entrada.entryId] = 'resuelta';
    await new Promise((r) => setTimeout(r, 900));
    tarjetas = tarjetas.filter((x) => x.entrada.entryId !== t.entrada.entryId);
  }

  async function descartar(t: TarjetaBandeja) {
    for (const p of t.preguntas) await descartarPregunta(p.preguntaId);
    tarjetas = tarjetas.filter((x) => x.entrada.entryId !== t.entrada.entryId);
  }

  const vacia = $derived(tarjetas.length === 0);
</script>

<div class="fondo">
  <div class="ventana">
    <div class="cuerpo">
      <div class="head">
        <div>
          <a href="/" class="volver-inicio">← Inicio</a>
          <div class="titulo">Resolución asistida</div>
          <div class="sub">Responde lo que quieras y procesa todo junto. Descartar nunca pierde nada.</div>
        </div>
        <div class="presupuesto">
          <div class="p-txt">{usadas} / {PRESUPUESTO} preguntas hoy</div>
          <div class="p-bar"><div class="p-fill" style:width={`${(usadas / PRESUPUESTO) * 100}%`}></div></div>
        </div>
      </div>

      {#each tarjetas as t (t.entrada.entryId)}
        <div class="tarjeta" class:validando={estados[t.entrada.entryId] === 'validando'} class:resuelta={estados[t.entrada.entryId] === 'resuelta'}>
          <div class="tarj-cab">
            <span class="dot-color" style:background={t.entrada.espacioColor}></span>
            <div class="tarj-body">
              <div class="tarj-txt">{t.entrada.contenido}</div>
              <div class="tarj-meta">{t.entrada.espacioNombre} · {t.entrada.proyectoNombre} · {t.entrada.origen} · confianza {t.entrada.confianza?.toFixed(2) ?? '?'}</div>
            </div>
            {#if t.entrada.esProvisional && estados[t.entrada.entryId] === 'activa'}
              <Chip variant="alerta">provisional</Chip>
            {/if}
            {#if estados[t.entrada.entryId] === 'validando'}
              <Chip variant="neutral">validando…</Chip>
            {:else if estados[t.entrada.entryId] === 'resuelta'}
              <Chip variant="ok">resuelta ✓</Chip>
            {:else if todasContestadas(t)}
              <Chip variant="indigo">lista para procesar</Chip>
            {/if}
          </div>

          {#each t.preguntas as q (q.preguntaId)}
            <div class="pregunta">
              <div class="p-txt">{q.texto}</div>
              <div class="opciones">
                {#each q.opciones as opcion, i}
                  <button
                    type="button"
                    class="opcion"
                    class:sel={q.respuestaOpcion === i}
                    onclick={() => marcarRespuesta(t.entrada.entryId, q, i)}
                  >
                    <Kbd>{String(i + 1)}</Kbd>
                    {opcion}
                  </button>
                {/each}
                <span class="libre">
                  <input
                    placeholder="…o dilo con tus palabras"
                    value={libres[q.preguntaId]?.texto ?? ''}
                    oninput={(e) => libres[q.preguntaId] = { texto: e.currentTarget.value, confirmado: false }}
                    onkeydown={(e) => {
                      if (e.key === 'Enter' && e.currentTarget.value.trim()) {
                        e.preventDefault();
                        marcarLibre(t.entrada.entryId, q, e.currentTarget.value);
                      }
                    }}
                    class:conf={libres[q.preguntaId]?.confirmado}
                  />
                  {#if libres[q.preguntaId]?.confirmado}
                    <span class="conf-hint">✓ con tus palabras</span>
                  {:else if libres[q.preguntaId]?.texto?.trim()}
                    <span class="enter-hint"><Kbd>↵</Kbd> confirmar</span>
                  {/if}
                </span>
              </div>
            </div>
          {/each}

          <div class="acciones">
            <span class="conf-meta">confianza {t.entrada.confianza?.toFixed(2) ?? '?'} · capa {t.entrada.confianzaCapa ?? '?'}</span>
            <button class="descartar" onclick={() => descartar(t)}>Descartar — vuelve al inbox</button>
            <button
              class="procesar"
              class:activo={todasContestadas(t) && estados[t.entrada.entryId] === 'activa'}
              disabled={!todasContestadas(t) || estados[t.entrada.entryId] !== 'activa'}
              onclick={() => procesar(t)}
            >
              {estados[t.entrada.entryId] === 'validando' ? 'Validando…' : estados[t.entrada.entryId] === 'resuelta' ? 'Archivada ✓' : 'Procesar'}
            </button>
          </div>
        </div>
      {/each}

      {#if vacia}
        <div class="vacia">
          <img src="/assets/logo.png" alt="Lunmia" width="56" height="56" style="opacity: 0.35" />
          <div class="v-t">Bandeja vacía</div>
          <div class="v-s">Nada que preguntar. Sigue a lo tuyo.</div>
          <Button variant="secondary" onclick={() => goto('/')}>Volver a Inicio</Button>
        </div>
      {/if}

      {#if !vacia}
        <div class="pie">
          <span class="hint"><Kbd>1–3</Kbd> responde la primera pregunta</span>
          <span class="right">Nada desaparece hasta que tú proceses esa Entrada.</span>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .fondo { min-height: 100vh; display: flex; align-items: center; justify-content: center; padding: 32px; background: #E7E7E3; }
  .ventana {
    width: 940px; max-width: 100%;
    background: var(--papel);
    border-radius: 12px;
    box-shadow: var(--sombra-flotante);
    border: 1px solid var(--borde);
    overflow: hidden;
  }
  .cuerpo { padding: 24px 30px 30px; display: flex; flex-direction: column; gap: 16px; min-height: 560px; }
  .head { display: flex; align-items: center; gap: 14px; }
  .volver-inicio { font-size: 11px; color: var(--apagado); text-decoration: none; display: block; margin-bottom: 4px; }
  .volver-inicio:hover { color: var(--indigo); }
  .titulo { font-family: var(--font-display); font-size: 22px; font-weight: 700; }
  .sub { font-size: 13px; color: var(--texto-2); margin-top: 2px; }
  .presupuesto { margin-left: auto; text-align: right; }
  .p-txt { font-family: var(--font-mono); font-size: 12px; color: var(--texto-2); }
  .p-bar { width: 140px; height: 4px; background: var(--borde); border-radius: 2px; overflow: hidden; margin-top: 5px; }
  .p-fill { height: 100%; background: var(--indigo); }
  .tarjeta {
    background: var(--superficie); border: 1px solid var(--borde);
    border-radius: var(--radio-l); padding: 16px 18px;
    display: flex; flex-direction: column; gap: 12px;
    transition: opacity 0.4s;
  }
  .tarjeta.validando { opacity: 0.6; }
  .tarjeta.resuelta { border-color: var(--ok-borde); opacity: 0.7; }
  .tarj-cab { display: flex; align-items: flex-start; gap: 10px; }
  .dot-color { width: 8px; height: 8px; border-radius: 2px; margin-top: 5px; flex-shrink: 0; }
  .tarj-body { flex: 1; min-width: 0; }
  .tarj-txt { font-size: 14px; font-weight: 500; }
  .tarj-meta { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); margin-top: 3px; }
  .pregunta { padding-left: 18px; display: flex; flex-direction: column; gap: 8px; }
  .pregunta .p-txt { font-size: 13px; color: var(--texto-2); }
  .opciones { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }
  .opcion {
    background: var(--superficie); color: var(--tinta);
    border: 1px solid var(--borde-fuerte); border-radius: var(--radio-m);
    padding: 6px 14px;
    font-family: var(--font-ui); font-weight: 500; font-size: 13px;
    display: inline-flex; align-items: center; gap: 7px;
    cursor: pointer;
  }
  .opcion:hover { border-color: var(--indigo); }
  .opcion.sel { background: var(--indigo-suave); color: var(--indigo); border-color: var(--indigo); }
  .libre { position: relative; flex: 1; min-width: 200px; display: flex; align-items: center; }
  .libre input {
    width: 100%; box-sizing: border-box;
    font-family: var(--font-ui); font-size: 12px;
    padding: 6px 100px 6px 10px;
    border: 1px solid var(--borde); border-radius: var(--radio-m);
    background: var(--papel); color: var(--tinta); outline: none;
  }
  .libre input.conf { background: #F2F7F3; border-color: var(--ok); }
  .conf-hint { position: absolute; right: 8px; font-size: 10px; color: var(--ok-texto); }
  .enter-hint { position: absolute; right: 8px; display: inline-flex; align-items: center; gap: 4px; font-size: 10px; color: var(--apagado); }
  .acciones { display: flex; align-items: center; gap: 10px; padding-left: 18px; }
  .conf-meta { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); }
  .descartar {
    margin-left: auto; background: transparent; color: var(--apagado);
    border: none; border-radius: var(--radio-m); padding: 5px 10px;
    font-family: var(--font-ui); font-size: 12px; cursor: pointer;
  }
  .descartar:hover { color: var(--error); background: var(--error-suave); }
  .procesar {
    background: var(--borde); color: var(--apagado);
    border: none; border-radius: var(--radio-m); padding: 6px 16px;
    font-family: var(--font-ui); font-weight: 600; font-size: 12px;
    cursor: default;
  }
  .procesar.activo { background: var(--tinta); color: var(--papel); cursor: pointer; }
  .procesar.activo:hover { background: var(--tinta-hover); }
  .vacia {
    flex: 1; display: flex; flex-direction: column;
    align-items: center; justify-content: center;
    gap: 12px; padding: 48px 0;
  }
  .v-t { font-family: var(--font-display); font-size: 20px; font-weight: 600; }
  .v-s { font-size: 13px; color: var(--apagado); }
  .pie {
    display: flex; align-items: center; gap: 12px;
    padding-top: 8px; border-top: 1px solid var(--borde);
  }
  .hint { display: inline-flex; align-items: center; gap: 5px; font-size: 11px; color: var(--apagado); }
  .right { flex: 1; text-align: right; font-size: 11px; color: var(--apagado); }
</style>
