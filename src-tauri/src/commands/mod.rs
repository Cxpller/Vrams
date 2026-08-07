//! IPC 命令入口模块
//!
//! 每个 `#[tauri::command]` 函数按功能域拆到对应子模块，
//! 最终在 `lib.rs` 的 `invoke_handler!` 中统一注册。

pub mod asset_commands;
pub mod import_commands;
pub mod system_commands;
pub mod tag_commands;
