import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import { fileURLToPath } from "node:url";
import svgr from "vite-plugin-svgr";

const filesNeedToExclude = ["build"];

const filesPathToExclude = filesNeedToExclude.map((src) => {
  return fileURLToPath(new URL(src, import.meta.url));
});

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), svgr({
    include: '**/*.svg'
  })],
  base: './',
  root: 'src',
  build: {
    outDir: '../dist',
    manifest: true,
  },
})
