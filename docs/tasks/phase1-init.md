# Phase 1: 项目初始化与环境搭建

**所属**: VRAMS MVP 开发计划
**预估工时**: 1 天
**前置依赖**: 无（这是第一阶段）
**产出**: 可编译运行的空应用工程

---

## Task 1.1 — 安装 Rust 后端依赖

### 任务目标

在 `Cargo.toml` 中添加 VRAMS MVP 所需的全部 Rust crate 依赖，为后续所有后端开发（数据库、压缩包解析、HTTP 请求、系统对话框）提供基础库。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/Cargo.toml` | Rust 项目清单，添加依赖声明 |

### 修改内容

向 `[dependencies]` 中添加以下 crate：

| Crate | 版本 | 用途 | Phase 出处 |
|-------|------|------|-----------|
| `rusqlite` | 0.32+ (features = ["bundled"]) | SQLite 数据库操作 | Phase 2 |
| `zip` | 2.x | .zip 文件解析 | Phase 4 |
| `tar` | 0.4 | .unitypackage (tar 格式) 解析 | Phase 4 |
| `flate2` | 1.x | gzip 解压（.unitypackage 入口） | Phase 4 |
| `reqwest` | 0.12+ (features = ["blocking"]) | 从 Booth 下载预览图 | Phase 7 |
| `scraper` | 0.20+ | 解析 HTML 提取 og:image | Phase 7 |

以及以下 Tauri 官方插件：

| 插件 | 用途 |
|------|------|
| `tauri-plugin-dialog` | 文件选择器（导入时选择文件） |
| `tauri-plugin-fs` | 前端读取文件元信息 |

### 前置依赖

- 无

### 验收标准

- [ ] `cargo check` 编译通过，无依赖版本冲突
- [ ] 新增 crate 均成功下载并编译
- [ ] 所有 crate 版本兼容 Tauri 2 的 MSRV

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| `rusqlite` bundled 模式编译慢 | 首次编译耗时长 | 只影响首次，后续增量编译快 |
| `reqwest` 依赖 OpenSSL | Windows 上需额外配置 | 启用 `native-tls-vendored` feature 或使用 `rustls` |
| crate 版本与 Tauri 2 MSRV 不兼容 | 编译失败 | MVP 使用 `cargo add` 自动选择兼容版本，锁定 `Cargo.lock` |

---

## Task 1.2 — 创建 Rust 后端模块目录结构

### 任务目标

按照系统架构设计，在 `src-tauri/src/` 下建立 Rust 模块化目录结构，每个模块创建 `mod.rs` 作为入口。此时模块内部可为空，后续各 Phase 逐步填充。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/mod.rs` | IPC 命令入口模块 |
| `src-tauri/src/commands/asset_commands.rs` | 资产相关命令（占位） |
| `src-tauri/src/commands/tag_commands.rs` | 标签相关命令（占位） |
| `src-tauri/src/commands/import_commands.rs` | 导入相关命令（占位） |
| `src-tauri/src/commands/system_commands.rs` | 系统配置命令（占位） |
| `src-tauri/src/models/mod.rs` | 数据模型入口模块 |
| `src-tauri/src/models/asset.rs` | 资产模型（占位） |
| `src-tauri/src/models/asset_file.rs` | 资产文件模型（占位） |
| `src-tauri/src/models/tag.rs` | 标签模型（占位） |
| `src-tauri/src/services/mod.rs` | 业务逻辑入口模块 |
| `src-tauri/src/services/import_service.rs` | 导入业务逻辑（占位） |
| `src-tauri/src/db/mod.rs` | 数据库层入口模块 |
| `src-tauri/src/db/migrations.rs` | 迁移管理（占位） |
| `src-tauri/src/db/repository/mod.rs` | 仓储层入口模块 |
| `src-tauri/src/db/repository/asset_repo.rs` | 资产仓储（占位） |
| `src-tauri/src/db/repository/asset_file_repo.rs` | 资产文件仓储（占位） |
| `src-tauri/src/db/repository/tag_repo.rs` | 标签仓储（占位） |
| `src-tauri/src/parsers/mod.rs` | 压缩包解析入口模块 |
| `src-tauri/src/parsers/zip_archive.rs` | zip 解析器（占位） |
| `src-tauri/src/parsers/sevenz_archive.rs` | 7z 解析器（占位） |
| `src-tauri/src/parsers/unitypackage.rs` | unitypackage 解析器（占位） |
| `src-tauri/src/download/mod.rs` | 网络下载入口模块 |
| `src-tauri/src/download/thumbnail.rs` | 预览图下载（占位） |
| `src-tauri/src/utils/mod.rs` | 工具函数入口模块 |
| `src-tauri/src/utils/path.rs` | 路径工具（占位） |
| `src-tauri/src/utils/recycle.rs` | 回收站操作（占位） |

### 修改内容

1. 在 `src-tauri/src/` 下创建 7 个模块目录：`commands/` `models/` `services/` `db/` `parsers/` `download/` `utils/`
2. 在每个模块目录下创建 `mod.rs`，使用 `pub mod` 声明子模块
3. 在 `db/repository/` 子目录下创建 `mod.rs`
4. 在 `lib.rs` 中通过 `mod commands;` 等声明所有顶级模块
5. 占位文件内容为空的模块声明，例如：

```rust
// 示例：commands/mod.rs（示意，不写代码）
// 声明所有 command 子模块
// 子模块文件先创建空的占位文件
```

### 前置依赖

- Task 1.1（依赖已安装，`cargo check` 才有意义）

### 验收标准

- [ ] 所有模块目录和 `mod.rs` 文件创建完毕
- [ ] `cargo check` 通过，无未使用的模块警告（可使用 `#[allow(dead_code)]` 临时标注）
- [ ] `lib.rs` 中正确声明了所有模块

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| `lib.rs` 过长的模块声明 | 可读性下降 | 按功能域分组注释，每个模块一行 |
| 空格命名冲突（如 `sevenz_archive` 命名歧义） | 编译正常但可读性差 | 命名统一用 snake_case，见名知意 |

---

## Task 1.3 — 清理脚手架默认代码

### 任务目标

移除 Tauri + Vue 脚手架生成的示例代码（greet 命令、Vue logo、模板样式等），替换为 VRAMS 的空白占位。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/App.vue` | 根组件 → 替换为布局骨架 |
| `src/main.ts` | 入口 → 保留基础挂载，移除多余引用 |
| `src/assets/vue.svg` | 移除（无用） |
| `public/vite.svg` | 保留（可后续替换为 VRAMS logo） |
| `public/tauri.svg` | 移除（无用） |
| `src-tauri/src/lib.rs` | 移除 `greet` 命令，保留空框架 |

### 修改内容

**`src-tauri/src/lib.rs`**:
- 删除 `greet` 函数
- 保留 `run()` 函数框架，暂时不注册任何命令
- 保留 `tauri_plugin_opener::init()` 插件注册

**`src/App.vue`**:
- 删除模板中的 greet 表单、logo 展示、欢迎文字
- 保留 `<script setup lang="ts">` 空壳
- 保留基础 `<template>` 结构，只留 `<router-view />`（路由在 Task 1.5 建立）
- 删除脚手架默认样式，替换为 Element Plus 全局样式引入

**`src/main.ts`**:
- 保留 `createApp(App).mount("#app")`
- 引入 Element Plus 全局注册（`import ElementPlus from "element-plus"` + `app.use(ElementPlus)`）
- 引入 Element Plus 样式（`import "element-plus/dist/index.css"`）

**资源文件**:
- 删除 `src/assets/vue.svg`
- 删除 `public/tauri.svg`
- 保留 `public/vite.svg`（作为临时占位图标）

### 前置依赖

- Task 1.4（前端依赖安装后，Element Plus 的 import 才能通过编译）

### 验收标准

- [ ] `pnpm dev` 启动后页面为空白布局，无 greet 表单
- [ ] `cargo check` 通过，无未使用的函数警告
- [ ] 编译产物中不包含 greet 相关代码
- [ ] Element Plus 全局注册成功（可在 Vue DevTools 中看到 ElementPlus 组件）

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| 删除 greet 后 `lib.rs` 缺少注册命令 | 运行不报错，但若有旧前端代码调用 greet 会报错 | 同时清理前端对 greet 的调用（App.vue 中） |
| Element Plus 全局引入体积大 | 首屏加载变慢 | MVP 阶段先全量引入，后续按需引入做优化 |

---

## Task 1.4 — 安装前端依赖

### 任务目标

在 `package.json` 中添加 VRAMS 前端所需的生产和开发依赖。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `package.json` | 前端依赖清单 |

### 修改内容

向 `dependencies` 中添加：

| 包名 | 用途 | 对应 Phase |
|------|------|-----------|
| `element-plus` | UI 组件库 | 全局 |
| `@element-plus/icons-vue` | Element Plus 图标集 | 全局 |
| `vue-router` | 前端路由 | Phase 3 |
| `pinia` | 状态管理 | Phase 3 |
| `@vueuse/core` | Vue 工具集（防抖、拖拽等） | Phase 5 |

### 前置依赖

- 无（可以提前安装，与 Task 1.2~1.3 并行）

### 验收标准

- [ ] `pnpm install` 成功完成，无版本冲突
- [ ] `pnpm-lock.yaml` 正确锁定所有依赖版本
- [ ] 在 `main.ts` 中 `import ElementPlus from "element-plus"` 编译通过

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| `element-plus` 与 `vue` 版本不兼容 | 运行时组件渲染异常 | 安装前检查 peerDependencies，使用 element-plus 最新版 |
| `@vueuse/core` 与 Vue 3.5 兼容性 | 极少数 composable 可能行为异常 | 锁定 `@vueuse/core` 大版本，验证 `useDebounceFn` 等常用 API |
| pnpm workspace 配置 (`pnpm-workspace.yaml`) 禁用了 esbuild | 可能与某些依赖构建冲突 | 确认当前配置不会阻塞 element-plus 的 postinstall 脚本 |

---

## Task 1.5 — 创建前端目录结构

### 任务目标

按照系统架构设计，在 `src/` 下建立前端模块化目录结构，初始化各目录入口文件。

### 涉及文件

```
src/api/             ← Tauri invoke 封装层
src/components/
  common/            ← 通用组件目录
  business/          ← 业务组件目录
src/composables/     ← 组合式函数目录
src/stores/          ← Pinia 状态仓库目录
src/views/           ← 页面组件目录
src/router/          ← 路由配置目录
  index.ts           ← 路由初始化（暂空）
src/types/           ← TypeScript 类型定义目录
  asset.ts           ← 资产类型（占位）
  tag.ts             ← 标签类型（占位）
  api.ts             ← API 请求/响应类型（占位）
src/styles/          ← 全局样式目录
  variables.css      ← 全局 CSS 变量
src/utils/           ← 工具函数目录
```

### 修改内容

1. 在 `src/` 下创建上述目录结构
2. 在每个业务目录下创建文件占位：
   - `types/` 下创建 `asset.ts` `tag.ts` `api.ts`，导出空接口（占位）
   - `router/index.ts` 创建路由实例（暂空路由表）
   - `styles/variables.css` 创建全局 CSS 变量（颜色、栅格、间距等）
3. 在 `main.ts` 中引入路由：`app.use(router)`
4. 在 `App.vue` 中引入 `<router-view />`

### 前置依赖

- Task 1.3（App.vue 已清理）
- Task 1.4（pinia、vue-router 已安装）

### 验收标准

- [ ] 所有目录创建完毕，结构对齐系统架构设计
- [ ] 类型文件可被正确 import（`import { Asset } from "@/types/asset"`）
- [ ] 路由实例创建成功，`pnpm build` 无 TS 错误
- [ ] `App.vue` 中 `<router-view />` 渲染（当前为空路由，白屏正常）

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| 目录过多但大部分为空 | 初看冗余 | 每个目录放 `README.md` 简要说明用途，便于开发者理解 |
| TypeScript 空接口导致 lint 警告 | `noUnusedLocals` 触发 | 使用 `export {}` 或 `// eslint-disable-next-line` 临时绕过 |

---

## Task 1.6 — 配置 Vite 和 TypeScript 路径别名

### 任务目标

配置 `@/` 路径别名指向 `src/` 目录，避免深层相对路径引用（`../../../`）。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `vite.config.ts` | Vite 配置，添加 `resolve.alias` |
| `tsconfig.json` | TypeScript 配置，添加 `paths` 和 `baseUrl` |

### 修改内容

**`vite.config.ts`**: 在 `defineConfig` 的返回对象中添加 `resolve.alias`：

```typescript
// 修改点：在 plugins 同级添加
import { fileURLToPath, URL } from "node:url";

resolve: {
  alias: {
    "@": fileURLToPath(new URL("./src", import.meta.url)),
  },
},
```

**`tsconfig.json`**: 在 `compilerOptions` 中添加：

```json
"baseUrl": ".",
"paths": {
  "@/*": ["src/*"]
}
```

### 前置依赖

- Task 1.4（依赖安装后修改配置才有验证意义）

### 验收标准

- [ ] 在任意 `.ts` 文件中 `import X from "@/types/asset"` 编译通过
- [ ] 在任意 `.vue` 文件中 `<script setup lang="ts">` 内使用 `@/` 路径编译通过
- [ ] VSCode 中 `@/` 路径有自动补全提示
- [ ] `pnpm build` 通过，无路径解析错误

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| Vite alias 和 tsconfig paths 不一致 | 编译通过但 VSCode 报错（或相反） | 保持两处配置一致，tsconfig 的 `paths` 必须和 Vite 的 `alias` 对应 |
| `import.meta.url` 在 Node 环境中可用性 | Vite 构建时运行在 Node 端，正常 | 确认 `@ts-expect-error` 或使用 `path.resolve(__dirname, "../src")` 备选方案 |

---

## Task 1.7 — 更新 Tauri 配置和安全设置

### 任务目标

更新 `tauri.conf.json` 和 `capabilities/default.json`，配置窗口标题、安全策略、权限。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/tauri.conf.json` | Tauri 核心配置 |
| `src-tauri/capabilities/default.json` | 窗口权限声明 |

### 修改内容

**`tauri.conf.json`**:
- 窗口标题改为 `"VRAMS"`
- 窗口默认尺寸建议 `1200×800`（资产卡片浏览需要更宽视野）
- 设置合理 CSP 策略（代替当前 `"csp": null`）：

```json
"security": {
  "csp": "default-src 'self'; img-src 'self' https:; style-src 'self' 'unsafe-inline'; script-src 'self'"
}
```

说明：`img-src https:` 允许 Booth 预览图加载；`style-src 'unsafe-inline'` 为 Element Plus 必需。

**`capabilities/default.json`**:
- 追加新插件权限：

| 权限 | Phase 出处 |
|------|-----------|
| `dialog:default` | Phase 4（文件选择器） |
| `fs:default` | Phase 4（文件读取） |
| `core:window:default` | Phase 6（打开文件夹） |

### 前置依赖

- Task 1.1（tauri-plugin-dialog 和 tauri-plugin-fs 安装后，capabilities 中的权限声明才有效）

### 验收标准

- [ ] `pnpm tauri dev` 启动后窗口标题显示为 "VRAMS"
- [ ] 窗口默认尺寸为 1200×800
- [ ] CSP 策略配置后无控制台安全警告（Element Plus 样式加载正常）
- [ ] `tauri-plugin-dialog` 和 `tauri-plugin-fs` 的权限生效

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| CSP `'unsafe-inline'` 降低安全防护 | XSS 防护减弱 | MVP 阶段接受此风险，后续优化为 hash 或 nonce 模式 |
| 窗口尺寸 1200×800 不适应小屏幕 | 部分用户窗口超屏 | 保留手动缩放能力，后续适配记住窗口大小 |
| `dialog:default` 权限不足 | 文件选择器无法打开 | 使用 `dialog:allow-open` 等更细粒度权限 |

---

## Task 1.8 — 安装依赖并验证构建

### 任务目标

执行依赖安装、运行 `cargo check` 和 `pnpm build`，确保工程在所有改动后仍可正常编译。

### 涉及文件

- 无（仅执行命令）

### 修改内容

按顺序执行：

```bash
# 1. Rust 依赖下载并编译检查
cargo check --manifest-path src-tauri/Cargo.toml

# 2. 前端依赖安装
pnpm install

# 3. 前端类型检查 + 构建
pnpm build

# 4. Tauri 完整构建验证
pnpm tauri build --debug
```

### 前置依赖

- Task 1.1 ~ Task 1.7 全部完成

### 验收标准

- [ ] `cargo check` 通过，无错误、无 warning
- [ ] `pnpm install` 成功，lockfile 更新
- [ ] `pnpm build`（vue-tsc + vite build）通过，无类型错误
- [ ] `pnpm tauri dev` 启动后窗口正常显示（空布局，无控制台报错）
- [ ] 所有 Phase 1 验收标准 checklist 全部满足

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| `vue-tsc` 类型检查失败 | 阻止构建 | 检查 `tsconfig.json` 配置，确保 `paths` 和 `include` 正确 |
| `cargo check` 因 module 声明不一致失败 | 阻止构建 | 检查 `lib.rs` 中 `mod` 声明与文件系统一致 |
| Tauri 插件未正确初始化 | 运行时 panic | 检查 `lib.rs` 中 `.plugin(...)` 调用与 capabilities 权限匹配 |

---

## Phase 1 完整验收 Checklist

完成后逐项确认：

- [ ] 1.1 Rust 依赖安装完成，`cargo check` 通过
- [ ] 1.2 后端模块目录结构创建完毕，所有 `mod.rs` 就位
- [ ] 1.3 脚手架 greet 代码已移除，Element Plus 已注册
- [ ] 1.4 前端依赖安装完成，`pnpm build` 通过
- [ ] 1.5 前端目录结构创建完毕，类型文件、路由占位就位
- [ ] 1.6 Vite + TypeScript 路径别名 `@/` 配置完成
- [ ] 1.7 Tauri CSP 已配置，窗口 1200×800，权限已更新
- [ ] 1.8 全量构建通过（`pnpm tauri build --debug`）
- [ ] 应用启动后显示空白布局，无 greet 残留，无控制台报错
- [ ] 工程状态可向 Phase 2（数据库与数据模型）交付

---

## 任务执行顺序建议

```
Day 1 上午:
  Task 1.1 (Rust 依赖) ─┐─ 可并行 ─┐
  Task 1.4 (前端依赖) ───┘          │
                                    ├─ 安装完成后继续
Day 1 下午:                          │
  Task 1.2 (Rust 模块)  ← 依赖 1.1 ─┘
  Task 1.5 (前端目录)  ← 依赖 1.4 ─┘
  Task 1.3 (清理代码)  ← 依赖 1.5 (路由)
  Task 1.6 (路径别名)  ← 依赖 1.3 (可同时)
  Task 1.7 (Tauri 配置) ← 依赖 1.1 (插件)
  Task 1.8 (验证构建)  ← 依赖全部完成
```

**并行建议**: Task 1.1（Rust 依赖）和 Task 1.4（前端依赖）可以同时触发安装，因为一个在 `Cargo.toml` 一个在 `package.json`，互不依赖。

---

## Phase 1 Review（审查环节）

**触发时机**: 本 Phase 全部 Task 完成、验证通过后执行（标准流程第⑤步）。

**审查方式**: 调用 `review` skill（`.claude/skills/review/SKILL.md`），审查范围为 Phase 1 的全部变更（`git diff` 或涉及文件清单）。

**审查重点**:

- [ ] 依赖 crate 版本兼容 Tauri 2 MSRV，无冗余依赖
- [ ] 后端模块目录结构对齐 `docs/系统架构设计.md`
- [ ] CSP 已配置（非 null），`capabilities/` 权限最小化
- [ ] Vite alias 与 tsconfig `paths` 双端一致
- [ ] greet 脚手架代码清理干净（前端 + 后端均无残留）
- [ ] `cargo check` / `pnpm build` / `pnpm tauri build --debug` 全部通过

**输出**: 审查报告（结论 + 问题清单 + 验收核对表）。S1 阻断问题未清零前，不允许 git commit / 交付下一 Phase。
