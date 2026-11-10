# Phase 4: 资产导入（核心功能）

**所属**: VRAMS MVP 开发计划
**预估工时**: 3 天
**前置依赖**: Phase 2（数据库）+ Phase 3（全栈验证通过）
**产出**: 用户可拖拽 .zip/.7z/.unitypackage 进入应用，经导入审查后复制到库目录

---

## Task 4.1 — 实现 zip 解析器

### 任务目标

读取 .zip 文件内部的文件列表（不解压到磁盘），返回目录树结构。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/parsers/zip_archive.rs` | zip 解析器 |

### 修改内容

```rust
// scan_zip(path: &Path) -> Result<ParsedArchive, String>
//   返回结构: ParsedArchive { files: Vec<FileEntry> }
//     FileEntry { path: String, is_directory: bool, size: i64 }
//   使用 zip crate: ZipArchive + file().name() 遍历
//   不解压文件内容，只读文件名和大小
```

### 前置依赖

- Phase 1 Task 1.1（zip crate 已安装）

### 验收标准

- [ ] 传入合法 .zip，返回正确的文件列表
- [ ] 传入损坏 .zip，返回友好错误信息
- [ ] 传入非 .zip 文件，返回格式错误提示
- [ ] 读取耗时 ≤ 1s（1000 文件以内）
- [ ] 不产生任何磁盘写入（只读）

---

## Task 4.2 — 实现 7z 解析器

### 任务目标

读取 .7z 文件内部的文件列表（不解压到磁盘），返回目录树结构。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/parsers/sevenz_archive.rs` | 7z 解析器 |

### 修改内容

```rust
// scan_sevenz(path: &Path) -> Result<ParsedArchive, String>
//   返回结构与 scan_zip 一致
//   使用 sevenz-rust crate 或调用 7z CLI
//   优先尝试 sevenz-rust，如不可用则回退到命令行调用
```

### 前置依赖

- Phase 1 Task 1.1

### 验收标准

- [ ] 传入合法 .7z，返回正确的文件列表
- [ ] 传入损坏 .7z，返回友好错误信息
- [ ] 与 zip 解析器返回相同的数据结构（可互换）

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| `sevenz-rust` crate 不成熟或编译失败 | 无法解析 7z | 备选方案：调用 `7z.exe l` CLI 命令解析 |

---

## Task 4.3 — 实现 unitypackage 解析器

### 任务目标

读取 .unitypackage 内部 pathname 文件列表，重建目录树。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/parsers/unitypackage.rs` | unitypackage 解析器 |

### 修改内容

```rust
// scan_unitypackage(path: &Path) -> Result<ParsedArchive, String>
//   流程:
//   1. 打开文件作为 gzip 流 (flate2::read::GzDecoder)
//   2. 读取 tar 存档 (tar::Archive)
//   3. 遍历所有条目，找到以 "/pathname" 结尾的条目
//   4. 读取文件内容（纯文本），得到原始路径如 "Assets/Models/Kikyo.fbx"
//   5. 构建 FileEntry 列表（不包含 asset 和 asset.meta 的条目）
//   6. 返回 ParsedArchive
```

### 前置依赖

- Phase 1 Task 1.1（tar, flate2 已安装）

### 验收标准

- [ ] 能正确解析标准 .unitypackage 结构
- [ ] 只读取 pathname 文件，不读取 asset/asset.meta
- [ ] 返回的路径以 `Assets/` 为根
- [ ] 文件损坏时返回可读的错误信息

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| .unitypackage 格式非标准 | 解析失败 | 大部分 Booth 下载的包为标准格式；遇到非标准格式时返回"无法解析" |

---

## Task 4.4 — 实现统一解析入口

### 任务目标

根据文件类型自动分发到对应的解析器，对外暴露统一接口。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/parsers/mod.rs` | 解析器统一入口 |

### 修改内容

```rust
// scan_archive(path: &Path) -> Result<ScanResult, String>
//   自动检测文件类型:
//     后缀 .zip         → scan_zip
//     后缀 .7z          → scan_sevenz
//     后缀 .unitypackage → scan_unitypackage
//     其他              → Err("不支持的文件格式")

// ScanResult struct:
//   file_type: String  ("zip" | "7z" | "unitypackage")
//   file_size: i64
//   files: Vec<FileEntry>
//   has_unitypackage: bool  // 压缩包内是否含 .unitypackage（用于导入 Unity 判断）
```

### 前置依赖

- Task 4.1, 4.2, 4.3

### 验收标准

- [ ] 传入 .zip → 调用 zip 解析器
- [ ] 传入 .7z → 调用 7z 解析器
- [ ] 传入 .unitypackage → 调用 unitypackage 解析器
- [ ] 传入不支持的格式 → 返回"不支持的文件格式"错误
- [ ] `has_unitypackage` 在扫描时正确标记

---

## Task 4.5 — 实现 preview_import 命令

### 任务目标

接收文件路径列表，扫描每个文件，返回预览信息供前端展示。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/import_commands.rs` | 导入命令 |

### 修改内容

```rust
#[tauri::command]
fn preview_import(state: State<AppState>, paths: Vec<String>) -> Result<Vec<PreviewItem>, String>

// PreviewItem struct (Serialize):
//   index: i64
//   file_name: String
//   file_size: i64
//   file_type: String ("zip"|"7z"|"unitypackage")
//   internal_structure: Vec<FileEntry>  // 文件树
//   has_unitypackage: bool
//   is_duplicate: bool  // 数据库是否已存在同名+同大小文件
```

### 前置依赖

- Task 4.4（解析器统一入口）
- Phase 2 Task 2.4（asset_repo 用于重复检测）

### 验收标准

- [ ] 传入路径数组，返回对应数量的 PreviewItem
- [ ] 每个 item 包含完整的内部文件列表
- [ ] 重复检测正确（同名+同大小视为重复）
- [ ] 路径不存在时返回错误而非 panic

---

## Task 4.6 — 实现文件复制逻辑

### 任务目标

将导入文件从源位置复制到库目录 `assets/{id}/`。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/utils/file_op.rs` | 文件操作工具 |

### 修改内容

```rust
// copy_file_to_library(source: &Path, library_root: &Path, asset_id: i64) -> Result<String, String>
//   1. 创建 library_root/assets/{asset_id}/ 目录
//   2. 复制文件到该目录，保留原始文件名
//   3. 重名冲突时自动追加编号: "file (2).zip"
//   4. 返回相对路径: "{asset_id}/filename.zip"

// delete_asset_files(library_root: &Path, asset_id: i64) -> Result<(), String>
//   删除 library_root/assets/{asset_id}/ 整个目录
```

### 前置依赖

- 无

### 验收标准

- [ ] 文件成功复制到目标目录
- [ ] 文件名正确保留（含中文/空格/Unicode 字符）
- [ ] 重名冲突时自动编号
- [ ] 目标目录不存在时自动创建

---

## Task 4.7 — 实现回收站操作

### 任务目标

将原始文件移至系统回收站（非直接删除）。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/utils/recycle.rs` | 回收站操作 |

### 修改内容

```rust
// move_to_recycle(path: &Path) -> Result<(), String>
//   通过 Windows API IFileOperation 或 shell32::SHFileOperationW 实现
//   使用 winapi crate 或调用 recycle 命令行工具
//   非 Windows 平台回退为直接删除
```

### 前置依赖

- 无

### 验收标准

- [ ] 调用后文件出现在系统回收站中
- [ ] 文件可正常从回收站恢复
- [ ] 传入不存在文件时静默跳过
- [ ] 无权限时返回可读错误

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| Windows API 绑定复杂 | 实现时间超预期 | 备选：直接 `std::fs::remove_file` 并提示用户（MVP 可接受） |

---

## Task 4.8 — 实现导入业务逻辑服务

### 任务目标

组合解析→去重→复制→写库→（可选）删除源文件的完整链路。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/services/import_service.rs` | 导入业务逻辑 |

### 修改内容

```rust
// execute_import(
//     conn: &Connection,
//     library_root: &Path,
//     groups: Vec<ImportGroup>,
//     paths: &[String],
//     delete_originals: bool,
// ) -> Result<Vec<ImportResult>, String>

// ImportGroup: { name, source_url?, file_indices: Vec<usize> }
// ImportResult: { asset_id, name, file_count }
```

业务逻辑流程：
1. 为每个 group 创建一条 assets 记录
2. 为该 group 关联的每个文件创建 asset_files 记录
3. 复制文件到 `assets/{asset_id}/`
4. 若 `delete_originals` → 调用 recycle
5. 若 `source_url` 有值 → 标记待下载预览图（Phase 7 实现）

### 前置依赖

- Task 4.6（文件复制）
- Task 4.7（回收站）
- Phase 2 全部（数据库操作）

### 验收标准

- [ ] 单个 group → 一条 asset + N 条 asset_files
- [ ] 多个 group → 多条 asset
- [ ] 事务保证：中途失败时回滚（文件复制无法事务，需手动清理）
- [ ] 删除源文件选项正确生效

---

## Task 4.9 — 实现 confirm_import 命令

### 任务目标

前端确认导入后调用的入口命令。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/import_commands.rs` | 导入命令 |

### 修改内容

```rust
#[tauri::command]
fn confirm_import(
    state: State<AppState>,
    library_root: State<LibraryRoot>,  // 库目录路径
    groups: Vec<ImportGroup>,
    paths: Vec<String>,
    delete_originals: bool,
) -> Result<Vec<ImportResult>, String>
```

内部调用 `import_service::execute_import`。

### 前置依赖

- Task 4.8

### 验收标准

- [ ] 命令返回正确的导入结果（含 asset_id）
- [ ] 错误时事务回滚，已复制的文件清理
- [ ] 返回结果中包含足够的 asset_id 列表供前端刷新

---

## Task 4.10 — 实现导入审查窗口

### 任务目标

拖入文件后弹出导入审查窗口，展示文件信息、包内结构、供用户编辑。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/components/business/ImportReviewDialog.vue` | 导入审查弹窗 |

### 修改内容

UI 结构：
- 文件列表区域：每个文件一行，显示文件名、大小、类型图标
- 勾选 checkbox + "合并选中" / "拆开" 按钮
- 合并后展示折叠组，组内可展开查看文件
- 每行/每组：商品名输入框、Booth URL 输入框
- 底部：删除源文件 checkbox（默认勾选）+ 确认/取消按钮

交互逻辑：
- 接收 `PreviewItem[]` 作为 prop
- 内部维护 `groups` 状态（用户可调整分组）
- 确认时 emit `confirm(groups, deleteOriginals)`
- 取消时 emit `cancel()`

### 前置依赖

- Task 4.5（preview_import 命令）

### 验收标准

- [ ] 窗口正确展示所有文件预览信息
- [ ] 勾选 + 合并选中 → 文件折叠为组
- [ ] 拆开 → 组恢复为独立行
- [ ] 商品名输入框有默认值（文件名）
- [ ] Booth URL 输入框可选
- [ ] 删除源文件 checkbox 默认勾选
- [ ] 确认按钮触发回调

---

## Task 4.11 — 实现拖拽导入

### 任务目标

用户从文件管理器拖拽文件到应用窗口时触发导入流程。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/App.vue` | 根组件，监听拖拽事件 |

### 修改内容

使用 Tauri v2 的 `onDragDropEvent` 监听：

```typescript
// 在 App.vue onMounted 中注册
import { onDragDropEvent } from "@tauri-apps/api/webview";

onDragDropEvent((event) => {
  if (event.payload.type === "drop") {
    // 获取 event.payload.paths
    // 调用 preview_import(paths)
    // 弹出 ImportReviewDialog
  }
});
```

### 前置依赖

- Task 4.10（导入审查窗口已实现）

### 验收标准

- [ ] 从 Windows 文件管理器拖拽文件到窗口 → 触发导入流程
- [ ] 拖入非支持格式 → 提示"暂不支持此格式"
- [ ] 拖入多个文件 → 全部显示在导入审查窗口

---

## Task 4.12 — 实现文件选择器导入

### 任务目标

用户点击"导入"按钮后通过系统文件选择器选择文件。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/components/common/ImportButton.vue` | 导入按钮组件 |

### 修改内容

使用 `tauri-plugin-dialog` 打开文件选择器：

```typescript
import { open } from "@tauri-apps/plugin-dialog";

// 点击按钮：
// const files = await open({ multiple: true, filters: [...] });
// 调用 preview_import(files)
// 弹出 ImportReviewDialog
```

文件过滤器：`.unitypackage`, `.zip`, `.7z`

### 前置依赖

- Phase 1 Task 1.7（tauri-plugin-dialog 权限已配置）
- Task 4.10

### 验收标准

- [ ] 点击按钮后弹出系统文件选择器
- [ ] 可选择单个或多个文件
- [ ] 文件过滤器正确显示
- [ ] 选择后进入导入审查流程

---

## Task 4.13 — 注册 Import Commands

### 任务目标

将 import commands 注册到 Tauri 中。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/lib.rs` | 注册入口 |

### 修改内容

在 `generate_handler![]` 中添加 `preview_import` 和 `confirm_import`。

### 前置依赖

- Task 4.5, 4.9

### 验收标准

- [ ] `cargo check` 通过
- [ ] 前端可调用两个命令

---

## Phase 4 完整验收 Checklist

- [ ] 4.1~4.4 三种格式解析器 + 统一入口完成
- [ ] 4.5 preview_import 命令可用
- [ ] 4.6 文件复制到库目录正确
- [ ] 4.7 回收站操作可用（含备选方案）
- [ ] 4.8 导入业务逻辑完整（事务+回滚）
- [ ] 4.9 confirm_import 命令可用
- [ ] 4.10 导入审查窗口 UI 完整
- [ ] 4.11 拖拽导入可用
- [ ] 4.12 点击按钮选择文件可用
- [ ] **完整导入流程通过**: 拖拽 → 预览 → 命名 → 确认 → 文件入库 → 卡片刷新
- [ ] 可向 Phase 5 交付

---

## Phase 4 Review（审查环节）

**触发时机**: 本 Phase 全部 Task 完成、验证通过后执行（标准流程第⑤步）。本 Phase 为核心功能，建议对 Task 4.8（import_service 组合链路）单独追加一次 review。

**审查方式**: 调用 `review` skill（`.claude/skills/review/SKILL.md`），审查范围为 Phase 4 的全部变更。

**审查重点**:

- [ ] `import_service` 组合链路完整（解析→去重→复制→写库→可选删除源文件），事务 + 失败回滚
- [ ] 文件复制失败 / 磁盘空间不足 / 文件被占用的错误处理与中文提示
- [ ] 重名文件处理策略正确
- [ ] 回收站操作失败时降级路径可用
- [ ] 重复导入检测（同一路径再次导入有提示）
- [ ] 路径安全：目录遍历防护、Unicode 路径处理
- [ ] 解析器容错：损坏的 .zip/.7z 不 panic，预览阶段不崩溃
- [ ] 拖拽导入与按钮选择两条入口流程一致

**输出**: 审查报告（结论 + 问题清单 + 验收核对表）。S1 阻断问题未清零前，不允许 git commit / 交付下一 Phase。
