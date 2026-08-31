<script lang="ts">
  import '$lib/styles/tokens.css';
  import '$lib/styles/app.css';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { estadoOnboarding, isTauri } from '$lib/tauri';
  import { app } from '$lib/stores/app.svelte';

  let { children } = $props();
  let listo = $state(false);

  onMount(async () => {
    if (typeof window === 'undefined') return;

    // Estilo de la ventana de captura flotante (transparente)
    if (isTauri()) {
      try {
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        const win = getCurrentWindow();
        if (win.label === 'captura') {
          document.documentElement.style.background = 'transparent';
          document.body.style.background = 'transparent';
        }
        // Escuchar eventos de navegación desde el menú del tray
        const { listen } = await import('@tauri-apps/api/event');
        void listen<string>('navegar', (event) => {
          void goto(event.payload);
        });
        // Refrescar cuando el backend termina de clasificar una entrada.
        if (win.label === 'main') {
          void listen('entrada:clasificada', () => {
            void app.refrescarEntradas();
          });
          void listen('agenda:cambio', () => {
            app.tocarAgenda();
          });
        }
      } catch (err) {
        console.warn('Tauri APIs no disponibles:', err);
      }
    }

    // Redirigir a onboarding si aún no se completó (ventana principal)
    const pathname = $page.url.pathname;
    const rutasSinOnboarding = ['/onboarding', '/captura'];
    try {
      await app.cargarZona();
    } catch (err) {
      console.warn('No se pudo cargar la zona horaria:', err);
    }
    void app.comprobarLlm();
    if (!rutasSinOnboarding.includes(pathname)) {
      try {
        const estado = await estadoOnboarding();
        if (!estado.completado) {
          await goto('/onboarding');
          listo = true;
          return;
        }
      } catch (err) {
        console.warn('No se pudo consultar el estado del onboarding:', err);
      }
    }

    listo = true;
  });
</script>

{#if listo}
  {@render children()}
{/if}
