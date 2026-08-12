import { createApp } from "vue";
import { createPinia } from "pinia";

// Element Plus 按需引入：模板组件样式由 unplugin-vue-components 自动注入，
// 但 ElMessage/ElMessageBox 这类函数式 API 的样式不会自动跟随，
// 需在此显式引入（否则弹窗/toast 无样式，内容堆在左上角）
import "element-plus/es/components/message/style/css";
import "element-plus/es/components/message-box/style/css";

import App from "@/App.vue";
import router from "@/router";

// Element Plus 按需引入：组件由 unplugin-vue-components 在构建时自动 import，
// 无需在此全局注册（见 vite.config.ts 的 AutoImport/Components 插件）
createApp(App).use(createPinia()).use(router).mount("#app");
