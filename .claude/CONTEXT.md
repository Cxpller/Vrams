# VRAMS — 项目上下文

**最后更新**: 2026-11-10

> ## ⚠️ v2 重写中（2026-11）
>
> - 范围、决策、数据模型以 [`docs/v2-需求与框架.md`](../docs/v2-需求与框架.md) 为准；下方 §1–§7 为 **v1 历史记录**，仅供参考。
> - 代码在 `v2` 分支：前端 Vue → **React 19 + antd + Zustand**；后端保留 Tauri + Rust + SQLite，只留 `db`（新迁移机制）与 `utils::path`。
> - v1 已存档为标签 `v1-final`，旧文档移至 `docs/archive/v1/`。
> - 本机环境（2026-11-10 装好）：Rust 1.99 在 `E:\DevTools`、VS 2022 Build Tools（仅 C++）、pnpm 12。
> - 下一步：第一版 —— V001 迁移（library_roots / assets / asset_files / archive_entries）→ 目录扫描 → React 列表页。

---

## 1. 项目状态

| 项目 | 内容 |
|------|------|
| **项目名称** | VRAMS (Virtual Reality Asset Management System) |
| **定位** | 面向 VRChat 用户及 Unity 创作者的本地化虚拟资产管理工具 |
| **技术栈** | Tauri v2 + Vue 3 + TypeScript + Rust + SQLite |
| **开发阶段** | **Phase 3 已完成**（2026-08 会话），可交付 Phase 4 |
| **用户模型** | 单机单用户，无登录/多用户 |
| **支持格式** | .unitypackage / .zip / .7z |
| **存储策略** | 复制到库目录 `assets/{id}/`，保留原始文件名，导入后可删除源文件 |
| **包体解析** | 只读不解压，不缓存 |

### 1.1 当前开发进度

**已完成（Phase 1，2026-08 会话）**

**已完成**

- [x] 仓库推送 GitHub：`main` = `ecab94f`（含 mentor skill + 教学模式登记）
- [x] **Task 1.1 Rust 依赖**：Cargo.toml 依赖全部就位，`cargo check` 通过（EXIT 0 无警告）
- [x] **Task 1.2 后端模块目录**：commands/models/services/db/parsers/download/utils 7 域 + db/repository，共 26 个文件（mod.rs 声明 + 占位）
- [x] **Task 1.3 清理脚手架**：greet 前后端清除，App.vue 仅留 `<router-view />`，Element Plus **按需引入**（用户决策，unplugin-vue-components + unplugin-auto-import，CSS 产物减半）
- [x] **Task 1.4 前端依赖**：element-plus 2.14.3、icons-vue、vue-router **4.6.4**、pinia 4.0.2、@vueuse/core 14.4.0（vue-router 曾误装 5.x 与 vite 6 冲突，已降级）
- [x] **Task 1.5 前端目录**：api/components(common+business)/composables/stores/views/router/types/styles/utils 就位，types 占位接口、空路由、CSS 变量
- [x] **Task 1.6 路径别名**：vite `resolve.alias` + tsconfig `baseUrl/paths` 双端一致，`@/` 可用
- [x] **Task 1.7 Tauri 配置**：窗口 VRAMS 1200×800，CSP 从 null 收紧，capabilities 追加 `dialog:default`/`fs:default`/`core:window:default`，lib.rs 注册 dialog+fs 插件
- [x] **Task 1.8 全量构建**：`cargo check` + `pnpm build` + `pnpm tauri build --debug` 全绿，产出 MSI + NSIS 安装包，vrams.exe 启动冒烟通过

**关键事件**

- ✅ VS Build Tools 已装（vswhere 确认含 VC 工具链），link.exe 卡点解除
- ✅ WiX/NSIS 下载卡 GitHub 直连 → 走用户本地代理 `127.0.0.1:27888`（`HTTPS_PROXY` 环境变量）秒下
- ✅ 教学模式知识点已沉淀到 `.claude/notes/`（rust.md / tauri.md / agent.md）

**下一步**：Phase 4（资产导入核心），任务文件 `docs/tasks/phase4-import.md`

**已完成（Phase 2，2026-08 会话）**

- [x] **Task 2.1 V001__init.sql**：`src-tauri/migrations/V001__init.sql` 四表（assets/asset_files/tags/asset_tags）+ 6 索引 + `user_version=1`，幂等，外键 `ON DELETE CASCADE`
- [x] **Task 2.2 数据库初始化**：`db/mod.rs` `initialize_database()`（建目录→开库→外键→读 user_version→`include_str!` 执行迁移），2 测试
- [x] **Task 2.3 数据模型**：`models/asset.rs`（Asset/AssetListItem/Create/UpdateAssetRequest）、`asset_file.rs`（AssetFile）、`tag.rs`（Tag/Create/UpdateTagRequest），全部 Serialize/Deserialize，请求响应分离
- [x] **Task 2.4 Asset Repository**：`asset_repo.rs` CRUD + 分页 + LIKE 搜索 + 部分更新 + 级联删除，8 测试
- [x] **Task 2.5 AssetFile Repository**：`asset_file_repo.rs` 事务批量插入 + `NewAssetFile` 结构体 + set_primary_file 互斥，4 测试
- [x] **Task 2.6 Tag Repository**：`tag_repo.rs` tags CRUD + asset_tags 全量替换/双向查询，6 测试
- [x] **Task 2.7 AppState**：`lib.rs` `AppState { db: Mutex<Connection> }` + `.manage()` 注入；`utils/path.rs` 库路径（`VRAMS_LIBRARY_PATH` 环境变量优先）
- [x] **验证**：`cargo test` 22 passed + `cargo clippy` 零警告 + Tauri 冒烟（vrams.db 自动创建 57KB）
- [x] **Review**：无 S1 阻断；S2（unchecked_transaction 嵌套事务隐患，Phase 4 导入时统一事务边界）已登记遗留问题；S3（clippy 复杂类型）已修复

**环境变化（Phase 2 会话）**：用户删除了 VS 下载 temp，导致 vswhere 注册信息丢失 → cargo 找不到 cl.exe/link.exe。**修复方案**：跑 cargo 前先 `vcvarsall.bat x64` 加载 MSVC 环境（`cmd /c '"...\vcvarsall.bat" x64 && cargo ...'`），VS 本体完好无需重装。

**已完成（Phase 3，2026-08 会话）**

- [x] **Task 3.1 tag commands**：`commands/tag_commands.rs` 实现 get_tags/create_tag/update_tag/delete_tag，每个 command 单次 `state.db.lock()`；create/update 加 trim + 非空 + 32 字符校验；重名通过识别 UNIQUE 约束转「标签名已存在」友好错误（利用约束而非先查再插，避免竞态）
- [x] **Task 3.2 注册**：`lib.rs` `generate_handler!` 注册 4 命令，三步注册齐全（commands 模块 → handler → 前端 invoke）
- [x] **Task 3.3 前端 API**：`types/tag.ts` 类型与 `models/tag.rs` serde 字段逐字段对齐（返回字段不转换保持 snake_case）；`api/tags.ts` invoke 封装，参数 camelCase→snake_case 由 Tauri 自动转换
- [x] **Task 3.4 Pinia store**：`stores/tagStore.ts` setup 风格（ref=state/computed=getters/函数=actions），tagsByGroup 空分组归「未分组」、tagOptions 供选择器；`main.ts` 注册 createPinia
- [x] **Task 3.5 页面**：`views/TagManage.vue` 左分组右标签 + 新建/编辑/删除（ElMessageBox 二次确认）+ 空状态；`components/business/TagEditDialog.vue` 弹窗，groupOptions 仅真实分组
- [x] **Task 3.6 路由**：`/`（HomePage 占位）+ `/tags` 懒加载
- [x] **验证**：`cargo check` + `cargo test` 21 passed + `pnpm build`（vue-tsc）全绿；`tauri dev` 全栈冒烟通过（新建/编辑/删除/重名/空名/分组筛选）
- [x] **Review**：两次 review 均通过无 S1/S2。修复 3 项 S3 + 1 个冒烟问题（见下）

**关键坑（Phase 3 会话）**：Element Plus **按需引入**模式下，`ElMessage`/`ElMessageBox` 等函数式 API 的样式不会自动跟随——显式 `import { ElMessageBox } from "element-plus"` 只拿到 JS。症状：弹窗/toast 功能正常但无样式，内容堆在左上角。**修复**：`main.ts` 显式 `import "element-plus/es/components/message/style/css"` + `"element-plus/es/components/message-box/style/css"`。

---

## 2. 关键决策记录

| 决策 | 结论 | 理由 |
|------|------|------|
| 存储策略 | 复制到库目录，非仅索引 | 用户要清空 Downloads，统一管理 |
| 包体解析 | 只读不解压，不缓存 | 不占内存，不占磁盘 |
| 合并多文件 | 导入审查窗口勾选 → 合并选中 | 同一商品分卷场景 |
| 导入 Unity | OS 默认打开 / 提取 .unitypackage | 不需模拟拖拽或 Unity 插件 |
| 预览图 | Booth og:image 自动下载（默认开启）+ 手动兜底 | 用户便利最大化 |
| 多用户 | 无 | VRChat 场景无此需求 |
| Python sidecar | 不需要 | Rust 可独立完成所有工作 |
| 数据库删除 | 硬删除，文件 + 数据库同步清理 | 软删除无意义 |
| 文件命名 | 保留原始文件名 | 用户通过文件管理器查看时认识 |

---

## 3. 文档体系

### 3.1 顶层文档

```
docs/
├── 产品需求文档.md      # v3.0 — 产品需求定稿（用户故事/用例/优先级/路线）
├── 系统架构设计.md       # v2.0 — 技术架构定稿（架构图/存储/解析/API/目录结构）
├── 数据库设计.md         # v1.0 — 数据库设计定稿（ER 图/DDL/查询/迁移）
├── 开发计划.md           # v1.0 — MVP 开发计划（9 Phase / 12 天预估）
└── tasks/                # 各 Phase 详细任务
    ├── phase1-init.md
    ├── phase2-db.md
    ├── phase3-tags.md
    ├── phase4-import.md
    ├── phase5-display-search.md
    ├── phase6-management.md
    ├── phase7-thumbnail.md
    ├── phase8-firstlaunch.md
    └── phase9-integration.md
```

### 3.2 每个 Task 文档的结构

| 字段 | 说明 |
|------|------|
| 任务目标 | 做什么、为什么 |
| 涉及文件 | 精确到文件路径 |
| 修改内容 | 具体改动描述 |
| 前置依赖 | 需先完成的 Task/Phase |
| 验收标准 | checklist 形式，逐条可打勾 |
| 风险点 | 可能的问题 + 缓解措施 |

### 3.3 Skills

| Skill | 角色 |
|-------|------|
| `development-standard` | 全栈开发规范（Vue/Tauri/Python/SQLite/Git/论文） |
| `product-analysis` | 产品经理分析需求（工作流 + PRD 模板） |
| `architecture-design` | 架构设计工作流 |
| `project-analysis` | 项目分析 |
| `review` | 代码审查（独立 subagent，对照验收标准/开发规范/架构文档输出审查报告） |
| `mentor` | 教学模式：边做项目边讲 Agent/Rust/Tauri/SQLite 知识，默认开启可随时喊停（详见 `.claude/skills/mentor/SKILL.md`） |

---

## 4. 开发计划一览

**当前进度**: P1 ✅ P2 ✅ P3 ✅（21 测试全绿 + 全栈冒烟通过），下一步按 `phase4-import.md` 开工。

| Phase | 名称 | 工时 | 依赖 | 任务数 | 文件 |
|-------|------|------|------|--------|------|
| 1 | 项目初始化与环境搭建 | 1 天 | — | 8 | `phase1-init.md` |
| 2 | 数据库与数据模型 | 1 天 | P1 | 7 | `phase2-db.md` |
| 3 | 标签系统（全栈验证） | 1 天 | P2 | 6 | `phase3-tags.md` |
| 4 | 资产导入（核心） | 3 天 | P2+P3 | 13 | `phase4-import.md` |
| 5 | 资产展示与搜索 | 2 天 | P4 | 7 | `phase5-display-search.md` |
| 6 | 资产管理操作 | 1.5 天 | P5 | 9 | `phase6-management.md` |
| 7 | 预览图与来源链接 | 1 天 | P6 | 4 | `phase7-thumbnail.md` |
| 8 | 首次启动体验 | 0.5 天 | P5 | 4 | `phase8-firstlaunch.md` |
| 9 | 集成验收与边界处理 | 1 天 | P1~P8 | 5 | `phase9-integration.md` |
| **合计** | | **12 天** | | **63** | |

### 依赖关系

```
P1 → P2 → P3 ────────────────────────
                  │                   │
                  ├── P4 ─→ P5 ─→ P6 ─┤── P9
                  │           │       │
                  │           ├── P7 ─┘
                  │           │
                  │           └── P8
                  │
```

---

## 5. 当前项目文件结构（现状）

```
vrams/
├── .claude/
│   ├── CONTEXT.md              # 本文件
│   ├── notes/                  # 教学模式知识点沉淀（rust.md/tauri.md/agent.md）
│   └── skills/
├── docs/                       # 需求/架构/数据库/计划/任务
├── src/                        # Vue 前端（Phase 3 标签系统完成）
│   ├── api/                    # invoke 封装层（tags.ts）
│   ├── components/common|business/
│   ├── composables/
│   ├── stores/                 # tagStore.ts（Pinia）
│   ├── views/                  # HomePage.vue / TagManage.vue
│   ├── router/index.ts         # / 与 /tags 路由（懒加载）
│   ├── types/                  # tag.ts 已填充 / asset.ts、api.ts 占位
│   ├── styles/variables.css    # 全局 CSS 变量
│   ├── utils/
│   ├── App.vue                 # 仅 <router-view />
│   ├── main.ts                 # createApp + router
│   ├── components.d.ts / auto-imports.d.ts   # unplugin 自动生成
│   └── vite-env.d.ts
├── src-tauri/                  # Rust 后端（7 模块域占位）
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs              # AppState 注入 + generate_handler（tag 命令已注册）
│   │   ├── commands/ models/ services/ db/ parsers/ download/ utils/
│   ├── capabilities/default.json  # dialog/fs/window 权限已声明
│   ├── Cargo.toml              # 全部 Rust 依赖
│   └── tauri.conf.json         # VRAMS 1200×800 + CSP
├── package.json                # 前端依赖 + unplugin 按需引入
├── vite.config.ts              # alias @/ + AutoImport/Components
├── tsconfig.json               # paths @/*
└── pnpm-lock.yaml
```

当前 Phase 3 完成：标签系统全栈可用的应用工程，下一步 Phase 4（资产导入核心）。

---

## 6. 遗留问题

| # | 问题 | 状态 | 备注 |
|---|------|------|------|
| 1 | 系统架构设计.md 中 assets 表仍有 `status` 字段 | ⚠️ 待同步 | 数据库设计已改为无 status |
| 2 | `sevenz-rust` crate 成熟度待验证 | ⚠️ 技术预研 | 备选方案：调用 7z CLI |
| 3 | 回收站操作 Rust 实现方案待定 | ⚠️ 待验证 | 备选：直接删除 |
| 4 | CSP 中 `'unsafe-inline'` 是安全弱项 | ✅ 已知接受 | MVP 后可优化 |
| 5 | Phase 8 前库目录路径未定 | ✅ 计划阶段处理 | Phase 8 实现引导前先写死路径 |
| 6 | `zip` 实装 8.6.0 与任务文档「2.x」版本线不符 | ⚠️ Phase 4 确认 | cargo add 自动取最新；`cargo check` 无冲突，Phase 4 实现 zip 解析时确认 8.x API 差异 |
| 7 | `reqwest` 默认 TLS 是 native-tls（非此前记录的 rustls） | ✅ 已修正记录 | Windows 走 schannel 无需 OpenSSL，当前无碍；未来跨平台构建需补 `rustls-tls` feature |
| 8 | `fs:default` 权限偏宽（含递归读写） | ⚠️ Phase 4 收窄 | 导入实现时按需限定 scope（如 dialog 选定路径） |
| 9 | 架构文档 §9 列出的若干模块未在 Phase 1 创建 | ⚠️ 后续补齐 | 如 `utils/temp_dir.rs`、`services/asset_service.rs`、`tag_service.rs`、前端 `layouts/`、`api/{assets,tags,system}.ts` |
| 10 | vswhere 注册信息丢失（删 VS temp 后） | ⚠️ 已绕过 | cargo 前跑 `vcvarsall.bat x64`；VS 本体完好；如需根治可 VS Installer 修复 |
| 11 | `unchecked_transaction` 嵌套事务隐患（asset_file/tag_repo） | ⚠️ Phase 4 处理 | Phase 4 导入「资产+文件+标签」单事务时统一事务边界，避免嵌套 BEGIN |
| 12 | `tags.name` 仅 UNIQUE 无长度约束（32 字符校验纯应用层） | ⚠️ 可选 | 后续迁移可加 `CHECK(length(name) <= 32)` 纵深防御 |
| 13 | 错误提示 `String(e)` 对 Error 对象会带 "Error: " 前缀 | ⚠️ 可选 | 可统一 `e instanceof Error ? e.message : String(e)` |
| 14 | 分组名恰为「全部」时与筛选关键字冲突 | ⚠️ 边缘场景 | 可用保留字规避（如常量 `ALL_KEY`） |
| 15 | 前后端 32 字符校验口径不一致（前端 UTF-16 code units / 后端 Unicode 标量） | ⚠️ 可选 | emoji 等代理对前端更严格（提前截断），无安全风险；可统一 |
| 16 | `createWebHistory` 生产环境（asset protocol）深层路由刷新可能 404 | ⚠️ 后续确认 | 待验证 Tauri asset fallback；或评估 hash 模式（Phase 9 集成时处理） |
| 17 | 默认库目录原为 exe 同级 `vrams-library/`：MSI 装入 Program Files 不可写→启动 panic；卸载/升级会连同资产库删除 | ✅ 已修复 | 改为 `{app_data_dir}/library`（`%APPDATA%\com.cxpller.vrams\library`），DB 初始化移入 `.setup()`；`VRAMS_LIBRARY_PATH` 覆盖保留。旧 dev 库（`target/debug/vrams-library`）不自动迁移 |

---

## 7. 后续建议

### 7.1 标准 Agent 工作流

> **教学模式默认开启**：以下流程不变，开发动作照常执行；同时按 `mentor` skill 在每个 Task 穿插概念讲解（用户喊停即停、喊「继续」即恢复）。

每次会话按以下流程执行，①② 只在需求/架构变化时触发：

```
① 上下文加载  读本文件 + CLAUDE.md + 相关 docs（每次会话第一步）
② 需求/设计   模糊需求 → product-analysis；新功能/架构变更 → architecture-design；产出更新 docs
③ 任务执行    按 docs/tasks/phaseX.md 逐 Task 开发，遵循 development-standard
④ 验证        pnpm build / cargo check / 单元测试 / pnpm tauri dev 冒烟
⑤ 审查        review skill → 问题清单 → 修复 → 复审通过（S1 未清零不提交）
⑥ 收尾        task checklist 打勾 → 更新本文件（状态/遗留问题）→ git commit
```

**审查触发粒度**: 每 Phase 完成后一次完整 review（对应 phaseX.md 末尾「Phase N Review」小节）；高风险 Task（如 4.8、6.3）单独 review。

### 7.2 其他建议

- **首次会话**: 读取本文件后，按 `docs/tasks/phase1-init.md` 开始执行 Phase 1
- **每完成一个 Task**: 在对应验收 checklist 中打勾
- **每完成一个 Phase**: 更新本文件 §4 的状态 + 执行第⑤步 review
- **文档同步**: Phase 1 完成后更新 CLAUDE.md 反映新目录结构
