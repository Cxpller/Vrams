<script setup lang="ts">
// 新建/编辑标签弹窗
// props.modelValue 控制显隐; tag 为 null = 新建, 非 null = 编辑(预填)
// 保存走 store 的 addTag/editTag, 弹窗自身不直接改 store 数据
import { computed, reactive, ref, watch } from "vue";
import { ElMessage } from "element-plus";

import { useTagStore } from "@/stores/tagStore";
import type { Tag } from "@/types/tag";

const props = defineProps<{
  modelValue: boolean;
  tag: Tag | null;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
}>();

const store = useTagStore();

// ---- 表单状态 ----
const form = reactive({
  name: "",
  group_name: "",
});
const saving = ref(false);

/** 已有分组名列表(仅真实分组,不含 UI 合成的"未分组"),供 el-select 选择 */
const groupOptions = computed(() => {
  const groups = new Set<string>();
  for (const t of store.tags) {
    const g = t.group_name.trim();
    if (g) groups.add(g);
  }
  return [...groups];
});

const title = computed(() => (props.tag ? "编辑标签" : "新建标签"));

// 弹窗打开时:编辑 → 预填当前值;新建 → 清空
watch(
  () => props.modelValue,
  (open) => {
    if (!open) return;
    form.name = props.tag?.name ?? "";
    form.group_name = props.tag?.group_name ?? "";
  },
);

/** 弹窗关闭事件:透传给父组件(v-model 的 update:modelValue) */
function handleUpdateModelValue(v: boolean) {
  emit("update:modelValue", v);
}

async function handleSave() {
  const name = form.name.trim();
  if (!name) {
    ElMessage.warning("标签名不能为空");
    return;
  }

  saving.value = true;
  try {
    if (props.tag) {
      // 编辑:group_name 原样传(空串表示清空分组,由后端更新)
      await store.editTag(props.tag.id, { name, group_name: form.group_name });
    } else {
      // 新建:分组留空则不传,后端落库为空串
      await store.addTag({ name, group_name: form.group_name || undefined });
    }
    ElMessage.success(props.tag ? "标签已更新" : "标签已创建");
    emit("update:modelValue", false);
  } catch (e) {
    ElMessage.error(String(e));
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <el-dialog
    :model-value="modelValue"
    :title="title"
    width="420px"
    :close-on-click-modal="false"
    @update:model-value="handleUpdateModelValue"
  >
    <el-form label-width="64px" @submit.prevent>
      <el-form-item label="标签名" required>
        <el-input
          v-model="form.name"
          placeholder="如：Kikyo、已购买"
          maxlength="32"
          show-word-limit
          @keyup.enter="handleSave"
        />
      </el-form-item>
      <el-form-item label="分组">
        <el-select
          v-model="form.group_name"
          placeholder="选择已有分组或输入新分组"
          filterable
          allow-create
          default-first-option
          clearable
          style="width: 100%"
        >
          <el-option v-for="g in groupOptions" :key="g" :label="g" :value="g" />
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="emit('update:modelValue', false)">取消</el-button>
      <el-button type="primary" :loading="saving" @click="handleSave">保存</el-button>
    </template>
  </el-dialog>
</template>
