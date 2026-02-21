import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    proxy: {
      '/catalog': 'http://localhost:8080',
      '/provider': 'http://localhost:8080'
    }
  }
})
