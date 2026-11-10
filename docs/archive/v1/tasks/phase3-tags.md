# Phase 3: 标签系统（全栈验证）

**所属**: VRAMS MVP 开发计划
**预估工时**: 1 天
**前置依赖**: Phase 2 完成（数据库 + AppState 就位）
**产出**: 标签页面可创建/编辑/删除标签，验证 Vue → invoke → Rust → SQLite 全链路

---

## 概要

标签系统是 MVP 中最简单的全栈功能，用来**验证整个技术栈的通断**。如果 Phase 3 跑通，后续 Phase 4~9 的架构风险就大大降低。

---

## Task 3.1 — 实现 Rust Tag Commands

### 任务目标

实现标签 CRUD 的四个 Tauri command 函数。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/tag_commands.rs` | 标签命令实现 |

### 修改内容

实现以下 `#[tauri::command]` 函数：

```rust
// get_tags(state: State<AppState>) -> Result<Vec<Tag>, String>
//   查询所有标签（用于标签管理页 + 资产编辑时的标签选择器）
//   调用 tag_repo::get_all_tags

// create_tag(state, name: String, group_name: Option<String>) -> Result<Tag, String>
//   创建新标签
//   校验: name 非空、去重
//   调用 tag_repo::insert_tag

// update_tag(state, id: i64, name: Option<String>, group_name: Option<String>) -> Result<Tag, String>
//   更新标签名或分组
//   调用 tag_repo::update_tag

// delete_tag(state, id: i64) -> Result<(), String>
//   删除标签，自动清理关联关系
//   调用 tag_repo::delete_tag
```

### 前置依赖

- Phase 2 Task 2.6（tag_repo 就绪）
- Phase 2 Task 2.7（AppState 就绪）

### 验收标准

- [x] 四个 command 编译通过，可被 Tauri 注册
- [x] 每个 command 内部先 `state.db.lock()` 再调 repo
- [x] 错误统一返回 `Err(e.to_string())`
- [x] `create_tag` 对重复名称返回友好错误

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| `Mutex` 使用不当 | command 间死锁 | 确保所有 command 中使用 `let conn = state.db.lock().unwrap()` 不嵌套 |
| Tauri command 参数类型不匹配 | invoke 调用失败 | command 参数类型需与前端 `invoke` 传入的 JS 对象对齐 |

---

## Task 3.2 — 注册 Tag Commands

### 任务目标

将 tag commands 注册到 Tauri 的 invoke_handler 中。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/lib.rs` | Tauri Builder 注册入口 |

### 修改内容

1. 在 `lib.rs` 中添加 `mod commands;`
2. 在 `generate_handler![]` 中添加四个 tag command
3. 添加 `commands::tag_commands::*` 引入

### 前置依赖

- Task 3.1

### 验收标准

- [x] `cargo check` 通过
- [x] 启动后前端可通过 `invoke("get_tags")` 调用成功（返回空数组）

---

## Task 3.3 — 创建前端 API 层

### 任务目标

封装 tag 相关的 Tauri invoke 调用，提供 TypeScript 类型安全的 API 函数。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/api/tags.ts` | 标签 API 封装 |

### 修改内容

创建 `src/api/tags.ts`：

```typescript
// 导出类型
export interface Tag {
  id: number;
  name: string;
  group_name: string;
  created_at: string;
}

export interface CreateTagRequest {
  name: string;
  group_name?: string;
}

export interface UpdateTagRequest {
  name?: string;
  group_name?: string;
}

// 导出函数
export async function getTags(): Promise<Tag[]>;
export async function createTag(req: CreateTagRequest): Promise<Tag>;
export async function updateTag(id: number, req: UpdateTagRequest): Promise<Tag>;
export async function deleteTag(id: number): Promise<void>;
```

每个函数内部调用 `invoke("xxx", { ... })`。

### 前置依赖

- Task 3.2（后端 command 已注册）

### 验收标准

- [x] 类型定义与 Rust 后端返回的 JSON 结构一致
- [x] `invoke` 调用参数名与 Rust command 参数名一致（Tauri 会将参数名转为 snake_case）
- [x] 异步函数可在 `onMounted` 等 Vue 生命周期中调用

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| Rust 侧参数名是 snake_case，前端传 camelCase | Rust 收不到参数 | Tauri 会自动转换，但需确认 `invoke("create_tag", { name, groupName })` 对应 Rust `name: String, group_name: Option<String>` |

---

## Task 3.4 — 创建 Pinia Store

### 任务目标

创建标签状态管理 store，管理标签列表和选中状态。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/stores/tagStore.ts` | 标签状态仓库 |

### 修改内容

```typescript
// 定义 store
export const useTagStore = defineStore("tags", () => {
  // state
  const tags = ref<Tag[]>([]);
  const loading = ref(false);

  // actions
  async function fetchTags(): Promise<void>;
  async function addTag(req: CreateTagRequest): Promise<Tag>;
  async function editTag(id: number, req: UpdateTagRequest): Promise<void>;
  async function removeTag(id: number): Promise<void>;

  // getters (用 computed)
  const tagsByGroup: ComputedRef<Record<string, Tag[]>>;  // 按 group_name 分组
  const tagOptions: ComputedRef<{ label: string; value: number }[]>;  // 用于选择器

  return { tags, loading, tagsByGroup, tagOptions, fetchTags, addTag, editTag, removeTag };
});
```

### 前置依赖

- Task 3.3（API 层可用）

### 验收标准

- [x] `fetchTags` 后 `tags` 正确更新
- [x] `addTag` 后列表自动追加
- [x] `removeTag` 后列表中移除
- [x] `tagsByGroup` 按 group_name 分组正确

---

## Task 3.5 — 创建标签管理页面

### 目标

实现标签管理页面：标签列表展示、新建、编辑、删除对话框。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/views/TagManage.vue` | 标签管理页面 |
| `src/components/business/TagEditDialog.vue` | 标签编辑弹窗 |

### 修改内容

**`TagManage.vue`**:
- 页面布局：左侧分组列表（按 group_name 分组），右侧标签列表
- 顶部"新建标签"按钮
- 每个标签项附带编辑和删除按钮
- 删除带二次确认（Element Plus `ElMessageBox.confirm`）

**`TagEditDialog.vue`**:
- 表单：标签名输入（必填）+ 分组选择/输入（可选）
- 编辑时预填当前值
- 保存后调用 store 对应 action

### 前置依赖

- Task 3.4（store 可用）
- Phase 1 Task 1.5（路由已配置）

### 验收标准

- [x] 页面加载时自动调用 `fetchTags`
- [x] 新建标签 → 输入名称 → 选择分组 → 保存 → 列表刷新
- [x] 编辑标签 → 预填 → 修改 → 保存 → 列表更新
- [x] 删除标签 → 确认 → 列表移除
- [x] 标签按分组展示（"角色"组下显示 Kikyo/Mafuyu 等）

---

## Task 3.6 — 配置路由

### 任务目标

为标签管理页面添加路由。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/router/index.ts` | 路由配置 |

### 修改内容

```typescript
const routes = [
  { path: "/", name: "home", component: () => import("@/views/HomePage.vue") },
  { path: "/tags", name: "tags", component: () => import("@/views/TagManage.vue") },
  { path: "/asset/:id", name: "asset-detail", component: () => import("@/views/AssetDetail.vue") },
];
```

Phase 3 先配 `/tags` 和占位 `/`，`/asset/:id` 留到 Phase 6。

### 前置依赖

- Task 3.5（页面文件已存在）

### 验收标准

- [x] 访问 `/tags` 可看到标签管理页面
- [x] 路由懒加载正常工作（`() => import(...)`）

---

## Phase 3 完整验收 Checklist

- [x] 3.1 四个 tag command（get/create/update/delete）在 Rust 端实现
- [x] 3.2 在 lib.rs 中注册，`cargo check` 通过
- [x] 3.3 前端 API 层封装完成，类型定义对齐后端
- [x] 3.4 Pinia store 实现，含分组 getter
- [x] 3.5 标签管理页面可创建/编辑/删除标签，可分组展示
- [x] 3.6 路由配置完成
- [x] **全栈链路验证通过**：Vue invoke → Rust command → SQLite → 回显前端
- [x] 工程风险已释放，可继续 Phase 4

---

## Phase 3 Review（审查环节）

**触发时机**: 本 Phase 全部 Task 完成、验证通过后执行（标准流程第⑤步）。

**审查方式**: 调用 `review` skill（`.claude/skills/review/SKILL.md`），审查范围为 Phase 3 的全部变更。

**审查重点**:

- [x] 全栈链路验证：Vue invoke → Rust command → SQLite → 回显前端
- [x] 命令三步注册齐全（`commands/` 模块 → `lib.rs` `generate_handler!` → 前端 `invoke`）
- [x] 前端错误处理：invoke 失败 → `ElMessage` 提示，不静默
- [x] 类型安全：`Tag` 类型前后端对齐，无 `any`
- [x] 标签分组展示 + 空状态处理

**输出**: 审查报告（结论 + 问题清单 + 验收核对表）。S1 阻断问题未清零前，不允许 git commit / 交付下一 Phase。

**Phase 3 审查结论（2026-08-12）**: 两次 review 均通过，无 S1/S2 阻断。第一次发现 3 项 S3 代码问题（groupOptions 泄漏「未分组」键、handleSave 格式、后端缺 name 长度校验）已修复；复审确认修复正确无回归。另修复冒烟中暴露的 ElMessageBox 按需引入样式缺失（main.ts 显式补 message/message-box 样式）。剩余 S3（DB 层 name 长度约束、错误提示前缀、分组名恰为「全部」的边界）登记入 `.claude/CONTEXT.md` 遗留问题。
