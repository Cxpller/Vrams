# vrams × Reasonix 缓存友好启动说明

> 目的：让 Reasonix（DeepSeek 终端编码代理）在 vrams 项目上以最低成本运行。
> 原理：DeepSeek 前缀缓存命中 → 上下文前缀相同 → 计费结果直接复用（命中率 ~99.8%，费用约降 80%）。
> 一切纪律围绕两个字：**稳定**。

---

## 0. 项目上下文速览（供人读，不要给代理读这个清单本身）

- 记忆文件：`.claude/CONTEXT.md`（实时进度、决策记录、遗留问题，**会话结束时更新**）
- 架构概览：`CLAUDE.md`（Tauri v2 + Vue3 + TS + Rust + SQLite，pnpm）
- 顶层文档：`docs/产品需求文档.md`、`docs/系统架构设计.md`、`docs/数据库设计.md`、`docs/开发计划.md`
- 任务文档：`docs/tasks/phase1-init.md` ~ `phase9-integration.md`（每份含：任务目标/涉及文件/修改内容/前置依赖/验收标准/风险点）
- 当前进度：P1 ✅ P2 ✅ P3 ✅（标签系统全栈）→ 下一步 Phase 4（资产导入核心，13 个 Task）

---

## 1. 一次性准备（做完一劳永逸）

| 项 | 状态 | 说明 |
|---|---|---|
| `README.md` | ✅ 已替换 | 现在是中文项目索引，指向 docs/，不再是无用模板 |
| `CLAUDE.md` | 保持现状 | 唯一架构事实源；**只在架构变化时更新，绝不在会话中途改** |
| `.claude/CONTEXT.md` | 保持现状 | 记忆文件；**只在会话收尾更新**（对应工作流第⑥步） |
| `docs/*.md` | 保持现状 | 稳定前缀的核心资产，别删别改 |
| `auto-imports.d.ts` / `components.d.ts` | ⚠️ 确认 | unplugin 自动生成，构建会 churn；确认未被 git 跟踪（`.gitignore` 未列出它们，值得 `git status` 查一次，若已跟踪则加入忽略） |

## 2. 会话纪律（省钱的 80%）

1. **一个 Phase = 一个长会话**。Phase 4 有 13 个 Task / 3 天工时 → 挂着一个会话连续干，别拆成十几个短会话。Reasonix 的定位就是"leave it running"。
2. **开场模板一字不改地复用**（见 §3）。这是你所有会话里最长的公共前缀，每次原样粘贴才能命中缓存。
3. **前端一轮、Rust 一轮**：Phase 4 以 Rust 为主（parsers → import_service → asset commands → 前端 API/页面），按阶段推进，别在会话里来回横跳。
4. **任务写完整再丢**：说清"读哪些文件、做什么、验收标准"，不给它自由探索的余地（让它自己扫目录 = 白烧 token）。
5. **prompt 里直接给文件路径**，禁止它每次遍历目录树。
6. **小任务攒批**：3~5 个同类小改动一次发，别一次一个。

## 3. 固定开场模板（复制即用，每次一字不改）

### 模板 A · Phase 会话（最常见）

```
项目 vrams（VRAMS，Tauri v2 + Vue 3 + TypeScript + Rust + SQLite，包管理器 pnpm）。
严格按以下顺序读取上下文，然后执行 docs/tasks/phase4-import.md：
1. .claude/CONTEXT.md     （项目状态与决策记录）
2. CLAUDE.md              （架构概览）
3. docs/系统架构设计.md
4. docs/数据库设计.md
5. docs/tasks/phase4-import.md （本次任务）
执行规范：一次只做一个 Task；涉及文件以任务文档「涉及文件」为准，路径直接引用；
验证用 cargo check（过滤只看 error 摘要）和 pnpm build；不修改 .claude/CONTEXT.md 与 CLAUDE.md；
全部完成后更新 .claude/CONTEXT.md 的进度与遗留问题，并 git commit。
```

### 模板 B · 单 Task 会话（短任务专用，比 A 更省）

```
项目 vrams。先读 .claude/CONTEXT.md、CLAUDE.md，再按 docs/tasks/phase4-import.md 的
Task 4.5 执行（涉及文件见该节，直接引用路径）。
任务：<一句话描述>。验收：<照抄该 Task 的验收标准>。验证：cargo check + pnpm build。
完成后 git commit。
```

### 模板 C · 修复会话

```
项目 vrams。先读 .claude/CONTEXT.md §6 遗留问题 #11（unchecked_transaction 嵌套事务隐患）。
背景与代码位置：src-tauri/src/db/repository/ 下 asset_file_repo.rs 与 tag_repo.rs。
任务：Phase 4 导入「资产+文件+标签」单事务场景下消除嵌套 BEGIN，统一事务边界。
验收：cargo test 全绿、cargo clippy 无新增警告；不得改动其他模块。
```

> 三个模板的**前半段（第 1~2 行）完全一致**，这是全项目最高频复用的前缀，务必原样。

## 4. 验证节奏（构建输出是烧钱大户）

- **Task 粒度验证**：一个 Task 做完验证一次（`cargo check` + 相关单元测试），别改一行跑一次全量。
- `cargo check` 比 `cargo build` 快且输出少，日常用它；让代理只回错误摘要，别把完整输出贴回上下文。
- 前端 `pnpm build`（vue-tsc + vite）输出长，攒一批改动再跑。
- 禁止让它打开 `dist/`、`node_modules/`、`src-tauri/target/` 等大目录。

## 5. 配置建议（具体参数以 Reasonix 官方文档为准）

- 机械改动（搬运、补测试、改文案）用**非思考档**；只有架构设计、疑难 bug 才开强推理。
- 打开**自动批准**：每步停下来等确认会换工具、断缓存前缀，打断越少越省。
- 会话内连续干活 > 断点续传：断点续传本质是全新前缀。

## 6. 每会话检查清单

- [ ] `git status` 干净（半成品重构会污染上下文）
- [ ] `.claude/CONTEXT.md` 与当前代码一致（上个会话收尾过）
- [ ] 本次开场模板（§3）一字不改粘贴
- [ ] 任务清单、涉及文件路径一次给全
- [ ] 收尾固定动作：更新 CONTEXT.md 进度 → git commit

---

## 7. 什么时候回到 DSH（别浪费你的钱，也别浪费你的时间）

| 场景 | 用哪个 |
|---|---|
| 按 phase 文档推进开发、挂机长跑 | Reasonix |
| 大重构 / 多模块并行（前后端同时改） | DSH（并行子代理 + 后台任务） |
| 跨多轮长目标、会话中断后续跑 | DSH（goal 机制） |
| 需要完整计划先审批再动手 | DSH（计划模式） |
| 查最新 API 文档（如 Tauri v2 / zip crate 8.x） | DSH（内置 web_search） |

两者操作同一个工作目录，切换零迁移成本。
