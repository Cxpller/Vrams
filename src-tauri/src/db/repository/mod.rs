//! 仓储层入口模块
//!
//! 每个仓储封装一张表的增删改查，向上层屏蔽 SQL 细节。
//! Phase 3 的 commands 层接入前,仓储函数尚未被引用,临时允许 dead_code。

#![allow(dead_code)]

pub mod asset_file_repo;
pub mod asset_repo;
pub mod tag_repo;

use std::fmt;

/// 仓储层统一错误类型
///
/// - `Db`: 底层 rusqlite 错误(SQL 语法、约束冲突等)
/// - `NotFound`: 业务层面的"目标记录不存在"
#[derive(Debug)]
pub enum RepoError {
    Db(rusqlite::Error),
    NotFound(String),
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepoError::Db(e) => write!(f, "数据库错误: {e}"),
            RepoError::NotFound(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for RepoError {}

impl From<rusqlite::Error> for RepoError {
    fn from(e: rusqlite::Error) -> Self {
        RepoError::Db(e)
    }
}
