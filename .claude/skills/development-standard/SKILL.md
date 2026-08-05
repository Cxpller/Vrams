---
name: development-standard
description: Full-stack development standards for VRAMS — Vue, Tauri, Python, SQLite, Git, and thesis.
---

# VRAMS 开发规范

## 目录

1. [Vue 规范](#1-vue-规范)
2. [Tauri 规范](#2-tauri-规范)
3. [Python 规范](#3-python-规范)
4. [SQLite 规范](#4-sqlite-规范)
5. [Git 规范](#5-git-规范)
6. [论文规范](#6-论文规范)
7. [通用原则](#7-通用原则)

---

## 1. Vue 规范

### 1.1 技术选型

| 项目 | 选型 | 说明 |
|------|------|------|
| 框架 | Vue 3 + Composition API | 必须使用 `<script setup lang="ts">` |
| 类型系统 | TypeScript (strict mode) | 禁用 `any`，除非有充分理由 |
| 状态管理 | Pinia | Options Store 模式，配合 Composition API |
| UI 库 | Element Plus | 全局按需引入 |
| 构建 | Vite 6 | 固定端口 1420 |
| 路由 | Vue Router 4 | History 模式 |
| 包管理 | pnpm | 禁止使用 npm/yarn |

### 1.2 目录结构

```
src/
├── api/              # API 调用封装（Tauri invoke）
├── assets/           # 静态资源（图片、字体等）
├── components/       # 通用组件（原子/分子组件）
│   ├── common/       # 全局通用组件
│   └── business/     # 业务组件
├── composables/      # 组合式函数（useXxx）
├── layouts/          # 布局组件
├── router/           # 路由配置
├── stores/           # Pinia 状态仓库
├── styles/           # 全局样式变量 & mixins
├── types/            # TypeScript 类型定义
├── utils/            # 工具函数
├── views/            # 页面级组件（路由页面）
├── App.vue           # 根组件
└── main.ts           # 入口文件
```

### 1.3 组件规范

- **命名**: 多单词 PascalCase（`UserProfile.vue`），目录 kebab-case
- **Props**: 使用 `withDefaults` + 接口定义，必须指定类型和默认值
- **Emits**: 使用 `defineEmits`，事件名 kebab-case
- **模板**: 不要超过 200 行，超过则拆分子组件
- **逻辑复用**: 抽离为 `composables/useXxx.ts`
- **样式**: scoped 样式，使用 CSS 变量主题化
- **强制指令顺序**: `v-if` → `v-for` → `v-model` → `@event` → `:prop`

### 1.4 代码示例

```typescript
// ✅ 正确: 标准组件模板
<script setup lang="ts">
import { ref, computed } from "vue";
import { useUserStore } from "@/stores/user";

interface Props {
  userId: string;
  readonly?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  readonly: false,
});

const emit = defineEmits<{
  "update": [id: string];
  "close": [];
}>();

const userStore = useUserStore();
const loading = ref(false);
const userData = computed(() => userStore.getUser(props.userId));
</script>
```

```typescript
// ❌ 禁止: Options API、any 类型、直接修改 props
// ❌ export default { data() { ... } }
```

### 1.5 性能规范

- 大列表使用 `v-memo` 或虚拟滚动
- 避免模板中复杂计算，使用 `computed`
- `v-for` 必须配 `:key`
- 无需响应式的数据用 `shallowRef` / `markRaw`

---

## 2. Tauri 规范

### 2.1 架构原则

```
Vue frontend → invoke() → Rust Command → 业务逻辑 / 数据库 / 文件系统
                ↑                              ↓
           IPC (JSON 序列化)              serde Serialize
```

- **前端不直接访问文件系统或数据库** — 一律经 Tauri Command 中转
- **Command 职责单一** — 一个命令只做一件事
- **错误处理** — Rust 端返回 `Result<T, E>`，前端统一处理错误

### 2.2 Rust 后端结构

```
src-tauri/src/
├── main.rs              # 入口，调用 vrams_lib::run()
├── lib.rs               # Tauri Builder，注册命令和插件
├── commands/            # #[tauri::command] 函数模块
│   ├── mod.rs
│   ├── greet.rs
│   └── ...
├── models/              # 数据模型 (struct + serde)
│   ├── mod.rs
│   └── ...
├── db/                  # 数据库访问层
│   ├── mod.rs
│   └── ...
├── services/            # 业务逻辑层
│   ├── mod.rs
│   └── ...
└── utils/               # 工具函数
    ├── mod.rs
    └── ...
```

### 2.3 Command 规范

```rust
// ✅ 正确: 标准 Command 模式
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[tauri::command]
pub fn get_user(id: i64, state: tauri::State<'_, AppState>) -> Result<UserInfo, String> {
    let user = state.db.get_user(id).map_err(|e| e.to_string())?;
    Ok(user)
}
```

### 2.4 注册流程

每次新增 Command 必须三步：

1. 在 `commands/` 模块中定义 `#[tauri::command] fn`
2. 在 `lib.rs` 中 `mod commands` + `tauri::generate_handler![...]`
3. 前端通过 `invoke("command_name", { args })` 调用

### 2.5 状态管理

- 使用 `tauri::State<T>` 管理应用级状态（数据库连接、配置等）
- 使用 `tauri::async_runtime::Mutex` 包装可变状态

```rust
pub struct AppState {
    pub db: Mutex<Database>,
    pub config: Config,
}
```

### 2.6 安全规范

- **禁止** `csp: null` — 必须配置合理的 CSP 策略
- 最小权限原则 — `capabilities/` 中只开放必需的权限
- 前端传参必须校验（长度、类型、边界值）
- 敏感操作需二次确认（删除/覆盖/导出等）

### 2.7 事件系统

- 用 `app_handle.emit()` 推送后端事件给前端
- 前端用 `listen()` 订阅，用完后 `unlisten()`

---

## 3. Python 规范

### 3.1 技术选型

| 项目 | 选型 | 说明 |
|------|------|------|
| 版本 | Python 3.11+ | 强制类型注解 |
| 包管理 | uv / pip | 使用 `pyproject.toml` 声明依赖 |
| 格式化 | Ruff | 行宽 100，单引号 |
| 类型检查 | mypy / pyright | strict 模式 |
| 测试 | pytest | 覆盖率 >= 80% |
| 异步 | asyncio + aiofiles | 文件/网络 IO 必须异步 |

### 3.2 Python 在 VRAMS 中的定位

Python 在 VRAMS 中用于**数据处理和解析**场景（非桌面主程序）：

- 论文文档解析（PDF/DOCX 提取）
- 数据分析与统计
- 原型验证与算法测试
- CLI 工具脚本
- 与 Rust 后端通过 subprocess 或文件协议通信

### 3.3 目录结构

```
scripts/                 # Python 脚本目录
├── pyproject.toml       # 项目配置与依赖
├── src/
│   ├── __init__.py
│   ├── parsers/         # 解析器
│   │   ├── __init__.py
│   │   ├── pdf_parser.py
│   │   └── docx_parser.py
│   ├── analysis/        # 数据分析
│   │   ├── __init__.py
│   │   └── ...
│   └── utils/           # 工具函数
│       ├── __init__.py
│       └── ...
├── tests/               # 测试
│   ├── __init__.py
│   ├── test_parsers/
│   └── ...
└── README.md
```

### 3.4 代码规范

```python
# ✅ 正确: 标准 Python 模块
"""Document parser module for VRAMS.

This module handles extracting text and metadata from academic papers.
"""

from __future__ import annotations

from pathlib import Path
from typing import Self


class PDFParser:
    """Parse PDF documents for VRAMS thesis management."""

    def __init__(self, file_path: Path) -> None:
        self._path = file_path
        self._metadata: dict[str, str] = {}

    @classmethod
    def from_path(cls, path: str | Path) -> Self:
        return cls(Path(path))

    def extract_text(self) -> str:
        """Extract all text content from the PDF."""
        ...

    @property
    def metadata(self) -> dict[str, str]:
        return self._metadata.copy()
```

```python
# ❌ 禁止:
# - 混用 str/Path 不做类型标注
# - except: 裸捕获
# - 可变默认参数（def foo(x=[])）
# - 全局可变状态
```

### 3.5 错误处理

- 自定义异常继承自 `VRAMSError(Exception)`
- 函数应明确声明可能抛出的异常类型
- 使用 `try/except/finally` 确保资源释放

### 3.6 类型注解规范

- 所有函数参数和返回值必须标注类型
- 使用 `TypeVar` / `Generic` 处理泛型
- 复杂类型使用 `type alias`

---

## 4. SQLite 规范

### 4.1 使用约束

- SQLite 仅供**本地存储**，不由 Rust/Tauri 直连管理
- 操作均通过 Rust 后端的 `rusqlite` 或 `sqlx` 执行
- 禁止前端直接访问 `.db` 文件

### 4.2 数据库设计规范

```sql
-- ✅ 正确: 标准建表语句
CREATE TABLE IF NOT EXISTS papers (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT    NOT NULL,
    author      TEXT    NOT NULL DEFAULT 'Unknown',
    abstract    TEXT,
    file_path   TEXT    NOT NULL UNIQUE,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE INDEX idx_papers_title ON papers(title);
```

### 4.3 命名规范

| 项目 | 规范 | 示例 |
|------|------|------|
| 表名 | 复数 snake_case | `papers`, `tags`, `paper_tags` |
| 列名 | snake_case | `file_path`, `created_at` |
| 主键 | `id` | `INTEGER PRIMARY KEY AUTOINCREMENT` |
| 外键 | 引用表名 + `_id` | `paper_id`, `tag_id` |
| 索引 | `idx_表名_列名` | `idx_papers_title` |
| 联合索引 | `idx_表名_列1_列2` | `idx_paper_tags_paper_tag` |

### 4.4 迁移规范

- 使用版本化迁移脚本：`migrations/V001__init.sql`
- 使用 `PRAGMA user_version` 记录 schema 版本
- 迁移脚本必须**幂等**（`IF NOT EXISTS` / `IF EXISTS`）

```sql
-- migrations/V001__init.sql
PRAGMA user_version = 1;

CREATE TABLE IF NOT EXISTS papers (...);
CREATE TABLE IF NOT EXISTS tags (...);
```

### 4.5 查询规范

- 使用**参数化查询**（`?` 或 `:named`），禁止字符串拼接
- 批量插入使用事务包裹
- 大数据量查询使用分页：`LIMIT ? OFFSET ?`

```rust
// ✅ 正确: 参数化查询
sqlx::query!("SELECT * FROM papers WHERE id = ?", paper_id)
    .fetch_optional(&pool)
    .await?;

// ❌ 禁止: 字符串拼接
// format!("SELECT * FROM papers WHERE title = '{}'", title)
```

### 4.6 性能规范

- 为查询频繁的列建立索引
- `EXPLAIN QUERY PLAN` 检查查询计划
- 批量写入使用事务
- 定期 `PRAGMA optimize`
- 避免在 WHERE 中包裹列：`WHERE YEAR(created_at) = 2024` → `WHERE created_at >= '2024-01-01' AND created_at < '2025-01-01'`

---

## 5. Git 规范

### 5.1 分支策略

```
main           ← 生产就绪，只允许 merge 进入
  └─ develop   ← 开发主分支
       ├─ feat/xxx    ← 功能分支
       ├─ fix/xxx     ← 修复分支
       ├─ refactor/   ← 重构分支
       └─ chore/xxx   ← 杂项（配置、依赖等）
```

| 分支类型 | 前缀 | 合并目标 | 说明 |
|----------|------|----------|------|
| 功能 | `feat/` | `develop` | 新功能开发 |
| 修复 | `fix/` | `develop` | Bug 修复 |
| 重构 | `refactor/` | `develop` | 代码重构，不改功能 |
| 文档 | `docs/` | `develop` | 文档变更 |
| 杂项 | `chore/` | `develop` | 构建/配置/依赖变更 |
| 热修复 | `hotfix/` | `main` + `develop` | 生产紧急修复 |

### 5.2 Commit Message 规范

使用 Conventional Commits 格式：

```
<type>(<scope>): <简短描述>

<详细说明（可选）>

<关联 issue / breaking change（可选）>
```

**类型 (type)**:
| 类型 | 用途 |
|------|------|
| `feat` | 新功能 |
| `fix` | Bug 修复 |
| `refactor` | 重构（不修 bug 不加功能） |
| `docs` | 文档变更 |
| `style` | 代码格式（不影响逻辑） |
| `perf` | 性能优化 |
| `test` | 测试相关 |
| `chore` | 构建/工具/依赖 |
| `db` | 数据库迁移变更 |

**范围 (scope)**:
- `vue` — 前端变更
- `tauri` — Rust 后端变更
- `py` — Python 脚本变更
- `db` — 数据库变更
- `conf` — 配置文件
- `docs` — 文档/论文

**示例**:
```
feat(vue): add paper list search with fuzzy matching

- integrate Fuse.js for fuzzy search
- add debounced search input component
- handle empty state and loading state

Closes #42
```

```
fix(tauri): handle file path with Unicode characters on Windows
```

```
db: add papers table migration

V001__create_papers_table
```

### 5.3 Commit 原子性原则

- **一个 commit 只做一件事**
- 每个 commit 必须能通过构建（`pnpm build`）
- 不要混合格式化改动和逻辑改动

### 5.4 Pull Request 规范

- PR 标题遵循 Conventional Commits 格式
- PR 描述包含：**做了什么 / 如何实现 / 测试方式 / 截图（如涉及 UI）**
- 合并方式：`Squash and Merge`（功能分支）或 `Rebase and Merge`（多人协作分支）
- PR 至少 1 人 Review 后才可合并

### 5.5 常用命令流程

```bash
# 开始新功能
git checkout develop
git pull
git checkout -b feat/my-feature

# 提交
git add <files>
git commit -m "feat(vue): add new feature"

# 同步 develop
git fetch origin
git rebase origin/develop

# 提交 PR → Review → Squash Merge
```

### 5.6 禁止事项

- ❌ 直接向 `main` 推送（hotfix 除外且必须 PR）
- ❌ 在 `main` 或 `develop` 上直接修改
- ❌ `--force` 推送（除非有充分理由且已通知团队）
- ❌ 提交包含 `node_modules/`、`target/`、`dist/`、`.env` 文件
- ❌ 将大文件（>10MB）提交到仓库

---

## 6. 论文规范

### 6.1 论文总体结构

```
第1章 绪论
  1.1 研究背景与意义
  1.2 国内外研究现状
  1.3 论文主要工作
  1.4 论文组织结构

第2章 相关技术介绍
  2.1 Tauri 桌面应用框架
  2.2 Vue 3 前端框架
  2.3 数据存储技术
  2.4 文献管理相关技术

第3章 系统需求分析
  3.1 可行性分析
  3.2 功能需求分析
  3.3 非功能需求分析
  3.4 系统用例分析

第4章 系统设计
  4.1 系统架构设计
  4.2 功能模块设计
  4.3 数据库设计
  4.4 前后端接口设计

第5章 系统实现
  5.1 开发环境配置
  5.2 核心功能实现
  5.3 关键技术难点与解决方案

第6章 系统测试
  6.1 测试环境与方法
  6.2 功能测试
  6.3 性能测试
  6.4 测试结果分析

第7章 总结与展望
  7.1 工作总结
  7.2 不足与展望

参考文献

致谢

附录
```

### 6.2 写作规范

#### 语言风格
- **学术汉语**：正式、客观、严谨
- 使用第三人称（"本文"、"该系统"），避免第一人称（"我"、"我们"）
- 术语一致：首次出现时标注英文全称和缩写
- **避免 AI 生成腔调**（"首先...其次...再次"、"总的来说"、"值得一提的是"）

#### 技术描述
- 不贴完整代码，用**伪代码**或**核心代码片段**
- 架构图、流程图、ER 图等必须统一风格（推荐 Draw.io / Excalidraw）
- 数据显示用**表格**，趋势用**图表**
- 每个图/表必须有编号和标题（"图 4-1 系统架构图"）

#### 参考文献
- 推荐使用 Zotero 管理
- 引用格式统一（GB/T 7714）
- 文献数量：15–25 篇，含 5 篇以上外文
- 近 3–5 年文献占比不低于 50%

### 6.3 与代码的关联

- **论文中的架构图必须与代码一致** — 重构后同步更新论文
- 功能实现章节引用具体模块路径：`src/views/PaperList.vue`
- 数据库设计章节展示核心表的 DDL
- 测试数据从项目实际运行结果导出

### 6.4 时间规划

| 阶段 | 内容 | 预计时间 |
|------|------|----------|
| 开题 | 选题、文献调研、开题报告 | 第 1–4 周 |
| 需求 | 需求分析、用例建模 | 第 5–6 周 |
| 设计 | 架构设计、数据库设计、接口设计 | 第 7–9 周 |
| 实现 | 编码、单元测试 | 第 10–13 周 |
| 测试 | 集成测试、系统测试、性能优化 | 第 14–15 周 |
| 论文 | 论文撰写、修改、查重、答辩 | 第 14–18 周 |

### 6.5 查重与格式

- 查重率要求：按学校规定（通常 ≤ 20% 或 ≤ 30%）
- 格式按学校模板严格排版
- 避免大段直接引用（改写 + 标注出处）
- 所有代码片段需转为**伪代码**或**流程图**描述

---

## 7. 通用原则

### 7.1 代码质量

| 维度 | 原则 |
|------|------|
| 可读性 | 代码即文档，命名自解释 |
| 可维护性 | 单一职责，模块高内聚低耦合 |
| 可测试性 | 纯函数优先，副作用隔离 |
| 可追溯性 | 每个功能对应 issue/commit |

### 7.2 错误处理

```
前端层     → try/catch invoke() + 用户友好的 Toast 提示
Tauri 层   → Result<T, String> + 详细错误信息
业务层     → 自定义错误类型 + 日志记录
数据库层   → 连接池 + 重试 + 事务回滚
```

### 7.3 日志规范

- Rust 后端：`tracing` / `log` crate，结构化日志
- Vue 前端：`console.warn/error` 用于开发，生产环境禁用 `console.log`
- Python 脚本：`logging` 模块，输出到 stderr

### 7.4 安全规范

- 用户输入必须校验（长度、类型、白名单）
- 文件路径防止目录遍历攻击
- CSP 必须配置，禁止 `'unsafe-inline'`（除非必要）
- Tauri 权限遵循最小原则

### 7.5 文档规范

- 每个 Rust 公开函数必须有 doc comment（`///`）
- 每个 Vue 组件必须有功能说明注释
- API 变更必须同步更新 README 或相关文档
- 数据库 Schema 变更必须编写迁移脚本

---

<!-- 本规范由 development-standard skill 管理，用于约束 VRAMS 全栈开发的代码质量、流程与一致性 -->
