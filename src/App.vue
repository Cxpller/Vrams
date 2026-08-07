<script setup lang="ts">
// 根组件：仅承载路由出口。
// 具体页面（资产列表、标签管理、设置等）在 src/views/ 中实现，
// 由 src/router/index.ts 的路由表驱动渲染。
</script>

<template>
  <router-view />
</template>

<style>
@import "./styles/variables.css";

/* 全局基础样式：让 #app 撑满窗口，页面布局自己管理高度 */
html,
body,
#app {
  height: 100%;
  margin: 0;
  background-color: var(--vrams-bg);
  color: var(--vrams-text);
  font-family: Inter, "Segoe UI", "Microsoft YaHei", Arial, sans-serif;
  font-size: var(--vrams-font-md);
}
</style>
