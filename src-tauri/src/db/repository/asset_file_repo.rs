//! 资产文件仓储层 — 封装 `asset_files` 表的所有操作
//!
//! 批量插入在单一事务中完成,出错自动回滚,保证数据一致性。

use rusqlite::{params, Connection, Row};

use super::RepoError;
use crate::models::asset_file::AssetFile;

/// 待插入的新文件记录(批量导入时的输入)。
/// 具名结构体替代裸元组,字段语义清晰。
#[derive(Debug, Clone)]
pub struct NewAssetFile {
    pub file_name: String,
    pub file_size: Option<i64>,
    pub is_primary: i64,
    pub file_type: String,
    pub internal_info: Option<String>,
}

/// 从查询行构造 `AssetFile`
fn row_to_asset_file(row: &Row) -> rusqlite::Result<AssetFile> {
    Ok(AssetFile {
        id: row.get(0)?,
        asset_id: row.get(1)?,
        file_name: row.get(2)?,
        file_size: row.get(3)?,
        is_primary: row.get(4)?,
        file_type: row.get(5)?,
        internal_info: row.get(6)?,
        created_at: row.get(7)?,
    })
}

/// 批量插入一个资产的文件记录(单一事务,失败整体回滚)
pub fn insert_asset_files(
    conn: &mut Connection,
    asset_id: i64,
    files: &[NewAssetFile],
) -> Result<(), RepoError> {
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(
            "INSERT INTO asset_files (asset_id, file_name, file_size, is_primary, file_type, internal_info)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;
        for f in files {
            stmt.execute(params![
                asset_id,
                f.file_name,
                f.file_size,
                f.is_primary,
                f.file_type,
                f.internal_info
            ])?;
        }
    }
    // 事务 drop 前显式提交;任何一步出错会走 `?` 提前返回,tx 自动回滚
    tx.commit()?;
    Ok(())
}

/// 查询某资产的全部文件,主文件优先、按 id 升序
pub fn get_files_by_asset_id(conn: &Connection, asset_id: i64) -> Result<Vec<AssetFile>, RepoError> {
    let mut stmt = conn.prepare(
        "SELECT id, asset_id, file_name, file_size, is_primary, file_type, internal_info, created_at
         FROM asset_files WHERE asset_id = ?1
         ORDER BY is_primary DESC, id ASC",
    )?;
    let files = stmt
        .query_map(params![asset_id], row_to_asset_file)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(files)
}

/// 删除某资产的全部文件记录
pub fn delete_files_by_asset_id(conn: &Connection, asset_id: i64) -> Result<(), RepoError> {
    conn.execute("DELETE FROM asset_files WHERE asset_id = ?1", params![asset_id])?;
    Ok(())
}

/// 将某文件设为主文件:先把该资产下所有文件清零,再置目标文件为 1
pub fn set_primary_file(conn: &Connection, file_id: i64, asset_id: i64) -> Result<(), RepoError> {
    // 前置检查:目标文件必须属于该资产
    let belongs: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM asset_files WHERE id = ?1 AND asset_id = ?2)",
        params![file_id, asset_id],
        |r| r.get(0),
    )?;
    if !belongs {
        return Err(RepoError::NotFound(format!("文件 {file_id} 不属于资产 {asset_id}")));
    }

    // 同一事务内:先清空主标记,再设置目标文件
    let tx = conn.unchecked_transaction()?;
    tx.execute(
        "UPDATE asset_files SET is_primary = 0 WHERE asset_id = ?1",
        params![asset_id],
    )?;
    tx.execute(
        "UPDATE asset_files SET is_primary = 1 WHERE id = ?1",
        params![file_id],
    )?;
    tx.commit()?;
    Ok(())
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
    fn batch_insert_and_query() {
        let mut conn = test_conn();
        conn.execute("INSERT INTO assets (name) VALUES ('a')", []).unwrap();
        let asset_id = conn.last_insert_rowid();

        let files = vec![
            NewAssetFile { file_name: "1/Avatar.zip".into(), file_size: Some(100), is_primary: 1, file_type: "zip".into(), internal_info: None },
            NewAssetFile { file_name: "1/Hair.zip".into(), file_size: Some(50), is_primary: 0, file_type: "zip".into(), internal_info: None },
        ];
        insert_asset_files(&mut conn, asset_id, &files).unwrap();

        let got = get_files_by_asset_id(&conn, asset_id).unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].file_name, "1/Avatar.zip", "主文件应排最前");
        assert_eq!(got[0].is_primary, 1);
    }

    #[test]
    fn batch_insert_rolls_back_on_error() {
        let mut conn = test_conn();
        conn.execute("INSERT INTO assets (name) VALUES ('a')", []).unwrap();
        let asset_id = conn.last_insert_rowid();

        // 第三组数据 file_type 非法,违反 CHECK 约束 → 整个事务应回滚
        let files = vec![
            NewAssetFile { file_name: "1/A.zip".into(), file_size: Some(1), is_primary: 0, file_type: "zip".into(), internal_info: None },
            NewAssetFile { file_name: "1/B.zip".into(), file_size: Some(2), is_primary: 0, file_type: "zip".into(), internal_info: None },
            NewAssetFile { file_name: "1/C.zip".into(), file_size: Some(3), is_primary: 0, file_type: "exe".into(), internal_info: None },
        ];
        assert!(insert_asset_files(&mut conn, asset_id, &files).is_err());

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM asset_files", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 0, "失败后不应留下任何记录");
    }

    #[test]
    fn set_primary_switches_exclusively() {
        let mut conn = test_conn();
        conn.execute("INSERT INTO assets (name) VALUES ('a')", []).unwrap();
        let asset_id = conn.last_insert_rowid();
        insert_asset_files(
            &mut conn,
            asset_id,
            &[
                NewAssetFile { file_name: "1/A.zip".into(), file_size: Some(1), is_primary: 1, file_type: "zip".into(), internal_info: None },
                NewAssetFile { file_name: "1/B.zip".into(), file_size: Some(2), is_primary: 0, file_type: "zip".into(), internal_info: None },
            ],
        )
        .unwrap();

        // 把 B 设为主文件
        let b_id: i64 = conn
            .query_row("SELECT id FROM asset_files WHERE file_name = '1/B.zip'", [], |r| r.get(0))
            .unwrap();
        set_primary_file(&conn, b_id, asset_id).unwrap();

        let primaries: Vec<i64> = conn
            .prepare("SELECT id FROM asset_files WHERE is_primary = 1")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(primaries, vec![b_id], "同一资产只能有一个主文件");
    }

    #[test]
    fn set_primary_rejects_foreign_file() {
        let conn = test_conn();
        let err = set_primary_file(&conn, 123, 456).unwrap_err();
        assert!(matches!(err, RepoError::NotFound(_)));
    }
}
