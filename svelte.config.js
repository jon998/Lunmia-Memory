// Tauri no tiene servidor Node para SSR real: usamos adapter-static con
// fallback SPA para servir rutas dinámicas (/entrada/[id]) desde el bundle.
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: "index.html",
      strict: false,
    }),
  },
};

export default config;
