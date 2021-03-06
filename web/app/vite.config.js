import {defineConfig} from 'vite'
import {svelte} from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    host: '0.0.0.0',
    port: 55555,
    open: true
  },
  // resolve: {
  //   alias: {
  //     '@': process.cwd() + '/src'
  //   },
  // },
})
