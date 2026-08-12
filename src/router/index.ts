// 路由配置
// Phase 3 先配 / 与 /tags,/asset/:id 等留到后续 Phase
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("@/views/HomePage.vue"),
    },
    {
      path: "/tags",
      name: "tags",
      component: () => import("@/views/TagManage.vue"),
    },
  ],
});

export default router;
