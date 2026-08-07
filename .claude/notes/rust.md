# Rust 知识点（VRAMS Phase 1 沉淀）

## 模块系统 `mod`
- 一句话：Rust 用 `mod` 把代码组织成文件夹式命名空间，`mod.rs` 是每个目录的「说明书」。
- 类比：电脑文件夹；`src/models/asset.rs` = models 文件夹下的 asset 文件。
- 出处：Task 1.2

## 占位模块与 dead_code
- 空占位 `.rs` 文件合法编译，不会产生 unused 警告；填入代码后若未被引用会触发 dead_code，可临时 `#[allow(dead_code)]`。

## rusqlite bundled
- 一句话：`bundled` feature 在编译期现场编译 SQLite C 源码，免系统安装。
- 类比：外卖带锅上门做饭（首次备料久，之后快）。
- 出处：Task 1.1 / 1.8

## cargo check 首次编译慢
- 下载 + 编译全部依赖树（本项目 ~170 crate），首次 1-2 分钟+，之后增量秒级。

## PowerShell 下跑 cargo 的坑
- cargo 把编译进度写到 **stderr**，PowerShell `2>&1` 会抛 NativeCommandError 并误报 exit 1。
- 正确姿势：不加 `2>&1` 直接跑，用 `$LASTEXITCODE` 判断真实退出码。
