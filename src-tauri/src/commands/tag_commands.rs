//! 标签相关 IPC 命令 — Phase 3
//!
//! 职责:接收前端 invoke 参数 → 加锁取数据库连接 → 调 repo 层 → 返回结果。
//! command 层只做"参数校验 + 错误转友好文案",SQL 逻辑全在 repository 层。

use tauri::State;

use crate::db::repository::tag_repo;
use crate::db::repository::RepoError;
use crate::models::tag::{CreateTagRequest, Tag, UpdateTagRequest};
use crate::AppState;

/// 查询全部标签(按分组名+名称排序),用于标签管理页与资产编辑时的选择器
#[tauri::command]
pub fn get_tags(state: State<AppState>) -> Result<Vec<Tag>, String> {
    let conn = state.db.lock().unwrap();
    tag_repo::get_all_tags(&conn).map_err(|e| e.to_string())
}

/// 创建标签
///
/// 校验:
/// - name 去空白后非空
/// - 重名由数据库 UNIQUE 约束兜底,转成友好错误(而非"先查再插",避免竞态)
#[tauri::command]
pub fn create_tag(
    state: State<AppState>,
    name: String,
    group_name: Option<String>,
) -> Result<Tag, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("标签名不能为空".into());
    }
    if name.chars().count() > 32 {
        return Err("标签名不能超过 32 个字符".into());
    }

    let req = CreateTagRequest {
        name: name.to_string(),
        // 模型层 group_name 是 String(非 Option),前端未传时用空串兜底
        group_name: group_name.unwrap_or_default().trim().to_string(),
    };

    let conn = state.db.lock().unwrap();
    tag_repo::insert_tag(&conn, &req).map_err(friendly_tag_error)
}

/// 更新标签(部分更新:只改传了值的字段)
#[tauri::command]
pub fn update_tag(
    state: State<AppState>,
    id: i64,
    name: Option<String>,
    group_name: Option<String>,
) -> Result<Tag, String> {
    // 若提供了 name,同样做非空 + 长度校验
    if let Some(n) = &name {
        let n = n.trim();
        if n.is_empty() {
            return Err("标签名不能为空".into());
        }
        if n.chars().count() > 32 {
            return Err("标签名不能超过 32 个字符".into());
        }
    }

    let req = UpdateTagRequest {
        name: name.map(|n| n.trim().to_string()),
        group_name: group_name.map(|g| g.trim().to_string()),
    };

    let conn = state.db.lock().unwrap();
    tag_repo::update_tag(&conn, id, &req).map_err(friendly_tag_error)
}

/// 删除标签(asset_tags 关联由 ON DELETE CASCADE 自动清理)
#[tauri::command]
pub fn delete_tag(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    tag_repo::delete_tag(&conn, id).map_err(|e| e.to_string())
}

/// 把 repo 错误转成用户可读文案
///
/// 重名会触发 tags.name 的 UNIQUE 约束,SQLite 报 "UNIQUE constraint failed",
/// 这里识别出来换成友好提示;其余错误原样透传。
fn friendly_tag_error(e: RepoError) -> String {
    match &e {
        RepoError::Db(rusqlite::Error::SqliteFailure(_, Some(msg)))
            if msg.contains("UNIQUE constraint failed") =>
        {
            "标签名已存在".into()
        }
        _ => e.to_string(),
    }
}
