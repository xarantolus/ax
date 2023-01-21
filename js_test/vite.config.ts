import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    target: 'es2020'
  },
  server: {
    fs: {
      allow: ['..']
    }
  },
  json: {
    namedExports: false
  }
})
