//! 标签数据模型 — 对应 `tags` 表

use serde::{Deserialize, Serialize};

/// 标签记录 — 映射 `tags` 表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    /// 标签名,唯一约束
    pub name: String,
    /// 分组名,用于标签分类展示,如 "角色" / "类型" / "状态"
    pub group_name: String,
    pub created_at: String,
}

/// 创建标签的请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    #[serde(default)]
    pub group_name: String,
}

/// 更新标签的请求 — 所有字段可选
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
}
