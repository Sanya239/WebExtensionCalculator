import tailwindcss from '@tailwindcss/vite'
import react from '@vitejs/plugin-react'
import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [react(), tailwindcss()],
  build: {
    rollupOptions: {
      input: {
        popup: fileURLToPath(new URL('index.html', import.meta.url)),
        background: fileURLToPath(new URL('src/extension/background.ts', import.meta.url)),
      },
      output: {
        entryFileNames: ({ name }) => name === 'background'
          ? 'background.js'
          : 'assets/[name]-[hash].js',
      },
    },
  },
})
