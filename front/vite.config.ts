// Plugins
import vue from "@vitejs/plugin-vue";
import { chunkSplitPlugin } from 'vite-plugin-chunk-split';
import htmlPurge from 'vite-plugin-purgecss'
import viteCompression from 'vite-plugin-compression';
// Utilities
import { defineConfig } from "vite";
import { fileURLToPath, URL } from "node:url";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue({}),
    chunkSplitPlugin(),
    // htmlPurge({
    //   safelist: {
    //     standard: [],
    //     deep: [],
    //     greedy: [/p-/, /pi-/, /col-/, /flag/, /layout/, /customer/, /lg/, /md/, /sm/, /hover/, /min/, /max/, /out/]
    //   }
    // }),
    viteCompression(),
  ],
  define: { "process.env": {} },
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
    extensions: [".js", ".json", ".jsx", ".mjs", ".ts", ".tsx", ".vue"],
  },
  server: {
    port: 3000,
  },
});
