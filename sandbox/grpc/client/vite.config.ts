import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { viteCommonjs } from "@originjs/vite-plugin-commonjs";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    viteCommonjs({
      include: ['src/generated/**/*.js'],
    }),
    react(), 
  ],
  server: {
    port: 5174,
  },
  optimizeDeps: {
    include: ['google-protobuf', 'grpc-web'],
  },
  define: {
    // gRPC-web and google-protobuf rely on 'global'
    global: 'window',
  },
})
