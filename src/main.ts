import { createApp } from "vue";
import App from "@/App.vue";
import router from "@/router";

// Element Plus 按需引入：组件由 unplugin-vue-components 在构建时自动 import，
// 无需在此全局注册（见 vite.config.ts 的 AutoImport/Components 插件）
createApp(App).use(router).mount("#app");
