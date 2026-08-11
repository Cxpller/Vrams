//! 标签仓储层 — 封装 `tags` 表与 `asset_tags` 关联表的操作

use rusqlite::{params, Connection, Row};

use super::RepoError;
use crate::models::tag::{CreateTagRequest, Tag, UpdateTagRequest};

/// 从查询行构造 `Tag`
fn row_to_tag(row: &Row) -> rusqlite::Result<Tag> {
    Ok(Tag {
        id: row.get(0)?,
        name: row.get(1)?,
        group_name: row.get(2)?,
        created_at: row.get(3)?,
    })
}

/// 插入标签;name 有 UNIQUE 约束,重复会返回 Db 错误
pub fn insert_tag(conn: &Connection, req: &CreateTagRequest) -> Result<Tag, RepoError> {
    conn.execute(
        "INSERT INTO tags (name, group_name) VALUES (?1, ?2)",
        params![req.name, req.group_name],
    )?;
    let id = conn.last_insert_rowid();
    get_tag_by_id(conn, id)?.ok_or_else(|| RepoError::NotFound("插入后查询失败".into()))
}

/// 按 id 查询标签
pub fn get_tag_by_id(conn: &Connection, id: i64) -> Result<Option<Tag>, RepoError> {
    let mut stmt = conn.prepare("SELECT id, name, group_name, created_at FROM tags WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;
    match rows.next()? {
        Some(row) => Ok(Some(row_to_tag(row)?)),
        None => Ok(None),
    }
}

/// 查询全部标签,按分组名 + 名称排序
pub fn get_all_tags(conn: &Connection) -> Result<Vec<Tag>, RepoError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, group_name, created_at FROM tags ORDER BY group_name, name",
    )?;
    let tags = stmt
        .query_map([], row_to_tag)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(tags)
}

/// 更新标签(部分更新),返回更新后的完整记录
pub fn update_tag(conn: &Connection, id: i64, req: &UpdateTagRequest) -> Result<Tag, RepoError> {
    if get_tag_by_id(conn, id)?.is_none() {
        return Err(RepoError::NotFound(format!("标签 {id} 不存在")));
    }

    let mut sets: Vec<String> = Vec::new();
    let mut vals: Vec<rusqlite::types::Value> = Vec::new();
    if let Some(name) = &req.name {
        sets.push("name = ?".into());
        vals.push(name.clone().into());
    }
    if let Some(group) = &req.group_name {
        sets.push("group_name = ?".into());
        vals.push(group.clone().into());
    }
    if !sets.is_empty() {
        let sql = format!("UPDATE tags SET {} WHERE id = ?", sets.join(", "));
        vals.push(id.into());
        conn.execute(&sql, rusqlite::params_from_iter(vals))?;
    }

    get_tag_by_id(conn, id)?.ok_or_else(|| RepoError::NotFound(format!("标签 {id} 不存在")))
}

/// 删除标签;asset_tags 关联由 ON DELETE CASCADE 自动清理
pub fn delete_tag(conn: &Connection, id: i64) -> Result<(), RepoError> {
    let affected = conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(RepoError::NotFound(format!("标签 {id} 不存在")));
    }
    Ok(())
}

/// 全量替换某资产的标签关联(事务内:先删旧,后插新),保证原子性
pub fn set_asset_tags(conn: &Connection, asset_id: i64, tag_ids: &[i64]) -> Result<(), RepoError> {
    let tx = conn.unchecked_transaction()?;
    tx.execute("DELETE FROM asset_tags WHERE asset_id = ?1", params![asset_id])?;
    {
        let mut stmt =
            tx.prepare("INSERT INTO asset_tags (asset_id, tag_id) VALUES (?1, ?2)")?;
        for &tag_id in tag_ids {
            stmt.execute(params![asset_id, tag_id])?;
        }
    }
    tx.commit()?;
    Ok(())
}

/// 查询某资产关联的全部标签
pub fn get_tags_by_asset_id(conn: &Connection, asset_id: i64) -> Result<Vec<Tag>, RepoError> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, t.group_name, t.created_at
         FROM tags t
         JOIN asset_tags at ON at.tag_id = t.id
         WHERE at.asset_id = ?1
         ORDER BY t.group_name, t.name",
    )?;
    let tags = stmt
        .query_map(params![asset_id], row_to_tag)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(tags)
}

/// 查询某标签下的全部资产 id
pub fn get_assets_by_tag_id(conn: &Connection, tag_id: i64) -> Result<Vec<i64>, RepoError> {
    let mut stmt = conn.prepare("SELECT asset_id FROM asset_tags WHERE tag_id = ?1 ORDER BY asset_id")?;
    let ids = stmt
        .query_map(params![tag_id], |r| r.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../../migrations/V001__init.sql"))
            .unwrap();
        conn
    }

    #[test]
    fn insert_and_get_all() {
        let conn = test_conn();
        insert_tag(&conn, &CreateTagRequest { name: "Kikyo".into(), group_name: "角色".into() }).unwrap();
        insert_tag(&conn, &CreateTagRequest { name: "衣服".into(), group_name: "类型".into() }).unwrap();

        let tags = get_all_tags(&conn).unwrap();
        assert_eq!(tags.len(), 2);
    }

    #[test]
    fn name_unique_constraint() {
        let conn = test_conn();
        insert_tag(&conn, &CreateTagRequest { name: "已购买".into(), group_name: "状态".into() }).unwrap();
        let err = insert_tag(&conn, &CreateTagRequest { name: "已购买".into(), group_name: "".into() }).unwrap_err();
        assert!(matches!(err, RepoError::Db(_)), "重复标签名应触发 UNIQUE 约束");
    }

    #[test]
    fn update_tag_partial() {
        let conn = test_conn();
        let tag = insert_tag(&conn, &CreateTagRequest { name: "旧名".into(), group_name: "旧组".into() }).unwrap();
        let updated = update_tag(&conn, tag.id, &UpdateTagRequest { name: Some("新名".into()), group_name: None }).unwrap();
        assert_eq!(updated.name, "新名");
        assert_eq!(updated.group_name, "旧组", "未提供的字段应保留");
    }

    #[test]
    fn delete_tag_cleans_relations() {
        let conn = test_conn();
        conn.execute("INSERT INTO assets (name) VALUES ('a')", []).unwrap();
        let asset_id = conn.last_insert_rowid();
        let tag = insert_tag(&conn, &CreateTagRequest { name: "t".into(), group_name: "".into() }).unwrap();

        set_asset_tags(&conn, asset_id, &[tag.id]).unwrap();
        assert_eq!(get_tags_by_asset_id(&conn, asset_id).unwrap().len(), 1);

        delete_tag(&conn, tag.id).unwrap();
        let rels: i64 = conn.query_row("SELECT COUNT(*) FROM asset_tags", [], |r| r.get(0)).unwrap();
        assert_eq!(rels, 0, "删除标签应级联清理关联");
        // 资产不应被删除
        let assets: i64 = conn.query_row("SELECT COUNT(*) FROM assets", [], |r| r.get(0)).unwrap();
        assert_eq!(assets, 1, "删除标签不删除资产");
    }

    #[test]
    fn set_asset_tags_is_replace_all() {
        let conn = test_conn();
        conn.execute("INSERT INTO assets (name) VALUES ('a')", []).unwrap();
        let asset_id = conn.last_insert_rowid();

        let t1 = insert_tag(&conn, &CreateTagRequest { name: "t1".into(), group_name: "".into() }).unwrap();
        let t2 = insert_tag(&conn, &CreateTagRequest { name: "t2".into(), group_name: "".into() }).unwrap();

        set_asset_tags(&conn, asset_id, &[t1.id]).unwrap();
        set_asset_tags(&conn, asset_id, &[t2.id]).unwrap();

        let tags = get_tags_by_asset_id(&conn, asset_id).unwrap();
        assert_eq!(tags.len(), 1, "第二次设置应替换而非追加");
        assert_eq!(tags[0].id, t2.id);
    }

    #[test]
    fn get_assets_by_tag() {
        let conn = test_conn();
        conn.execute("INSERT INTO assets (name) VALUES ('a'), ('b')", []).unwrap();
        let id_a = 1_i64;
        let id_b = 2_i64;
        let tag = insert_tag(&conn, &CreateTagRequest { name: "x".into(), group_name: "".into() }).unwrap();

        set_asset_tags(&conn, id_a, &[tag.id]).unwrap();
        set_asset_tags(&conn, id_b, &[tag.id]).unwrap();
        assert_eq!(get_assets_by_tag_id(&conn, tag.id).unwrap(), vec![1, 2]);
    }
}
