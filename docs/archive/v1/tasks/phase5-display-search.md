# Phase 5: 资产展示与搜索

**所属**: VRAMS MVP 开发计划
**预估工时**: 2 天
**前置依赖**: Phase 4（资产导入完成，库中有数据可展示）
**产出**: 首页卡片列表 + 空状态引导 + 搜索功能

---

## Task 5.1 — 实现 get_assets 命令

### 任务目标

实现分页查询资产列表，返回卡片展示所需的基本信息。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/asset_commands.rs` | 资产命令 |

### 修改内容

```rust
#[tauri::command]
fn get_assets(state: State<AppState>, page: i64, page_size: i64) -> Result<AssetListResponse, String>

// AssetListResponse:
//   items: Vec<AssetListItem>
//   total: i64

// AssetListItem (与 assets 表查询 JOIN asset_files 聚合):
//   id, name, thumbnail, created_at,
//   file_count: i64,           // COUNT(asset_files)
//   primary_type: Option<String>  // 主文件类型（用于卡片图标）
```

查询逻辑：`SELECT a.*, COUNT(af.id) AS file_count, MAX(CASE WHEN af.is_primary=1 THEN af.file_type END) AS primary_type FROM assets a LEFT JOIN asset_files af ON af.asset_id = a.id GROUP BY a.id ORDER BY a.created_at DESC LIMIT ? OFFSET ?`

### 前置依赖

- Phase 2 Task 2.4（asset_repo）

### 验收标准

- [ ] 返回正确的分页结果 + 总数
- [ ] `page_size = 50` 时返回最多 50 条
- [ ] 按创建时间倒序
- [ ] `file_count` 正确统计关联文件数

---

## Task 5.2 — 实现 search_assets 命令

### 任务目标

按关键词搜索资产名称和备注。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/asset_commands.rs` | 资产命令 |

### 修改内容

```rust
#[tauri::command]
fn search_assets(state: State<AppState>, keyword: String, page: i64, page_size: i64) -> Result<AssetListResponse, String>

// 查询: WHERE a.name LIKE '%keyword%' OR a.notes LIKE '%keyword%'
```

### 前置依赖

- Task 5.1（复用 AssetListItem 和分页逻辑）

### 验收标准

- [ ] 搜索"kikyo"匹配"Avatar-Kikyo 套装"
- [ ] 搜索不区分大小写
- [ ] 无匹配时返回空列表而非错误
- [ ] 关键词为空时等同于 get_assets

---

## Task 5.3 — 创建前端 API 层

### 任务目标

封装 asset 相关的 invoke 调用。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/api/assets.ts` | 资产 API 封装 |

### 修改内容

```typescript
export interface AssetListItem {
  id: number;
  name: string;
  thumbnail: string | null;
  created_at: string;
  file_count: number;
  primary_type: string | null;
}

export interface AssetListResponse {
  items: AssetListItem[];
  total: number;
}

export async function getAssets(page: number, pageSize: number): Promise<AssetListResponse>;
export async function searchAssets(keyword: string, page: number, pageSize: number): Promise<AssetListResponse>;
```

### 前置依赖

- Task 5.1, 5.2

### 验收标准

- [ ] 调用 `getAssets` 返回正确的响应结构
- [ ] 调用 `searchAssets` 返回搜索结果

---

## Task 5.4 — 创建 Pinia Store

### 任务目标

管理资产列表状态，包括加载、分页、刷新。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/stores/assetStore.ts` | 资产状态仓库 |

### 修改内容

```typescript
export const useAssetStore = defineStore("assets", () => {
  const items = ref<AssetListItem[]>([]);
  const total = ref(0);
  const loading = ref(false);
  const currentPage = ref(1);
  const keyword = ref("");
  const PAGE_SIZE = 50;

  async function fetchAssets();         // 首页加载
  async function loadMore();            // 滚动加载下一页
  async function search(keyword: string); // 搜索
  async function refresh();             // 重新加载第一页

  return { items, total, loading, currentPage, keyword, fetchAssets, loadMore, search, refresh };
});
```

### 前置依赖

- Task 5.3

### 验收标准

- [ ] `fetchAssets` 加载第一页
- [ ] `loadMore` 追加下一页到 `items`
- [ ] `search` 重置列表为搜索结果
- [ ] `refresh` 重置并重新加载

---

## Task 5.5 — 完善 AssetCard 组件

### 任务目标

实现资产卡片组件，展示预览图、名称、标签、多文件角标。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/components/common/AssetCard.vue` | 资产卡片 |

### 修改内容

Props: `asset: AssetListItem`

UI：
- 卡片顶部：预览图区域（有图显示图，无图显示灰色骨架 + 类型图标）
- 卡片标题：资产名称
- 卡片底部：标签（最多显示 3 个，多出显示 "+N"）
- 多文件角标：`file_count > 1` 时右下角显示 "2文件 📎"

交互：
- 点击卡片 → `router.push({ name: "asset-detail", params: { id } })`
- 右键菜单（Phase 6 实现快捷操作）

### 前置依赖

- Task 5.4（store 提供数据）

### 验收标准

- [ ] 卡片正确展示资产名称
- [ ] 有预览图时显示图片，无预览图时显示骨架占位
- [ ] 多文件时显示角标
- [ ] 点击卡片跳转到详情页（路由准备，Phase 6 实现页面）

---

## Task 5.6 — 实现首页

### 任务目标

实现资产列表首页，网格布局、分页加载。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/views/HomePage.vue` | 首页 |
| `src/components/common/EmptyState.vue` | 空状态引导组件 |

### 修改内容

**`HomePage.vue`**:
- 页面布局：顶部搜索栏 + 标签筛选器（Phase 6 实现筛选）+ 卡片网格
- 卡片网格：使用 Element Plus `ElRow` + `ElCol` 或 CSS Grid，自适应列数
- 滚动加载：监听滚动到底部 → 调用 `assetStore.loadMore()`
- 加载中：骨架屏（Element Plus `ElSkeleton`）
- 无数据：显示 EmptyState

**`EmptyState.vue`**:
- 欢迎文字 + 导入按钮
- Props: `title`, `description`, `actionText`

### 前置依赖

- Task 5.4
- Task 5.5

### 验收标准

- [ ] 首页以卡片网格展示资产
- [ ] 滚动到底部自动加载更多
- [ ] 无资产时显示空状态引导页
- [ ] 空状态引导页中有"导入资产"按钮

---

## Task 5.7 — 实现搜索组件

### 任务目标

实现搜索输入框，支持防抖实时搜索。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/components/business/SearchBar.vue` | 搜索栏 |

### 修改内容

```typescript
// 使用 @vueuse/core 的 useDebounceFn
// input 事件 → 防抖 300ms → 调用 assetStore.search(keyword)
// 搜索框带清空按钮（×）
// 搜索中显示加载图标
// 搜索结果为空时显示"没有找到匹配的资产"
```

### 前置依赖

- Phase 1 Task 1.4（@vueuse/core 已安装）
- Task 5.4（assetStore.search）

### 验收标准

- [ ] 输入后 300ms 触发搜索
- [ ] 清空搜索框后恢复为全部资产
- [ ] 搜索无结果时显示空状态文字
- [ ] 搜索过程中不阻塞 UI

---

## Phase 5 完整验收 Checklist

- [ ] 5.1 get_assets 命令实现（分页 + 聚合查询）
- [ ] 5.2 search_assets 命令实现（LIKE 搜索）
- [ ] 5.3 前端 API 层封装完成
- [ ] 5.4 Pinia store（加载/搜索/分页/刷新）
- [ ] 5.5 AssetCard 组件（封面/骨架/角标/跳转）
- [ ] 5.6 首页（网格/无限滚动/空状态）
- [ ] 5.7 SearchBar（防抖/清空/空结果提示）
- [ ] 全流程：导入 → 首页展示 → 搜索 → 加载更多

---

## Phase 5 Review（审查环节）

**触发时机**: 本 Phase 全部 Task 完成、验证通过后执行（标准流程第⑤步）。

**审查方式**: 调用 `review` skill（`.claude/skills/review/SKILL.md`），审查范围为 Phase 5 的全部变更。

**审查重点**:

- [ ] 分页参数（`LIMIT/OFFSET`）正确，总数统计准确
- [ ] LIKE 搜索特殊字符（`%` `_`）转义处理
- [ ] 防抖 300ms 生效，清空搜索框恢复全部资产
- [ ] loading / empty / error 三态齐全
- [ ] 列表性能：`v-memo` / 虚拟滚动 / `shallowRef` 等按需使用
- [ ] AssetCard 字段完整（预览图、名称、标签、多文件角标、跳转）

**输出**: 审查报告（结论 + 问题清单 + 验收核对表）。S1 阻断问题未清零前，不允许 git commit / 交付下一 Phase。
