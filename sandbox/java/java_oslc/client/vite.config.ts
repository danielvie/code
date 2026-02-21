import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    host: '127.0.0.1',
    port: 3000,
    strictPort: false,
    proxy: {
      '/catalog': {
        target: 'http://127.0.0.1:8080',
        changeOrigin: true
      },
      '/provider': {
        target: 'http://127.0.0.1:8080',
        changeOrigin: true
      }
    }
  }
})
