import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import {VitePWA} from "vite-plugin-pwa"
import mkcert from 'vite-plugin-mkcert'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    mkcert(),
    VitePWA({
      registerType: "autoUpdate",
      manifest: {
        name: "my pwa app",
        short_name: 'myapp',
        description: 'my app and description',
        theme_color: "#ffffff",
        background_color: "#ffffff",
        display: "standalone",
        icons: [
          {
            src: 'icons/128.png',
            sizes: '128x128',
            type: 'image/png',
            purpose: "any"
          },
          {
            src: 'icons/192.png',
            sizes: '192x192',
            type: 'image/png',
            purpose: "any"
          },
          {
            src: 'icons/512.png',
            sizes: '512x512',
            type: 'image/png',
            purpose: "any maskable"
          },
        ]
      },
      devOptions: {
        enabled: true
      }
    })
  ],
})
