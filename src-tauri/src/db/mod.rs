//! 数据库层入口模块
//!
//! 负责 SQLite 连接管理、schema 迁移（migrations）与仓储层（repository）。

pub mod migrations;
pub mod repository;
