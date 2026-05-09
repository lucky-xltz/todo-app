import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src')
    }
  },
  // Tauri 需要固定端口
  server: {
    port: 5173,
    strictPort: true
  },
  // 确保 Tauri 可以访问
  clearScreen: false,
  envPrefix: ['VITE_', 'TAURI_']
})