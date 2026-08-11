# Rust 知识点（VRAMS Phase 1 + 2 沉淀）

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

## include_str! 相对路径
- 一句话：`include_str!` 路径相对于**当前源文件所在目录**，不是项目根或 Cargo.toml。
- 坑：`src/db/mod.rs` 引 `src-tauri/migrations/V001__init.sql` 要写 `../../migrations/`（编译器 help 会直接给出正确路径）。
- 类比：你在 `src/db/` 这间房里，去 `src-tauri/migrations/` 要「上一级、再上一级、再进文件夹」。
- 出处：Task 2.2

## Repository 模式（仓储）
- 一句话：把 SQL 封装在 `*_repo.rs` 门内，上层只调语义化函数，不碰 SQL。
- 类比：餐厅点菜——菜单（函数）≠ 后厨（SQL），换后厨菜单不变。
- 出处：Task 2.4-2.6

## 事务 transaction
- 一句话：一组 SQL 操作要么全成功、要么全回滚。
- `transaction()` 要 `&mut Connection`；`unchecked_transaction()` 放宽为 `&Connection`（并发由 Mutex 保证，本项目安全）。
- 类比：转账扣款+入账必须一起成功。
- 出处：Task 2.5/2.6

## 动态 SQL（部分更新）
- 只动态拼**固定列名**（代码内白名单），值一律走 `?` 参数绑定——无注入风险。
- 出处：Task 2.4 update_asset
