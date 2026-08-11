//! 资产文件数据模型 — 对应 `asset_files` 表

use serde::{Deserialize, Serialize};

/// 资产文件记录 — 映射 `asset_files` 表
///
/// `file_name` 存相对路径 `{asset_id}/{原始文件名}`,
/// 磁盘完整路径为 `{library}/assets/{file_name}`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetFile {
    pub id: i64,
    pub asset_id: i64,
    pub file_name: String,
    pub file_size: Option<i64>,
    /// 是否主文件(1=是,0=否),卡片展示时确定主类型图标
    pub is_primary: i64,
    /// 文件类型: zip / 7z / unitypackage
    pub file_type: String,
    /// JSON 格式的压缩包内文件清单(可选缓存),前端自行 JSON.parse
    pub internal_info: Option<String>,
    pub created_at: String,
}
