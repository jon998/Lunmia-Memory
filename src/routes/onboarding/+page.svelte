<script lang="ts">
  import { goto } from '$app/navigation';
  import Bell from '@lucide/svelte/icons/bell';
  import Keyboard from '@lucide/svelte/icons/keyboard';
  import Mic from '@lucide/svelte/icons/mic';
  import Sparkles from '@lucide/svelte/icons/sparkles';
  import Plus from '@lucide/svelte/icons/plus';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import RefreshCw from '@lucide/svelte/icons/refresh-cw';
  import Kbd from '$lib/components/Kbd.svelte';
  import Globe from '@lucide/svelte/icons/globe';
  import {
    finalizarOnboarding,
    proponerSetupInicial,
    verificarPermisos,
    solicitarPermisoNotificaciones,
    solicitarPermisoAccesibilidad,
    solicitarPermisoMicrofono
  } from '$lib/tauri';
  import type { EspacioPropuesto, PropuestaSetup, TipoPropuesto } from '$lib/types';
  import LimiteHint from '$lib/components/LimiteHint.svelte';
  import { LIMITES } from '$lib/limites';
  import {
    ZONAS,
    detectarZonaSistema,
    etiquetaZona,
    horaEnZona,
    offsetZona,
    setZonaActiva
  } from '$lib/zona';

  type PasoId = 'bienvenida' | 'oficio' | 'atajo' | 'zona' | 'permisos' | 'resumen';
  const pasos: PasoId[] = ['bienvenida', 'oficio', 'atajo', 'zona', 'permisos', 'resumen'];

  let indice = $state(0);
  const paso = $derived(pasos[indice]);
  const numero = $derived(indice + 1);
  const total = pasos.length;

  let oficio = $state('');
  let atajo = $state<'A' | 'B'>('A');
  let autostart = $state(true);
  let capturaVoz = $state(false);
  let zonaHoraria = $state(detectarZonaSistema());
  let busquedaZona = $state('');
  const zonaSistema = detectarZonaSistema();

  let permisos = $state({ accesibilidad: false, notificaciones: false, microfono: false });
  let solicitando = $state<string | null>(null);

  let propuesta = $state<PropuestaSetup | null>(null);
  let cargandoPropuesta = $state(false);
  let errorPropuesta = $state<string | null>(null);

  let espaciosEditables = $state<Array<EspacioPropuesto & { activo: boolean }>>([]);
  let tiposEditables = $state<Array<TipoPropuesto & { activo: boolean }>>([]);
  let nuevoEspacio = $state({ nombre: '', tipo: 'personal' as EspacioPropuesto['tipo'], color: '#6B4CA8' });
  let nuevoTipo = $state('');

  const permisosDefs = $derived.by(() => {
    const base: Array<{
      id: 'accesibilidad' | 'notificaciones' | 'microfono';
      icono: 'kbd' | 'bell' | 'mic';
      nombre: string;
      razon: string;
      accion: () => Promise<boolean>;
    }> = [
      {
        id: 'accesibilidad',
        icono: 'kbd',
        nombre: 'Accesibilidad',
        razon: 'Para que ⌘⇧Space funcione encima de cualquier aplicación.',
        accion: solicitarPermisoAccesibilidad
      },
      {
        id: 'notificaciones',
        icono: 'bell',
        nombre: 'Notificaciones',
        razon: 'Para que los recordatorios disparen incluso con Lunmia cerrada.',
        accion: solicitarPermisoNotificaciones
      }
    ];
    if (capturaVoz) {
      base.push({
        id: 'microfono',
        icono: 'mic',
        nombre: 'Micrófono',
        razon: 'Para capturar por voz (10–20 segundos, transcrito en tu Mac).',
        accion: solicitarPermisoMicrofono
      });
    }
    return base;
  });

  const todosConcedidos = $derived(permisosDefs.every((p) => permisos[p.id]));

  const zonasFiltradas = $derived.by(() => {
    const q = busquedaZona.trim().toLowerCase();
    let lista = [...ZONAS];
    if (!lista.some((z) => z.id === zonaSistema)) {
      lista = [
        { id: zonaSistema, ciudad: etiquetaZona(zonaSistema), grupo: 'Tu Mac' },
        ...lista
      ];
    }
    if (!q) return lista;
    return lista.filter(
      (z) =>
        z.ciudad.toLowerCase().includes(q) ||
        z.id.toLowerCase().includes(q) ||
        z.grupo.toLowerCase().includes(q)
    );
  });

  const gruposZona = $derived.by(() => {
    const map = new Map<string, typeof ZONAS>();
    for (const z of zonasFiltradas) {
      const arr = map.get(z.grupo) ?? [];
      arr.push(z);
      map.set(z.grupo, arr);
    }
    return [...map.entries()];
  });

  function elegirZona(id: string) {
    zonaHoraria = id;
    setZonaActiva(id);
  }

  async function refrescarPermisos() {
    try {
      permisos = await verificarPermisos();
    } catch (err) {
      console.warn('no se pudo verificar permisos', err);
    }
  }

  async function conceder(
    id: 'accesibilidad' | 'notificaciones' | 'microfono',
    accion: () => Promise<boolean>
  ) {
    solicitando = id;
    try {
      await accion();
      await refrescarPermisos();
      // Accesibilidad requiere que el usuario marque el switch en System
      // Settings; hacemos un par de re-checks para captarlo pronto.
      setTimeout(refrescarPermisos, 1500);
      setTimeout(refrescarPermisos, 4000);
    } catch (err) {
      console.warn('permiso rechazado', err);
    } finally {
      solicitando = null;
    }
  }

  async function cargarPropuesta() {
    if (cargandoPropuesta) return;
    cargandoPropuesta = true;
    errorPropuesta = null;
    try {
      propuesta = await proponerSetupInicial(oficio);
      espaciosEditables = propuesta.espacios.map((e) => ({ ...e, activo: true }));
      tiposEditables = propuesta.tipos.map((t) => ({ ...t, activo: true }));
    } catch (err) {
      console.warn(err);
      errorPropuesta = 'No pudimos generar la propuesta. Puedes seguir; se creará una base mínima.';
    } finally {
      cargandoPropuesta = false;
    }
  }

  function agregarEspacio() {
    if (!nuevoEspacio.nombre.trim()) return;
    espaciosEditables = [
      ...espaciosEditables,
      { ...nuevoEspacio, razon: 'Agregado por ti.', activo: true }
    ];
    nuevoEspacio = { nombre: '', tipo: 'personal', color: '#6B4CA8' };
  }

  function agregarTipo() {
    const n = nuevoTipo.trim().toLowerCase();
    if (!n) return;
    if (tiposEditables.some((t) => t.nombre === n)) {
      nuevoTipo = '';
      return;
    }
    tiposEditables = [...tiposEditables, { nombre: n, razon: 'Agregado por ti.', activo: true }];
    nuevoTipo = '';
  }

  const continuarBloqueado = $derived(
    cargandoPropuesta
    || (paso === 'oficio' && !oficio.trim())
    || (paso === 'resumen' && !propuesta && !errorPropuesta)
  );

  async function siguiente() {
    if (continuarBloqueado) return;
    if (paso === 'oficio') {
      if (cargandoPropuesta) return;
      await cargarPropuesta();
      indice += 1;
      return;
    }
    if (paso === 'permisos') {
      indice += 1;
      if (!propuesta && !cargandoPropuesta) cargarPropuesta();
      return;
    }
    if (paso === 'resumen') {
      await terminar();
      return;
    }
    indice += 1;
  }

  async function terminar() {
    try {
      await finalizarOnboarding({
        oficio: oficio.trim(),
        atajo: atajo === 'A' ? 'CmdOrCtrl+Shift+Space' : 'DoubleOption',
        autostart,
        capturaVoz,
        tipos: tiposEditables.filter((t) => t.activo).map((t) => t.nombre),
        espaciosNuevos: espaciosEditables
          .filter((e) => e.activo)
          .map(({ activo: _a, ...rest }) => rest),
        espaciosAEliminar: [],
        zonaHoraria
      });
    } catch (err) {
      console.warn(err);
    }
    goto('/');
  }

  $effect(() => {
    if (paso === 'permisos') refrescarPermisos();
  });
</script>

<div class="fondo">
  <div class="ventana">
    <div class="cuerpo">
      {#if paso === 'bienvenida'}
        <div class="paso centro">
          <img src="/assets/logo.png" alt="Lunmia" width="96" height="96" />
          <h1 class="titulo">Nada se pierde.</h1>
          <p class="lead">Lunmia captura ideas, notas y recordatorios en menos de tres segundos, desde cualquier app. Todo se queda en tu Mac; nada sale a la nube.</p>
          <div class="promesa">2 minutos. Prometido.</div>
          <button class="btn-primary" onclick={() => (indice = 1)}>Empezar</button>
        </div>

      {:else if paso === 'oficio'}
        <div class="paso">
          <span class="eyebrow">CONOCERTE</span>
          <h2 class="h">¿A qué te dedicas?</h2>
          <p class="lead">Cuéntalo como se lo contarías a alguien en un café. Con esto Lunmia te propondrá espacios y tipos de Entrada hechos a tu medida — tú los aceptas, renombras o tiras a la basura.</p>
          <textarea
            bind:value={oficio}
            disabled={cargandoPropuesta}
            maxlength={LIMITES.oficio}
            placeholder="Soy desarrollador, trabajo en una app de finanzas, estudio ML por las noches y tengo dos proyectos personales que siempre abandono en octubre…"
          ></textarea>
          {#if cargandoPropuesta}
            <div class="cargando"><Sparkles size={16} /> Pensando tu setup…</div>
          {:else}
            <div class="oficio-pie">
              <span class="nota-mono">Sin lista predefinida. Tu taxonomía nace de ti, no de nosotros.</span>
              <LimiteHint valor={oficio} max={LIMITES.oficio} />
            </div>
          {/if}
        </div>

      {:else if paso === 'atajo'}
        <div class="paso">
          <span class="eyebrow">EL ATAJO</span>
          <h2 class="h">Tu tecla de "no se me escapa"</h2>
          <p class="lead">Desde cualquier app, este atajo abre la captura flotante sin robarte el foco. Spotlight se queda con ⌘Space; esa guerra no se gana.</p>
          <div class="opciones">
            <button class="opcion" class:sel={atajo === 'A'} onclick={() => (atajo = 'A')}>
              <div class="teclas"><Kbd>⌘</Kbd><Kbd>⇧</Kbd><Kbd>Space</Kbd></div>
              <div class="nombre">Recomendado</div>
              <div class="desc">Suele estar libre y respeta el gesto mental.</div>
            </button>
            <button class="opcion" class:sel={atajo === 'B'} onclick={() => (atajo = 'B')}>
              <div class="teclas"><Kbd>⌥</Kbd><span class="times">× 2</span></div>
              <div class="nombre">Doble Option</div>
              <div class="desc">Más elegante; dos toques rápidos de ⌥.</div>
            </button>
          </div>
          <label class="toggle">
            <input type="checkbox" bind:checked={capturaVoz} />
            <span>También quiero capturar por voz (habilita permiso de micrófono).</span>
          </label>
          <span class="nota-mono">Se puede cambiar cuando quieras en Ajustes.</span>
        </div>

      {:else if paso === 'zona'}
        <div class="paso">
          <span class="eyebrow">TU HORA</span>
          <h2 class="h">¿En qué zona vives?</h2>
          <p class="lead">“Hoy”, “mañana” y el calendario se calculan en esta hora. Si viajas, cámbiala después — las fechas ya guardadas no se mueven.</p>
          <button
            type="button"
            class="opcion zona-detectada"
            class:sel={zonaHoraria === zonaSistema}
            onclick={() => elegirZona(zonaSistema)}
          >
            <Globe size={16} strokeWidth={1.5} />
            <div class="zona-detectada-body">
              <div class="nombre">La de tu Mac · {etiquetaZona(zonaSistema)}</div>
              <div class="desc">{offsetZona(zonaSistema)} · ahora {horaEnZona(zonaSistema)}</div>
            </div>
          </button>
          <input
            class="buscar-zona"
            bind:value={busquedaZona}
            maxlength={LIMITES.busqueda}
            placeholder="Buscar ciudad o zona…"
          />
          <div class="lista-zonas">
            {#each gruposZona as [grupo, zs]}
              <div class="grupo-zona">{grupo}</div>
              {#each zs as z (z.id)}
                <button
                  type="button"
                  class="fila-zona"
                  class:sel={zonaHoraria === z.id}
                  onclick={() => elegirZona(z.id)}
                >
                  <span class="fila-zona-n">{z.ciudad}</span>
                  <span class="fila-zona-m">{offsetZona(z.id)} · {horaEnZona(z.id)}</span>
                </button>
              {/each}
            {/each}
            {#if gruposZona.length === 0}
              <div class="nota-mono">Nada con esa búsqueda.</div>
            {/if}
          </div>
          <span class="nota-mono">Elegida: {etiquetaZona(zonaHoraria)} · {horaEnZona(zonaHoraria)}</span>
        </div>

      {:else if paso === 'permisos'}
        <div class="paso">
          <span class="eyebrow">PERMISOS DE MACOS</span>
          <h2 class="h">Los permisos que necesita macOS</h2>
          <p class="lead">macOS los pide por ti. Sin ellos Lunmia es una app muda que no escucha atajos — o sea, un icono bonito.</p>
          {#each permisosDefs as p (p.id)}
            <div class="perm">
              <div class="perm-icono">
                {#if p.icono === 'kbd'}<Keyboard size={18} strokeWidth={1.5} />{/if}
                {#if p.icono === 'bell'}<Bell size={18} strokeWidth={1.5} />{/if}
                {#if p.icono === 'mic'}<Mic size={18} strokeWidth={1.5} />{/if}
              </div>
              <div class="perm-body">
                <div class="perm-n">{p.nombre}</div>
                <div class="perm-r">{p.razon}</div>
              </div>
              {#if permisos[p.id]}
                <span class="chip-ok">Concedido ✓</span>
              {:else}
                <button
                  class="chip-conc"
                  disabled={solicitando === p.id}
                  onclick={() => conceder(p.id, p.accion)}
                >
                  {solicitando === p.id ? 'Abriendo…' : 'Conceder'}
                </button>
              {/if}
            </div>
          {/each}
          <button class="auto" onclick={() => (autostart = !autostart)}>
            <span class="check" class:on={autostart}>{autostart ? '✓' : ''}</span>
            <div class="auto-body">
              <div class="auto-n">Iniciar Lunmia al encender la Mac</div>
              <div class="auto-r">Siempre lista en la barra de menú, sin abrir nada.</div>
            </div>
          </button>
          <div class="permisos-pie">
            <button class="ghost mini" onclick={refrescarPermisos}>
              <RefreshCw size={12} strokeWidth={2} /> Revisar de nuevo
            </button>
            <span class="nota-mono">Accesibilidad requiere marcar Lunmia en System Settings.</span>
          </div>
        </div>

      {:else if paso === 'resumen'}
        <div class="paso">
          <span class="eyebrow">TU CONFIGURACIÓN INICIAL</span>
          <h2 class="h">Esto te preparamos. Ajústalo.</h2>
          {#if cargandoPropuesta}
            <div class="cargando"><Sparkles size={16} /> Pensando tu setup…</div>
          {:else if errorPropuesta}
            <div class="err">{errorPropuesta}</div>
          {:else}
            <div class="bloque">
              <div class="bloque-titulo">Espacios <span class="cont">{espaciosEditables.filter(e => e.activo).length}</span></div>
              <p class="bloque-lead">Cada espacio es un contexto de vida (trabajo, personal, un cliente concreto). Activa los que quieras y añade otros.</p>
              <ul class="lista">
                {#each espaciosEditables as esp, i (esp.nombre + i)}
                  <li class="fila" class:off={!esp.activo}>
                    <span class="punto" style="background: {esp.color}"></span>
                    <div class="fila-body">
                      <input class="fila-input" bind:value={esp.nombre} maxlength={LIMITES.nombre} />
                      <div class="fila-razon">{esp.razon}</div>
                    </div>
                    <label class="mini-toggle">
                      <input type="checkbox" bind:checked={esp.activo} />
                      <span>{esp.activo ? 'Usar' : 'Ignorar'}</span>
                    </label>
                    <button
                      class="btn-icono"
                      title="Quitar de la lista"
                      onclick={() => (espaciosEditables = espaciosEditables.filter((_, j) => j !== i))}
                    >
                      <Trash2 size={14} />
                    </button>
                  </li>
                {/each}
              </ul>
              <div class="agregar">
                <input class="mini-input" placeholder="Nombre del espacio" bind:value={nuevoEspacio.nombre} maxlength={LIMITES.nombre} />
                <select class="mini-input" bind:value={nuevoEspacio.tipo}>
                  <option value="trabajo">trabajo</option>
                  <option value="personal">personal</option>
                  <option value="cliente">cliente</option>
                  <option value="estudio">estudio</option>
                </select>
                <input class="mini-color" type="color" bind:value={nuevoEspacio.color} />
                <button class="btn-add" onclick={agregarEspacio}><Plus size={12} /> Agregar</button>
              </div>
            </div>

            <div class="bloque">
              <div class="bloque-titulo">Tipos de Entrada <span class="cont">{tiposEditables.filter(t => t.activo).length}</span></div>
              <p class="bloque-lead">Etiquetas semánticas. Aparecen como sugerencia al clasificar. Un click alterna activo/inactivo.</p>
              <div class="chips">
                {#each tiposEditables as t, i (t.nombre + i)}
                  <button
                    type="button"
                    class="chip"
                    class:off={!t.activo}
                    title={t.razon}
                    onclick={() => (t.activo = !t.activo)}
                  >
                    {t.nombre}
                  </button>
                {/each}
              </div>
              <div class="agregar">
                <input
                  class="mini-input"
                  placeholder="Añadir tipo (ej. bug, cita, seguimiento)"
                  bind:value={nuevoTipo}
                  maxlength={LIMITES.nombre}
                  onkeydown={(e) => e.key === 'Enter' && (e.preventDefault(), agregarTipo())}
                />
                <button class="btn-add" onclick={agregarTipo}><Plus size={12} /> Añadir</button>
              </div>
            </div>

            {#if propuesta?.recomendaciones?.length}
              <div class="bloque reco-bloque">
                <div class="bloque-titulo"><Sparkles size={12} /> Consejos</div>
                <ul class="reco">
                  {#each propuesta.recomendaciones as r}
                    <li>{r}</li>
                  {/each}
                </ul>
              </div>
            {/if}
          {/if}
        </div>
      {/if}

      <div class="pie">
        {#if indice > 0}
          <button class="ghost" onclick={() => (indice -= 1)} disabled={cargandoPropuesta}>← Atrás</button>
        {:else}
          <span class="ghost-space"></span>
        {/if}
        <div class="puntos">
          {#each pasos as _, n}
            <span class="pt" class:on={n === indice}></span>
          {/each}
        </div>
        <span class="counter">{numero} / {total}</span>
        {#if indice > 0}
          <button
            class="btn-primary sm"
            onclick={siguiente}
            disabled={continuarBloqueado}
          >
            {#if cargandoPropuesta}
              Pensando…
            {:else if paso === 'resumen'}
              Empezar a capturar →
            {:else if paso === 'permisos'}
              {todosConcedidos ? 'Continuar →' : 'Luego los concedo →'}
            {:else}
              Continuar →
            {/if}
          </button>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .fondo { min-height: 100vh; display: flex; align-items: center; justify-content: center; padding: 40px; background: #E7E7E3; }
  .ventana {
    width: 880px; max-width: 100%;
    background: var(--papel); border-radius: 12px;
    box-shadow: var(--sombra-flotante); border: 1px solid var(--borde);
    overflow: hidden;
  }
  .cuerpo {
    min-height: 560px; max-height: 78vh;
    display: flex; flex-direction: column;
    padding: 36px 60px 24px;
    box-sizing: border-box;
    overflow: hidden;
  }
  .paso {
    flex: 1;
    display: flex; flex-direction: column; gap: 14px;
    max-width: 620px; margin: 0 auto; width: 100%;
    overflow-y: auto;
    padding-right: 4px;
  }
  .paso.centro { align-items: center; text-align: center; gap: 18px; justify-content: center; }
  .titulo { font-family: var(--font-display); font-size: 40px; font-weight: 700; margin: 0; letter-spacing: -0.01em; }
  .h { font-family: var(--font-display); font-size: 30px; font-weight: 700; margin: 0 0 4px; letter-spacing: -0.01em; }
  .lead {
    font-size: 14px; color: var(--texto-2); line-height: 1.65; margin: 0;
    width: 100%; max-width: none;
    text-wrap: pretty;
  }
  .paso:not(.centro) .lead {
    text-align: justify;
    text-align-last: left;
    hyphens: auto;
    -webkit-hyphens: auto;
  }
  .paso.centro .lead { max-width: 480px; text-align: center; }
  .eyebrow { font-family: var(--font-mono); font-size: 11px; color: var(--indigo); letter-spacing: 0.12em; }
  .promesa { font-family: var(--font-mono); font-size: 12px; color: var(--apagado); }
  .nota-mono { font-family: var(--font-mono); font-size: 11px; color: var(--apagado); }
  .oficio-pie {
    display: flex; align-items: baseline; justify-content: space-between; gap: 16px;
    width: 100%;
  }
  .oficio-pie .nota-mono { flex: 1; min-width: 0; line-height: 1.45; }
  textarea {
    font-family: var(--font-ui); font-size: 14px; line-height: 1.6;
    padding: 14px 16px; border: 1.5px solid var(--tinta);
    border-radius: var(--radio-l); background: var(--superficie);
    color: var(--tinta); outline: none; resize: none;
    width: 100%; box-sizing: border-box;
    height: 140px; box-shadow: var(--sombra-captura);
  }
  .opciones { display: flex; gap: 10px; }
  .opcion {
    flex: 1; background: var(--superficie); border: 1.5px solid var(--borde);
    border-radius: var(--radio-l); padding: 16px;
    display: flex; flex-direction: column; gap: 8px;
    cursor: pointer; text-align: left;
  }
  .opcion.sel { background: var(--indigo-suave); border-color: var(--indigo); }
  .teclas { display: flex; gap: 6px; align-items: center; }
  .nombre { font-size: 13px; font-weight: 600; }
  .desc { font-size: 12px; color: var(--texto-2); }
  .times { font-family: var(--font-mono); font-size: 12px; color: var(--apagado); }
  .zona-detectada {
    flex-direction: row; align-items: center; gap: 12px; padding: 12px 14px;
  }
  .zona-detectada-body { display: flex; flex-direction: column; gap: 2px; }
  .buscar-zona {
    font-family: var(--font-ui); font-size: 13px;
    padding: 8px 12px; border: 1px solid var(--borde-fuerte);
    border-radius: var(--radio-m); background: var(--superficie);
  }
  .lista-zonas {
    flex: 1; min-height: 180px; max-height: 280px;
    overflow-y: auto; border: 1px solid var(--borde);
    border-radius: var(--radio-l); background: var(--superficie);
    padding: 6px;
  }
  .grupo-zona {
    font-family: var(--font-mono); font-size: 10px; color: var(--apagado);
    letter-spacing: 0.1em; text-transform: uppercase;
    padding: 8px 10px 4px;
  }
  .fila-zona {
    width: 100%; display: flex; align-items: baseline; gap: 10px;
    background: transparent; border: none; border-radius: var(--radio-m);
    padding: 7px 10px; cursor: pointer; text-align: left;
  }
  .fila-zona:hover { background: var(--superficie-2); }
  .fila-zona.sel { background: var(--indigo-suave); }
  .fila-zona-n { font-size: 13px; font-weight: 500; flex: 1; }
  .fila-zona-m { font-family: var(--font-mono); font-size: 10px; color: var(--apagado); }
  .toggle {
    display: flex; align-items: center; gap: 10px;
    font-size: 13px; color: var(--texto-2);
    cursor: pointer;
  }
  .toggle input { width: 16px; height: 16px; }

  .perm {
    display: flex; align-items: center; gap: 14px;
    background: var(--superficie); border: 1px solid var(--borde);
    border-radius: var(--radio-l); padding: 14px 16px;
  }
  .perm-icono {
    width: 34px; height: 34px; border-radius: 8px;
    background: var(--superficie-2);
    display: flex; align-items: center; justify-content: center;
    color: var(--texto-2); flex-shrink: 0;
  }
  .perm-body { flex: 1; }
  .perm-n { font-size: 14px; font-weight: 600; }
  .perm-r { font-size: 12px; color: var(--texto-2); margin-top: 2px; }
  .chip-ok { font-size: 12px; background: var(--ok-suave); color: var(--ok-texto); border: 1px solid var(--ok-borde); border-radius: 4px; padding: 4px 10px; font-weight: 500; }
  .chip-conc { background: var(--indigo-suave); color: var(--indigo); border: 1px solid var(--indigo-borde); border-radius: var(--radio-m); padding: 6px 14px; font-family: var(--font-ui); font-weight: 600; font-size: 13px; cursor: pointer; }
  .chip-conc:disabled { opacity: 0.5; cursor: wait; }
  .auto {
    display: flex; align-items: center; gap: 12px;
    background: var(--indigo-suave); border: 1px solid var(--indigo-borde);
    border-radius: var(--radio-l); padding: 12px 16px;
    cursor: pointer; text-align: left; width: 100%;
  }
  .check {
    width: 18px; height: 18px; border-radius: 4px;
    border: 1.5px solid var(--borde-fuerte);
    display: flex; align-items: center; justify-content: center;
    color: var(--superficie); font-size: 12px; font-weight: 700; flex-shrink: 0;
  }
  .check.on { background: var(--indigo); border-color: var(--indigo); }
  .auto-body { flex: 1; }
  .auto-n { font-size: 14px; font-weight: 600; }
  .auto-r { font-size: 12px; color: var(--texto-2); margin-top: 2px; }
  .permisos-pie {
    display: flex; align-items: center; gap: 12px; justify-content: space-between;
  }
  .ghost.mini {
    display: inline-flex; align-items: center; gap: 6px;
    font-family: var(--font-mono); font-size: 11px;
    padding: 4px 8px;
  }

  .bloque {
    background: var(--superficie); border: 1px solid var(--borde);
    border-radius: var(--radio-l); padding: 14px 16px;
    display: flex; flex-direction: column; gap: 10px;
  }
  .bloque-titulo {
    display: flex; align-items: center; gap: 6px;
    font-size: 13px; font-weight: 600; color: var(--tinta);
  }
  .bloque-titulo .cont {
    background: var(--indigo-suave); color: var(--indigo);
    border-radius: 999px; padding: 1px 8px; font-size: 11px; font-weight: 600;
  }
  .bloque-lead { font-size: 12px; color: var(--texto-2); margin: 0; }
  .lista { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 6px; }
  .fila {
    display: flex; align-items: center; gap: 10px;
    padding: 8px 10px; border: 1px solid var(--borde);
    border-radius: var(--radio-m); background: var(--superficie-2);
  }
  .fila.off { opacity: 0.5; }
  .punto { width: 12px; height: 12px; border-radius: 50%; flex-shrink: 0; }
  .fila-body { flex: 1; min-width: 0; }
  .fila-input {
    width: 100%; background: transparent; border: none; outline: none;
    font-family: var(--font-ui); font-size: 13px; font-weight: 600; color: var(--tinta);
    padding: 0;
  }
  .fila-razon { font-size: 11px; color: var(--texto-2); }
  .mini-toggle { display: flex; align-items: center; gap: 4px; font-size: 11px; color: var(--texto-2); cursor: pointer; }
  .mini-toggle input { width: 14px; height: 14px; }
  .btn-icono {
    background: none; border: none; color: var(--texto-2); cursor: pointer;
    padding: 4px; border-radius: 4px;
  }
  .btn-icono:hover { background: var(--borde); color: var(--tinta); }
  .agregar { display: flex; gap: 6px; align-items: center; }
  .mini-input {
    flex: 1; padding: 6px 10px;
    background: var(--superficie); border: 1px solid var(--borde);
    border-radius: var(--radio-m); font-family: var(--font-ui); font-size: 12px;
    outline: none; color: var(--tinta);
  }
  .mini-color { width: 32px; height: 30px; border: 1px solid var(--borde); border-radius: 6px; cursor: pointer; padding: 0; background: transparent; }
  .btn-add {
    background: var(--indigo-suave); color: var(--indigo);
    border: 1px solid var(--indigo-borde); border-radius: var(--radio-m);
    padding: 6px 12px; font-family: var(--font-ui); font-size: 12px; font-weight: 600;
    cursor: pointer; display: inline-flex; align-items: center; gap: 4px;
  }
  .chips { display: flex; flex-wrap: wrap; gap: 6px; }
  .chip {
    background: var(--superficie-2); border: 1px solid var(--borde);
    border-radius: 999px; padding: 4px 12px;
    font-family: var(--font-mono); font-size: 12px; color: var(--tinta);
    cursor: pointer; transition: opacity 0.15s;
  }
  .chip.off { opacity: 0.4; text-decoration: line-through; }
  .reco-bloque { background: var(--indigo-suave); border-color: var(--indigo-borde); }
  .reco { margin: 0; padding-left: 18px; font-size: 12px; color: var(--tinta); line-height: 1.7; }
  .cargando {
    display: flex; align-items: center; gap: 8px;
    font-family: var(--font-mono); font-size: 12px; color: var(--indigo);
    padding: 20px; justify-content: center;
  }
  .err { font-size: 12px; color: #A64444; padding: 8px 12px; border: 1px solid #C58080; border-radius: 6px; background: rgba(192, 95, 95, 0.08); }

  .pie {
    display: flex; align-items: center; gap: 12px;
    padding-top: 16px; border-top: 1px solid var(--borde);
    margin-top: 12px;
  }
  .puntos { flex: 1; display: flex; justify-content: center; gap: 8px; }
  .pt { width: 7px; height: 7px; border-radius: 50%; background: var(--borde-fuerte); }
  .pt.on { background: var(--indigo); }
  .counter { font-family: var(--font-mono); font-size: 11px; color: var(--apagado); }
  .ghost {
    background: transparent; border: none; color: var(--texto-2);
    border-radius: var(--radio-m); padding: 8px 12px; cursor: pointer;
    font-family: var(--font-ui); font-size: 13px; font-weight: 500;
  }
  .ghost:hover { background: var(--superficie-2); }
  .ghost:disabled { opacity: 0.4; cursor: not-allowed; }
  textarea:disabled { opacity: 0.6; cursor: wait; }
  .ghost-space { width: 62px; }
  .btn-primary {
    background: var(--tinta); color: var(--papel);
    border: none; border-radius: var(--radio-m);
    padding: 11px 28px; font-family: var(--font-ui); font-weight: 600; font-size: 14px;
    cursor: pointer;
  }
  .btn-primary.sm { padding: 8px 18px; font-size: 13px; }
  .btn-primary:hover { background: var(--tinta-hover); }
  .btn-primary:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
