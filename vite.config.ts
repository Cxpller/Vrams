import { fileURLToPath, URL } from "node:url";

import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// Tauri 开发时可能通过局域网地址访问（移动端调试），由 CLI 注入
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],

  // 路径别名：@/ 指向 src/，与 tsconfig.json 的 paths 保持一致
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },

  // 以下为 Tauri 专用设置
  // 1. 不清屏，避免挡住 Rust 的编译错误
  clearScreen: false,
  server: {
    // 2. Tauri 需要固定端口，端口被占用时直接报错而不是换端口
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    // 3. 不监听 src-tauri，Rust 代码的变化由 Tauri CLI 自己处理
    watch: { ignored: ["**/src-tauri/**"] },
  },
});
