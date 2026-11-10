//! 数据库层入口模块
//!
//! 负责打开 SQLite 连接，并按顺序执行尚未执行过的迁移脚本。

use std::path::Path;

use rusqlite::Connection;

/// 迁移脚本列表：下标 0 = V001，下标 1 = V002……
///
/// 新增迁移只需在末尾追加一行 `include_str!`，**已发布的迁移永远不要修改**，
/// 否则已经执行过旧版本的数据库不会再执行一次，结构就和代码对不上了。
///
/// 注意：迁移在事务中执行，`PRAGMA foreign_keys` 在事务里不生效，不要写进迁移脚本。
const MIGRATIONS: &[&str] = &[
    // 第一版数据模型写好后，在这里加：
    // include_str!("../../migrations/V001__init.sql"),
];

/// 打开数据库并执行未完成的迁移。
///
/// 流程：
/// 1. 父目录不存在时先创建
/// 2. 打开（不存在则创建）SQLite 文件
/// 3. 开启外键约束（SQLite 默认关闭，且每个连接都要单独开）
/// 4. 执行未完成的迁移
pub fn initialize_database(db_path: &Path) -> Result<Connection, String> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建数据库目录失败: {e}"))?;
    }

    let mut conn = Connection::open(db_path).map_err(|e| format!("打开数据库失败: {e}"))?;

    conn.pragma_update(None, "foreign_keys", true)
        .map_err(|e| format!("启用外键约束失败: {e}"))?;

    run_migrations(&mut conn, MIGRATIONS)?;
    Ok(conn)
}

/// 按顺序执行 `user_version` 之后的迁移，每个迁移一个事务。
///
/// `PRAGMA user_version` 是 SQLite 文件头里的一个整数，用来记录"已经执行到第几个迁移"。
/// 每个迁移和版本号更新放在同一个事务里：要么一起成功，要么一起回滚，
/// 不会出现"表建了一半、版本号却没变"的状态。
fn run_migrations(conn: &mut Connection, migrations: &[&str]) -> Result<(), String> {
    let current: i64 = conn
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|e| format!("读取 user_version 失败: {e}"))?;

    for (index, sql) in migrations.iter().enumerate().skip(current as usize) {
        let version = index as i64 + 1;
        let tx = conn
            .transaction()
            .map_err(|e| format!("开启迁移事务失败: {e}"))?;
        tx.execute_batch(sql)
            .map_err(|e| format!("执行迁移 V{version:03} 失败: {e}"))?;
        tx.pragma_update(None, "user_version", version)
            .map_err(|e| format!("更新 user_version 失败: {e}"))?;
        tx.commit()
            .map_err(|e| format!("提交迁移 V{version:03} 失败: {e}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_version(conn: &Connection) -> i64 {
        conn.pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap()
    }

    /// 初始化会创建文件、开启外键，版本号等于迁移数量
    #[test]
    fn initialize_creates_db_with_foreign_keys() {
        let dir = std::env::temp_dir().join(format!("vrams_test_init_{}", std::process::id()));
        let db_path = dir.join("vrams.db");

        let conn = initialize_database(&db_path).expect("初始化数据库失败");
        assert!(db_path.exists(), "数据库文件应被创建");

        let fk: i64 = conn
            .pragma_query_value(None, "foreign_keys", |row| row.get(0))
            .unwrap();
        assert_eq!(fk, 1, "外键约束应开启");
        assert_eq!(user_version(&conn), MIGRATIONS.len() as i64);

        drop(conn);
        std::fs::remove_dir_all(&dir).ok();
    }

    /// 迁移按顺序执行，重复执行不会重跑已完成的迁移
    #[test]
    fn migrations_run_in_order_and_only_once() {
        let mut conn = Connection::open_in_memory().unwrap();
        let migrations = [
            "CREATE TABLE a (id INTEGER PRIMARY KEY);",
            "CREATE TABLE b (id INTEGER PRIMARY KEY);",
        ];

        run_migrations(&mut conn, &migrations[..1]).unwrap();
        assert_eq!(user_version(&conn), 1);

        // 第二次带上全部迁移：只应执行 V002；若重跑 V001 会因表已存在而报错
        run_migrations(&mut conn, &migrations).unwrap();
        assert_eq!(user_version(&conn), 2);
    }

    /// 迁移失败时整体回滚：版本号不变，半成品的表也不会留下
    #[test]
    fn failed_migration_rolls_back() {
        let mut conn = Connection::open_in_memory().unwrap();
        let migrations = ["CREATE TABLE ok (id INTEGER); THIS IS NOT SQL;"];

        assert!(run_migrations(&mut conn, &migrations).is_err());
        assert_eq!(user_version(&conn), 0, "失败的迁移不应推进版本号");
        let tables: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(tables, 0, "失败的迁移不应留下半成品表");
    }
}
