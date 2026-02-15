import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  base: '/naechste/',
  plugins: [react()],
  build: {
    outDir: '../docs',
    emptyOutDir: false,
    sourcemap: false
  },
  server: {
    open: true
  }
})
