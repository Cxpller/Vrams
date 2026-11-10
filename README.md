# VRAMS — Virtual Reality Asset Management System

面向 VRChat 用户及 Unity 创作者的**本地素材管理工具**。

技术栈：Tauri v2 + React 19 + TypeScript + Ant Design + Rust + SQLite（包管理器 pnpm）。

> **v2 重写中**（`v2` 分支）。v1（Vue 版，仅完成标签系统）已存档为标签 `v1-final`。

## 快速开始

```bash
pnpm install
pnpm tauri dev    # 桌面应用开发模式（热重载）
pnpm build        # 前端类型检查 + 构建
pnpm tauri build  # 打包安装程序（MSI / NSIS）
```

Rust 侧：

```bash
cd src-tauri
cargo test
cargo clippy --all-targets -- -D warnings
```

> 需要 Rust（MSVC 工具链）和 VS Build Tools 的 C++ 组件。`pnpm tauri dev` 使用固定端口 1420，被占用会启动失败。

## 文档

| 文档 | 说明 |
|------|------|
| [v2 需求与框架](docs/v2-需求与框架.md) | **当前**：功能清单、关键决策、数据模型、开发路线 |
| [v1 存档](docs/archive/v1/) | 旧版需求、架构、数据库设计与 Phase 任务 |
| [Reasonix 使用说明](docs/REASONIX使用说明.md) | 缓存友好会话指南 |

## 目录结构

```
vrams/
├── .claude/        # 项目上下文、教学笔记、skills
├── .github/        # CI（Windows 上前端构建 + cargo test/clippy）
├── docs/           # v2 需求与框架、v1 存档
├── src/            # React 前端
└── src-tauri/      # Rust 后端（db / utils，按需扩展）
```
