# Phase 7: 预览图与来源链接

**所属**: VRAMS MVP 开发计划
**预估工时**: 1 天
**前置依赖**: Phase 6（资产详情页可用）
**产出**: 用户可手动上传封面图；Booth URL 可填写和一键打开

---

## Task 7.1 — 实现上传预览图命令

### 任务目标

接收用户选择的图片文件，复制到 `thumbnails/{id}.jpg`，更新数据库。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/asset_commands.rs` | 资产命令 |

### 修改内容

```rust
#[tauri::command]
fn upload_thumbnail(state: State<AppState>, library_root: State<LibraryRoot>, asset_id: i64, source_path: String) -> Result<(), String>
```

逻辑：
1. 校验 source_path 图片格式（png/jpg/jpeg/webp）
2. 读取图片文件（限制大小 ≤ 10MB）
3. 生成缩略图（可选：resize 到 512×512 以减少磁盘占用）
4. 保存到 `{library}/thumbnails/{asset_id}.jpg`
5. 更新 `assets.thumbnail = "thumbnails/{asset_id}.jpg"`

### 前置依赖

- Phase 2（asset_repo 可用）

### 验收标准

- [ ] 上传后文件出现在 thumbnails/ 目录
- [ ] 数据库 thumbnail 字段正确更新
- [ ] 非图片格式返回错误
- [ ] 图片 > 10MB 返回错误
- [ ] 已有封面时覆盖旧文件

---

## Task 7.2 — 实现 ThumbnailUploader 组件

### 任务目标

在资产详情页提供上传封面的交互组件。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/components/business/ThumbnailUploader.vue` | 封面上传组件 |

### 修改内容

UI：
- 点击封面区域 → 打开系统文件选择器（图片格式过滤）
- 拖拽图片到封面区域 → 上传
- 上传中显示加载进度
- 上传成功后封面即时刷新

使用 `tauri-plugin-dialog` 的文件选择器。

### 前置依赖

- Task 7.1
- Phase 1 Task 1.7（dialog 权限）

### 验收标准

- [ ] 点击封面 → 弹出图片选择器
- [ ] 选中图片后自动上传并刷新封面
- [ ] 上传失败时显示错误提示
- [ ] 无封面时显示"点击上传封面"提示

---

## Task 7.3 — 卡片封面渲染

### 任务目标

AssetCard 根据 `thumbnail` 字段加载预览图，有图显示图，无图显示占位。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/components/common/AssetCard.vue` | 资产卡片 |

### 修改内容

```typescript
// 图片加载逻辑:
// 1. asset.thumbnail 有值 → 使用 convertFileSrc() 转换路径为前端可访问的 URL
// 2. 加载中 → 显示 Element Plus ElSkeleton
// 3. 加载失败 → 显示类型图标占位
// 4. 无 thumbnail → 显示默认灰色骨架 + 文件类型图标
```

使用 `@tauri-apps/api/core` 的 `convertFileSrc` 将本地路径转为 WebView 可访问的 URL。

### 前置依赖

- Task 7.1（至少有一个测试上传数据可用）

### 验收标准

- [ ] 有封面的卡片正确显示图片
- [ ] 无封面的卡片显示灰色骨架 + 类型图标
- [ ] 图片加载失败时降级为骨架
- [ ] 图片不拉伸变形（object-fit: cover）

---

## Task 7.4 — 实现一键打开 URL

### 任务目标

点击 Booth URL 时在系统默认浏览器中打开。

### 涉及文件

使用已有的 `tauri-plugin-opener`。

### 修改内容

在 AssetDetail.vue 中添加"在浏览器打开"按钮/链接：

```typescript
import { openUrl } from "@tauri-apps/plugin-opener";

// 点击 URL → openUrl(asset.source_url)
```

### 前置依赖

- 无（该插件已在 Phase 1 安装）

### 验收标准

- [ ] 点击 URL → 系统默认浏览器打开 Booth 商品页
- [ ] URL 为空时按钮隐藏或置灰
- [ ] URL 无效时不崩溃

---

## Phase 7 完整验收 Checklist

- [ ] 7.1 upload_thumbnail 命令实现
- [ ] 7.2 ThumbnailUploader 组件（选择/拖拽上传）
- [ ] 7.3 卡片封面渲染（有图/无图/加载失败）
- [ ] 7.4 一键打开 Booth URL
- [ ] 全流程：详情页上传封面 → 卡片刷新显示 → 点击 URL 跳转浏览器

---

## Phase 7 Review（审查环节）

**触发时机**: 本 Phase 全部 Task 完成、验证通过后执行（标准流程第⑤步）。

**审查方式**: 调用 `review` skill（`.claude/skills/review/SKILL.md`），审查范围为 Phase 7 的全部变更。

**审查重点**:

- [ ] `upload_thumbnail` 文件复制、`{id}.jpg` 命名、图片类型限制（PNG/JPG）
- [ ] 应用重启后预览图路径仍然有效
- [ ] URL 打开前校验 scheme（仅 http/https），空 URL 隐藏或置灰
- [ ] 封面加载骨架态 + 加载失败回退类型图标

**输出**: 审查报告（结论 + 问题清单 + 验收核对表）。S1 阻断问题未清零前，不允许 git commit / 交付下一 Phase。
