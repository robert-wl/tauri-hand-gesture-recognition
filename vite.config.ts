import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  optimizeDeps: {
    exclude: ["node_modules/*", "src-tauri/dataset/*", "src-tauri/processed/*", "src-tauri/result/*"],
  },
  root: ".",
});
