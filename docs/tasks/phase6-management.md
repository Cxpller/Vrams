# Phase 6: 资产管理操作

**所属**: VRAMS MVP 开发计划
**预估工时**: 1.5 天
**前置依赖**: Phase 5（首页展示完成）
**产出**: 资产详情页、编辑、删除、打开文件夹、导入 Unity

---

## Task 6.1 — 实现 get_asset_detail 命令

### 任务目标

返回单个资产的完整信息（含关联文件 + 关联标签）。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/asset_commands.rs` | 资产命令 |

### 修改内容

```rust
#[tauri::command]
fn get_asset_detail(state: State<AppState>, id: i64) -> Result<AssetDetail, String>

// AssetDetail:
//   id, name, source_url, thumbnail, notes, created_at, updated_at,
//   files: Vec<AssetFile>,
//   tags: Vec<Tag>
```

组合查询：`asset_repo::get_asset_by_id` + `asset_file_repo::get_files_by_asset_id` + `tag_repo::get_tags_by_asset_id`

### 前置依赖

- Phase 2 全部（三个 repo 可用）

### 验收标准

- [ ] 返回的详情包含正确的关联文件和标签
- [ ] ID 不存在时返回 `Err("资产不存在")`
- [ ] 文件列表按 `is_primary DESC, id ASC` 排序

---

## Task 6.2 — 实现 update_asset 命令

### 任务目标

更新资产信息（名称、Booth URL、备注）。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/asset_commands.rs` | 资产命令 |

### 修改内容

```rust
#[tauri::command]
fn update_asset(state: State<AppState>, id: i64, name: Option<String>, source_url: Option<String>, notes: Option<String>) -> Result<Asset, String>
```

### 前置依赖

- Phase 2 Task 2.4

### 验收标准

- [ ] 只更新非 null 字段（部分更新）
- [ ] 成功更新后返回完整 asset
- [ ] ID 不存在时返回错误

---

## Task 6.3 — 实现 delete_asset 命令

### 任务目标

删除资产：数据库记录 + 磁盘文件 + 预览图。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/asset_commands.rs` | 资产命令 |

### 修改内容

```rust
#[tauri::command]
fn delete_asset(state: State<AppState>, library_root: State<LibraryRoot>, id: i64) -> Result<(), String>
```

操作顺序：
1. 从数据库查询 asset 信息（验证存在）
2. 删除磁盘 `assets/{id}/` 目录（调用 utils/file_op.rs）
3. 删除磁盘 `thumbnails/{id}.jpg`（若存在）
4. 删除数据库记录（CASCADE 自动清理关联表和文件记录）

### 前置依赖

- Phase 4 Task 4.6（文件删除操作）

### 验收标准

- [ ] 删除后数据库记录消失（级联清理）
- [ ] 磁盘 `assets/{id}/` 目录被删除
- [ ] 磁盘 `thumbnails/{id}.jpg` 被删除（若存在）
- [ ] 文件不存在时降级（不报错，继续删数据库）
- [ ] ID 不存在时返回错误

---

## Task 6.4 — 实现 open_file_location 命令

### 任务目标

在系统文件管理器中定位并高亮该文件。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/asset_commands.rs` | 资产命令 |

### 修改内容

```rust
#[tauri::command]
fn open_file_location(state: State<AppState>, library_root: State<LibraryRoot>, file_id: i64) -> Result<(), String>

// 1. 查询 asset_files 获取 file_name
// 2. 拼接完整路径: {library_root}/assets/{file_name}
// 3. 调用 opener::open(path.parent()) 或系统 API 打开目录并选中文件
//    Windows: explorer.exe /select,{path}
```

### 前置依赖

- 无

### 验收标准

- [ ] Windows 上打开资源管理器并选中该文件
- [ ] 文件不存在时提示"文件已丢失"
- [ ] 命令不阻塞前端

---

## Task 6.5 — 实现 open_in_unity 命令

### 任务目标

将资产文件导入 Unity。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/asset_commands.rs` | 资产命令 |

### 修改内容

```rust
#[tauri::command]
fn open_in_unity(state: State<AppState>, library_root: State<LibraryRoot>, file_id: i64) -> Result<(), String>
```

逻辑：
1. 查询 asset_files 获取 `file_name` 和 `file_type`
2. 如果是 `.unitypackage` → 直接 `opener::open(full_path)`
3. 如果是 `.zip` 或 `.7z`：
   a. 检查压缩包内是否有 .unitypackage（查 `internal_info` 或重新扫描）
   b. 有 → 提取到临时目录 → `opener::open(temp_unitypackage_path)`
   c. 无 → 返回"未在压缩包内发现 UnityPackage，请手动解压"
4. 若 `opener::open` 失败（无关联程序）→ 返回"未检测到 Unity"

### 前置依赖

- Phase 4（解析器可用）

### 验收标准

- [ ] .unitypackage 文件直接通过 Unity 打开
- [ ] .zip 内含有 .unitypackage 时提取后打开
- [ ] .zip 内无 .unitypackage 时返回提示
- [ ] Unity 未安装时返回提示
- [ ] 临时文件在应用退出时清理

---

## Task 6.6 — 实现资产详情页

### 任务目标

展示资产完整信息，含关联文件列表和操作按钮。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/views/AssetDetail.vue` | 资产详情页 |

### 修改内容

页面结构：
- 顶部：返回按钮 + 资产名称（可编辑）
- 主体：
  - 基本信息区：名称、Booth URL（可点击打开）、备注
  - 标签区：显示已选标签 + "编辑标签"按钮
  - 文件列表区：每个文件一行，展示文件名、大小、类型
    - 每个文件行附带操作按钮：导入 Unity、打开文件夹
  - 预览图区（Phase 7 实现上传）
- 底部操作栏：编辑、删除

数据加载：`onMounted → invoke("get_asset_detail", { id })`

### 前置依赖

- Task 6.1

### 验收标准

- [ ] 正确展示资产基本信息和关联文件
- [ ] 文件列表正确排序（主文件在前）
- [ ] 每个文件有独立操作按钮
- [ ] 返回按钮回到首页

---

## Task 6.7 — 实现编辑弹窗

### 任务目标

资产信息编辑弹窗。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/components/business/AssetEditDialog.vue` | 编辑弹窗 |

### 修改内容

表单字段：名称（必填）、Booth URL（可选）、备注（可选文本域）
保存时调用 `invoke("update_asset", { id, name, sourceUrl, notes })`

### 前置依赖

- Task 6.2

### 验收标准

- [ ] 编辑弹窗预填当前值
- [ ] 名称不能为空（前端校验）
- [ ] 保存后详情页和卡片同步更新

---

## Task 6.8 — 实现删除确认

### 任务目标

删除资产时二次确认，确认后执行删除。

### 涉及文件

使用 Element Plus `ElMessageBox.confirm`，在 AssetDetail.vue 中集成。

### 修改内容

```typescript
// 删除按钮点击时:
ElMessageBox.confirm("将删除此资产及其所有关联文件，不可恢复。", "确认删除", {
  confirmButtonText: "删除",
  cancelButtonText: "取消",
  type: "warning",
}).then(() => {
  await invoke("delete_asset", { id });
  ElMessage.success("已删除");
  router.push({ name: "home" });
});
```

### 前置依赖

- Task 6.3

### 验收标准

- [ ] 点击删除 → 弹出确认对话框
- [ ] 取消 → 不执行操作
- [ ] 确认 → 删除成功 → 跳转回首页

---

## Task 6.9 — 实现标签选择器

### 任务目标

在资产详情中为资产勾选/取消标签。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/components/business/TagSelector.vue` | 标签选择器组件 |

### 修改内容

Props: `assetId: number`, `selectedTags: Tag[]`
UI：弹出面板，展示所有标签按分组排列，已选标签高亮
保存时调用 `invoke("set_asset_tags", { assetId, tagIds })`

### 前置依赖

- Phase 3（tag_repo 和 tag_commands 可用）

### 验收标准

- [ ] 已选标签正确高亮
- [ ] 点击标签切换选中状态
- [ ] 保存后详情页标签区刷新

---

## Phase 6 完整验收 Checklist

- [ ] 6.1 get_asset_detail 命令（含文件和标签）
- [ ] 6.2 update_asset 命令（部分更新）
- [ ] 6.3 delete_asset 命令（数据库 + 磁盘文件清理）
- [ ] 6.4 open_file_location 命令（资源管理器定位）
- [ ] 6.5 open_in_unity 命令（直接打开 / 提取后打开）
- [ ] 6.6 AssetDetail.vue 详情页（信息展示 + 文件列表 + 操作按钮）
- [ ] 6.7 AssetEditDialog.vue 编辑弹窗
- [ ] 6.8 删除确认对话框（ElMessageBox.confirm）
- [ ] 6.9 TagSelector.vue 标签选择器
- [ ] 全流程：首页 → 点卡片 → 详情 → 编辑/标签/删除 → 回首页

---

## Phase 6 Review（审查环节）

**触发时机**: 本 Phase 全部 Task 完成、验证通过后执行（标准流程第⑤步）。建议对 Task 6.3（delete_asset 级联清理）单独追加一次 review。

**审查方式**: 调用 `review` skill（`.claude/skills/review/SKILL.md`），审查范围为 Phase 6 的全部变更。

**审查重点**:

- [ ] `delete_asset` 级联清理一致性（数据库 + `assets/{id}/` + `thumbnails/{id}.jpg`），部分失败时降级且不产生孤立数据
- [ ] 删除前二次确认（`ElMessageBox.confirm`）
- [ ] `open_in_unity` 提取 .unitypackage 到临时目录后清理，不残留临时文件
- [ ] 文件缺失时友好提示（"文件已移动或删除"）
- [ ] 标签选择器多选/搜索状态正确

**输出**: 审查报告（结论 + 问题清单 + 验收核对表）。S1 阻断问题未清零前，不允许 git commit / 交付下一 Phase。
