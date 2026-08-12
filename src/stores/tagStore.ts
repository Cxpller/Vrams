//! 标签状态管理 store — 全局共享标签列表，供管理页与资产编辑选择器使用
//!
//! setup 风格：ref = state，computed = getters，函数 = actions。

import { computed, ref } from "vue";
import { defineStore } from "pinia";

import {
  createTag as apiCreateTag,
  deleteTag as apiDeleteTag,
  getTags,
  updateTag as apiUpdateTag,
} from "@/api/tags";
import type { CreateTagRequest, Tag, UpdateTagRequest } from "@/types/tag";

export const useTagStore = defineStore("tags", () => {
  // ---- state ----
  /** 全部标签列表 */
  const tags = ref<Tag[]>([]);
  /** 是否正在加载 */
  const loading = ref(false);

  // ---- actions ----
  /** 从后端拉取全部标签 */
  async function fetchTags(): Promise<void> {
    loading.value = true;
    try {
      tags.value = await getTags();
    } finally {
      loading.value = false;
    }
  }

  /** 新建标签，成功后追加到列表 */
  async function addTag(req: CreateTagRequest): Promise<Tag> {
    const tag = await apiCreateTag(req);
    tags.value.push(tag);
    return tag;
  }

  /** 编辑标签，成功后替换列表中的对应项 */
  async function editTag(id: number, req: UpdateTagRequest): Promise<void> {
    const updated = await apiUpdateTag(id, req);
    const idx = tags.value.findIndex((t) => t.id === id);
    if (idx !== -1) {
      tags.value[idx] = updated;
    }
  }

  /** 删除标签，成功后从列表移除 */
  async function removeTag(id: number): Promise<void> {
    await apiDeleteTag(id);
    tags.value = tags.value.filter((t) => t.id !== id);
  }

  // ---- getters ----
  /** 按 group_name 分组；空分组归入「未分组」 */
  const tagsByGroup = computed<Record<string, Tag[]>>(() => {
    const groups: Record<string, Tag[]> = {};
    for (const t of tags.value) {
      const key = t.group_name.trim() || "未分组";
      (groups[key] ??= []).push(t);
    }
    return groups;
  });

  /** 用于下拉选择器：label=标签名，value=标签 id */
  const tagOptions = computed<{ label: string; value: number }[]>(() =>
    tags.value.map((t) => ({ label: t.name, value: t.id })),
  );

  return {
    tags,
    loading,
    tagsByGroup,
    tagOptions,
    fetchTags,
    addTag,
    editTag,
    removeTag,
  };
});
