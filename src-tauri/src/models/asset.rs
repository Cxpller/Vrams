//! 资产数据模型 — 对应 `assets` 表
//!
//! 请求/响应分离:创建与更新请求不包含 `id`,由后端生成。

use serde::{Deserialize, Serialize};

/// 资产完整记录 — 映射 `assets` 表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub source_url: Option<String>,
    pub thumbnail: Option<String>,
    pub notes: Option<String>,
    /// ISO 8601 格式,如 `2026-07-28 14:30:00`
    pub created_at: String,
    pub updated_at: String,
}

/// 首页卡片列表的轻量视图 — 资产基础字段 + 统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetListItem {
    pub id: i64,
    pub name: String,
    pub thumbnail: Option<String>,
    pub created_at: String,
    /// 关联文件数(LEFT JOIN COUNT)
    pub file_count: i64,
    /// 主文件的类型,用于卡片图标;无主文件时为 None
    pub primary_type: Option<String>,
}

/// 导入时创建资产的请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAssetRequest {
    pub name: String,
    #[serde(default)]
    pub source_url: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

/// 编辑时更新资产的请求 — 所有字段可选,仅更新非 None 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAssetRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub source_url: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}
