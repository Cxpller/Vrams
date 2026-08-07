# Agent 工作流知识点（VRAMS Phase 1 沉淀）

## 报错 4 段读取法
- 格式：`文件(行,列): error TS6133: 原因`
- 逐段读：哪个文件 → 精确位置 → 错误编号 → 原因；原因看不懂再查编号。
- 出处：Task 1.5（TS6133 空接口泛型未使用）

## 路径别名双端配置
- Vite `resolve.alias` 管构建，tsconfig `paths` 管编辑器/类型检查——**两处必须一致**。
- 只配一处：要么构建报错、要么编辑器飘红。
- 出处：Task 1.6

## 后台任务 + wait 模式
- 长命令（cargo 首次编译、tauri build 打包）用 run_in_background 起后台任务，`wait`/`bash_output` 轮询。
- 避免前台超时，期间可并行做别的事（如 pnpm install 验证）。

## 环境变更检查先行
- 恢复被卡住的任务前，先检查环境是否变化（vswhere 查 VS Build Tools、PATH 查 link.exe），不要盲目重试。

## 决策点用 ask 工具
- 无法从上下文推断的用户决策（装系统软件/换 toolchain/改方案），用 ask 让用户选，不擅自决定。
