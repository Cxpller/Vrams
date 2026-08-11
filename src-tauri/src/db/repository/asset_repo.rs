//! 资产仓储层 — 封装 `assets` 表的所有增删改查
//!
//! 全部使用参数化查询(`?` 占位符),禁止字符串拼接用户输入。
//! 首页列表使用 LEFT JOIN 聚合 `asset_files` 统计文件数与主类型。

use rusqlite::{params, Connection, Row};

use super::RepoError;
use crate::models::asset::{Asset, AssetListItem, CreateAssetRequest, UpdateAssetRequest};

/// 从查询行构造 `Asset`
fn row_to_asset(row: &Row) -> rusqlite::Result<Asset> {
    Ok(Asset {
        id: row.get(0)?,
        name: row.get(1)?,
        source_url: row.get(2)?,
        thumbnail: row.get(3)?,
        notes: row.get(4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

/// 从查询行构造 `AssetListItem`(首页卡片轻量视图)
fn row_to_list_item(row: &Row) -> rusqlite::Result<AssetListItem> {
    Ok(AssetListItem {
        id: row.get(0)?,
        name: row.get(1)?,
        thumbnail: row.get(2)?,
        created_at: row.get(3)?,
        file_count: row.get(4)?,
        primary_type: row.get(5)?,
    })
}

/// 插入资产,返回完整记录(含数据库生成的自增 id 和时间戳)
pub fn insert_asset(conn: &Connection, req: &CreateAssetRequest) -> Result<Asset, RepoError> {
    conn.execute(
        "INSERT INTO assets (name, source_url, thumbnail, notes)
         VALUES (?1, ?2, ?3, ?4)",
        params![req.name, req.source_url, req.thumbnail, req.notes],
    )?;
    let id = conn.last_insert_rowid();
    get_asset_by_id(conn, id)?.ok_or_else(|| RepoError::NotFound("插入后查询失败".into()))
}

/// 按 id 查询资产,不存在返回 `None`
pub fn get_asset_by_id(conn: &Connection, id: i64) -> Result<Option<Asset>, RepoError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, source_url, thumbnail, notes, created_at, updated_at
         FROM assets WHERE id = ?1",
    )?;
    let mut rows = stmt.query(params![id])?;
    match rows.next()? {
        Some(row) => Ok(Some(row_to_asset(row)?)),
        None => Ok(None),
    }
}

/// 分页查询首页资产列表,返回 (列表, 总数)
pub fn get_asset_list(
    conn: &Connection,
    limit: i64,
    offset: i64,
) -> Result<(Vec<AssetListItem>, i64), RepoError> {
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM assets", [], |r| r.get(0))?;

    let mut stmt = conn.prepare(
        "SELECT a.id, a.name, a.thumbnail, a.created_at,
                COUNT(af.id) AS file_count,
                MAX(CASE WHEN af.is_primary = 1 THEN af.file_type END) AS primary_type
         FROM assets a
         LEFT JOIN asset_files af ON af.asset_id = a.id
         GROUP BY a.id
         ORDER BY a.created_at DESC
         LIMIT ?1 OFFSET ?2",
    )?;
    let items = stmt
        .query_map(params![limit, offset], row_to_list_item)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok((items, total))
}

/// 按关键字搜索资产(name 或 notes 模糊匹配),分页
pub fn search_assets(
    conn: &Connection,
    keyword: &str,
    limit: i64,
    offset: i64,
) -> Result<(Vec<AssetListItem>, i64), RepoError> {
    let like = format!("%{keyword}%");
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM assets WHERE name LIKE ?1 OR notes LIKE ?1",
        params![like],
        |r| r.get(0),
    )?;

    let mut stmt = conn.prepare(
        "SELECT a.id, a.name, a.thumbnail, a.created_at,
                COUNT(af.id) AS file_count,
                MAX(CASE WHEN af.is_primary = 1 THEN af.file_type END) AS primary_type
         FROM assets a
         LEFT JOIN asset_files af ON af.asset_id = a.id
         WHERE a.name LIKE ?1 OR a.notes LIKE ?1
         GROUP BY a.id
         ORDER BY a.created_at DESC
         LIMIT ?2 OFFSET ?3",
    )?;
    let items = stmt
        .query_map(params![like, limit, offset], row_to_list_item)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok((items, total))
}

/// 部分更新资产:仅更新请求中非 `None` 的字段,并刷新 `updated_at`。
///
/// 动态拼接的是**固定列名**(代码内白名单),用户输入值一律走参数化占位符,无注入风险。
pub fn update_asset(
    conn: &Connection,
    id: i64,
    req: &UpdateAssetRequest,
) -> Result<Asset, RepoError> {
    // 前置检查:资产必须存在,否则后续 updated_at 会误改
    if get_asset_by_id(conn, id)?.is_none() {
        return Err(RepoError::NotFound(format!("资产 {id} 不存在")));
    }

    let mut sets: Vec<String> = Vec::new();
    let mut vals: Vec<rusqlite::types::Value> = Vec::new();
    if let Some(name) = &req.name {
        sets.push("name = ?".into());
        vals.push(name.clone().into());
    }
    if let Some(url) = &req.source_url {
        sets.push("source_url = ?".into());
        vals.push(url.clone().into());
    }
    if let Some(thumb) = &req.thumbnail {
        sets.push("thumbnail = ?".into());
        vals.push(thumb.clone().into());
    }
    if let Some(notes) = &req.notes {
        sets.push("notes = ?".into());
        vals.push(notes.clone().into());
    }

    if !sets.is_empty() {
        sets.push("updated_at = datetime('now', 'localtime')".into());
        let sql = format!("UPDATE assets SET {} WHERE id = ?", sets.join(", "));
        vals.push(id.into());
        conn.execute(&sql, rusqlite::params_from_iter(vals))?;
    }

    get_asset_by_id(conn, id)?.ok_or_else(|| RepoError::NotFound(format!("资产 {id} 不存在")))
}

/// 删除资产(数据库层面),磁盘文件由上层负责清理
pub fn delete_asset(conn: &Connection, id: i64) -> Result<(), RepoError> {
    let affected = conn.execute("DELETE FROM assets WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(RepoError::NotFound(format!("资产 {id} 不存在")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    /// 内存数据库:初始化迁移后作为测试连接
    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../../migrations/V001__init.sql"))
            .unwrap();
        conn
    }

    #[test]
    fn insert_and_get_by_id() {
        let conn = test_conn();
        let req = CreateAssetRequest {
            name: "Avatar-Kikyo 套装".into(),
            source_url: Some("https://booth.pm/example".into()),
            thumbnail: None,
            notes: Some("角色模型".into()),
        };
        let asset = insert_asset(&conn, &req).unwrap();
        assert!(asset.id > 0, "插入后应返回自增 id");
        assert_eq!(asset.name, "Avatar-Kikyo 套装");

        let fetched = get_asset_by_id(&conn, asset.id).unwrap().unwrap();
        assert_eq!(fetched.source_url.as_deref(), Some("https://booth.pm/example"));
        assert!(!fetched.created_at.is_empty(), "created_at 应有默认值");
    }

    #[test]
    fn get_missing_returns_none() {
        let conn = test_conn();
        assert!(get_asset_by_id(&conn, 999).unwrap().is_none());
    }

    #[test]
    fn list_is_paginated_with_total() {
        let conn = test_conn();
        for i in 0..5 {
            insert_asset(
                &conn,
                &CreateAssetRequest { name: format!("资产{i}"), source_url: None, thumbnail: None, notes: None },
            )
            .unwrap();
        }
        let (items, total) = get_asset_list(&conn, 2, 0).unwrap();
        assert_eq!(items.len(), 2, "每页 2 条");
        assert_eq!(total, 5, "总数应为 5");

        let (page2, _) = get_asset_list(&conn, 2, 2).unwrap();
        assert_eq!(page2.len(), 2);
    }

    #[test]
    fn search_matches_name_and_notes() {
        let conn = test_conn();
        insert_asset(&conn, &CreateAssetRequest { name: "Kikyo 发型".into(), source_url: None, thumbnail: None, notes: None }).unwrap();
        insert_asset(&conn, &CreateAssetRequest { name: "普通衣服".into(), source_url: None, thumbnail: None, notes: Some("Kikyo 专用".into()) }).unwrap();
        insert_asset(&conn, &CreateAssetRequest { name: "鞋".into(), source_url: None, thumbnail: None, notes: None }).unwrap();

        let (items, total) = search_assets(&conn, "kikyo", 50, 0).unwrap();
        assert_eq!(total, 2, "name 和 notes 都命中");
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn update_only_changes_non_null_fields() {
        let conn = test_conn();
        let asset = insert_asset(&conn, &CreateAssetRequest { name: "原名".into(), source_url: None, thumbnail: None, notes: None }).unwrap();

        let updated = update_asset(&conn, asset.id, &UpdateAssetRequest {
            name: Some("新名".into()),
            source_url: None, // 不更新,应保留 None
            thumbnail: None,
            notes: Some("加了备注".into()),
        })
        .unwrap();
        assert_eq!(updated.name, "新名");
        assert_eq!(updated.notes.as_deref(), Some("加了备注"));
        assert_eq!(updated.source_url, None, "未提供的字段不应被清空");
        assert!(updated.updated_at >= asset.updated_at, "updated_at 应被刷新");
    }

    #[test]
    fn update_missing_returns_not_found() {
        let conn = test_conn();
        let err = update_asset(&conn, 999, &UpdateAssetRequest { name: Some("x".into()), source_url: None, thumbnail: None, notes: None }).unwrap_err();
        assert!(matches!(err, RepoError::NotFound(_)));
    }

    #[test]
    fn delete_cascades_to_files_and_tags() {
        let conn = test_conn();
        let asset = insert_asset(&conn, &CreateAssetRequest { name: "待删".into(), source_url: None, thumbnail: None, notes: None }).unwrap();

        // 关联文件 + 标签(级联验证)
        conn.execute("INSERT INTO asset_files (asset_id, file_name) VALUES (?1, ?2)", params![asset.id, "1/x.zip"]).unwrap();
        conn.execute("INSERT INTO tags (name) VALUES ('t')", []).unwrap();
        conn.execute("INSERT INTO asset_tags (asset_id, tag_id) VALUES (?1, 1)", params![asset.id]).unwrap();

        delete_asset(&conn, asset.id).unwrap();
        assert!(get_asset_by_id(&conn, asset.id).unwrap().is_none());
        let files: i64 = conn.query_row("SELECT COUNT(*) FROM asset_files", [], |r| r.get(0)).unwrap();
        assert_eq!(files, 0, "级联删除文件");
        let rels: i64 = conn.query_row("SELECT COUNT(*) FROM asset_tags", [], |r| r.get(0)).unwrap();
        assert_eq!(rels, 0, "级联删除标签关联");
    }

    #[test]
    fn delete_missing_returns_not_found() {
        let conn = test_conn();
        let err = delete_asset(&conn, 999).unwrap_err();
        assert!(matches!(err, RepoError::NotFound(_)));
    }
}
