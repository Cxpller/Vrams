<script setup lang="ts">
// 标签管理页面
// 左侧分组列表,右侧标签列表;支持新建/编辑/删除
import { computed, onMounted, ref } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";

import TagEditDialog from "@/components/business/TagEditDialog.vue";
import { useTagStore } from "@/stores/tagStore";
import type { Tag } from "@/types/tag";

const store = useTagStore();

// ---- 弹窗状态 ----
const dialogVisible = ref(false);
const editingTag = ref<Tag | null>(null);

// ---- 分组筛选 ----
/** "全部" 或某个分组名 */
const activeGroup = ref("全部");
const groupNames = computed(() => ["全部", ...Object.keys(store.tagsByGroup)]);

/** 当前分组下的标签列表 */
const visibleTags = computed(() => {
  if (activeGroup.value === "全部") return store.tags;
  return store.tagsByGroup[activeGroup.value] ?? [];
});

onMounted(() => {
  store.fetchTags().catch((e) => ElMessage.error(String(e)));
});

function openCreate() {
  editingTag.value = null;
  dialogVisible.value = true;
}

function openEdit(tag: Tag) {
  editingTag.value = tag;
  dialogVisible.value = true;
}

async function handleDelete(tag: Tag) {
  try {
    await ElMessageBox.confirm(`确定删除标签「${tag.name}」吗？`, "删除确认", {
      type: "warning",
      confirmButtonText: "删除",
      cancelButtonText: "取消",
    });
  } catch {
    return; // 用户点了取消
  }
  try {
    await store.removeTag(tag.id);
    ElMessage.success("标签已删除");
  } catch (e) {
    ElMessage.error(String(e));
  }
}
</script>

<template>
  <div class="tag-page">
    <header class="toolbar">
      <h2>标签管理</h2>
      <el-button type="primary" @click="openCreate">新建标签</el-button>
    </header>

    <div class="body">
      <!-- 左侧:分组列表 -->
      <aside class="group-panel">
        <div
          v-for="g in groupNames"
          :key="g"
          class="group-item"
          :class="{ active: activeGroup === g }"
          @click="activeGroup = g"
        >
          <span>{{ g }}</span>
          <span class="count">
            {{ g === "全部" ? store.tags.length : (store.tagsByGroup[g]?.length ?? 0) }}
          </span>
        </div>
      </aside>

      <!-- 右侧:标签列表 -->
      <main class="tag-panel" v-loading="store.loading">
        <el-empty v-if="!store.loading && visibleTags.length === 0" description="暂无标签，点击右上角新建" />
        <div v-else class="tag-list">
          <div v-for="tag in visibleTags" :key="tag.id" class="tag-item">
            <el-tag>{{ tag.name }}</el-tag>
            <div class="actions">
              <el-button size="small" @click="openEdit(tag)">编辑</el-button>
              <el-button size="small" type="danger" plain @click="handleDelete(tag)">删除</el-button>
            </div>
          </div>
        </div>
      </main>
    </div>

    <TagEditDialog v-model="dialogVisible" :tag="editingTag" />
  </div>
</template>

<style scoped>
.tag-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px;
  box-sizing: border-box;
  gap: 12px;
}

.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.toolbar h2 {
  margin: 0;
  font-size: 18px;
}

.body {
  display: flex;
  flex: 1;
  min-height: 0;
  gap: 12px;
}

.group-panel {
  width: 160px;
  flex-shrink: 0;
  border: 1px solid var(--el-border-color-light);
  border-radius: 6px;
  overflow-y: auto;
}

.group-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  cursor: pointer;
  font-size: 14px;
}

.group-item:hover {
  background-color: var(--el-fill-color-light);
}

.group-item.active {
  background-color: var(--el-color-primary-light-9);
  color: var(--el-color-primary);
  font-weight: 600;
}

.group-item .count {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.tag-panel {
  flex: 1;
  border: 1px solid var(--el-border-color-light);
  border-radius: 6px;
  overflow-y: auto;
  padding: 12px;
}

.tag-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.tag-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border: 1px solid var(--el-border-color-lighter);
  border-radius: 6px;
}

.tag-item:hover {
  background-color: var(--el-fill-color-light);
}

.actions {
  display: flex;
  gap: 4px;
}
</style>
