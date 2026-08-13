# VRAMS — Virtual Reality Asset Management System

面向 VRChat 用户及 Unity 创作者的**本地化虚拟资产管理工具**。
技术栈：Tauri v2 + Vue 3 + TypeScript + Rust + SQLite（包管理器 pnpm）。

## 快速开始

```bash
pnpm install
pnpm dev          # 前端 Vite 开发服务器（端口 1420）
pnpm tauri dev    # 桌面应用开发模式（热重载）
pnpm build        # 类型检查（vue-tsc）+ 前端生产构建
pnpm tauri build  # 打包安装程序（MSI / NSIS）
```

> 注意：`pnpm tauri dev` 使用**固定端口 1420**，端口被占用会启动失败。
> Rust 构建需要 MSVC 工具链；若 vswhere 注册信息丢失，先执行 `vcvarsall.bat x64` 再跑 cargo（见 `.claude/CONTEXT.md` §6 遗留问题 #10）。

## 文档索引

| 文档 | 说明 |
|------|------|
| [产品需求文档](docs/产品需求文档.md) | 需求定稿（用户故事 / 用例 / 优先级） |
| [系统架构设计](docs/系统架构设计.md) | 技术架构定稿（架构图 / 存储 / 解析 / API） |
| [数据库设计](docs/数据库设计.md) | ER 图 / DDL / 查询 / 迁移 |
| [开发计划](docs/开发计划.md) | MVP 计划：9 Phase / 12 天 / 63 任务 |
| [任务清单](docs/tasks/) | 各 Phase 详细任务（phase1-init ~ phase9-integration） |
| [项目上下文](.claude/CONTEXT.md) | 实时进度、关键决策、遗留问题（记忆文件） |
| [Reasonix 使用说明](docs/REASONIX使用说明.md) | 缓存友好会话指南（低成本运行 / 省钱） |

## 项目状态

**P1 ✅ P2 ✅ P3 ✅** — 工程脚手架、数据库与数据模型、标签系统（全栈）均完成。
下一步：**Phase 4 资产导入（核心）**，详见 [`docs/tasks/phase4-import.md`](docs/tasks/phase4-import.md)。

## 目录结构

```
vrams/
├── .claude/        # 项目上下文、教学笔记、skills
├── docs/           # 需求 / 架构 / 数据库 / 计划 / 任务
├── src/            # Vue 前端（api / components / stores / views / router / types）
└── src-tauri/      # Rust 后端（commands / models / services / db / parsers / download / utils）
```
