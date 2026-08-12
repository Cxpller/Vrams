# Tauri 知识点（VRAMS Phase 1 沉淀）

## CSP（Content Security Policy）
- 一句话：声明 WebView 页面允许从哪些来源加载资源。
- 类比：快递驿站黑名单制度——`'self'` 只收自家件，`img-src https:` 额外收 https 图。
- VRAMS 策略：`default-src 'self'; img-src 'self' https:; style-src 'self' 'unsafe-inline'; script-src 'self'`
  - `style-src 'unsafe-inline'` 是 Element Plus 运行时样式必需（已知弱项）。
- 出处：Task 1.7

## Capabilities 权限模型（Tauri v2 新概念）
- 一句话：前端调用某插件能力前，必须先在 `capabilities/*.json` 显式声明权限。
- 类比：iOS 逐个问权限（v1 是默认全放行）。
- 已声明：`core:default` `opener:default` `dialog:default` `fs:default` `core:window:default`。

## 插件注册 vs 权限
- `.plugin(tauri_plugin_xxx::init())` 是把插件装进 Builder；capabilities 是授权前端调用。
- **两个都要做**，缺一个命令不可用。
- 出处：Task 1.7

## Element Plus 按需引入
- `unplugin-vue-components` + `unplugin-auto-import` 构建时自动补 import + 样式（ElementPlusResolver）。
- 产物：构建时自动生成 `src/components.d.ts`、`src/auto-imports.d.ts`（须在 tsconfig include 内）。
- 效果：CSS 1.40kB → 0.71kB（比全量引入小一半）。
- 出处：Task 1.3

## 代理环境变量
- 一句话：`HTTP_PROXY` / `HTTPS_PROXY` / `ALL_PROXY` 让命令行工具（reqwest 等）所有请求走代理。
- 用途：tauri-cli 下载 WiX/NSIS 卡 GitHub 时，`$env:HTTPS_PROXY="http://127.0.0.1:<port>"` 后重跑即通。
- 出处：Task 1.8

## 函数式 API 的按需引入样式坑（Phase 3 实战）
- 一句话：按需引入下，`ElMessage`/`ElMessageBox` 这类函数式 API 的样式**不会自动跟随** JS。
- 症状：显式 `import { ElMessageBox } from "element-plus"` 后功能正常，但弹窗无遮罩/无定位，内容堆在左上角。
- 根因：模板组件由 unplugin-vue-components 自动带样式；函数式 API 要么走 unplugin-auto-import 的 resolver（自动注入 JS+样式），要么**显式 import 样式**。
- 修复：`main.ts` 加 `import "element-plus/es/components/message/style/css"` + `"element-plus/es/components/message-box/style/css"`。
- 出处：Phase 3 冒烟（删除确认弹窗无样式）
