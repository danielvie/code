# Install tailwind with vite

01. install tailwind
```powershell
bun install tailwindcss @tailwindcss/vite
```

02. configure the vite plugin
```ts
import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [
    tailwindcss(),
  ],
})
```

03. import tailwind CSS: add on `@import` to your CSS file that imports tailwindC CSS
```ts
@import "tailwindcss";
```