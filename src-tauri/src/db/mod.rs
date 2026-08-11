//! 数据库层入口模块
//!
//! 负责 SQLite 连接管理、schema 迁移（migrations）与仓储层（repository）。

pub mod repository;

use rusqlite::Connection;

/// 初始化数据库连接并执行未完成的迁移。
///
/// 流程:
/// 1. 打开(不存在则创建)SQLite 数据库文件
/// 2. 启用外键约束 `PRAGMA foreign_keys = ON`
/// 3. 读取 `PRAGMA user_version` 当前 schema 版本
/// 4. 若版本 < 1,执行 `V001__init.sql` 完成初始建表
///
/// 迁移脚本通过 `include_str!` 编译期嵌入,运行时无需访问外部文件。
pub fn initialize_database(db_path: &str) -> Result<Connection, String> {
    // 目录不存在时先创建,避免 open() 因父目录缺失而失败
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建数据库目录失败: {e}"))?;
    }

    let conn = Connection::open(db_path).map_err(|e| format!("打开数据库失败: {e}"))?;

    // 外键约束必须在每次连接时显式开启(SQLite 默认关闭)
    conn.pragma_update(None, "foreign_keys", true)
        .map_err(|e| format!("启用外键约束失败: {e}"))?;

    // 读取当前 schema 版本
    let user_version: i64 = conn
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|e| format!("读取 user_version 失败: {e}"))?;

    if user_version < 1 {
        // include_str! 在编译期把 SQL 文件内容嵌入二进制
        let sql = include_str!("../../migrations/V001__init.sql");
        conn.execute_batch(sql)
            .map_err(|e| format!("执行迁移 V001__init.sql 失败: {e}"))?;
    }

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 初始化后:四张表存在、索引存在、user_version = 1、外键已开启
    #[test]
    fn initialize_creates_schema() {
        let db_path =
            std::env::temp_dir().join(format!("vrams_test_init_{}.db", std::process::id()));
        let db_path_str = db_path.to_str().unwrap();

        let conn = initialize_database(db_path_str).expect("初始化数据库失败");

        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(
            tables,
            vec!["asset_files", "asset_tags", "assets", "tags"],
            "四张业务表应全部创建"
        );

        // 索引数量验证(6 个业务索引 + 隐含索引)
        let index_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(index_count, 6, "应有 6 个自定义索引");

        let user_version: i64 = conn
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        assert_eq!(user_version, 1, "schema 版本应为 1");

        let fk: i64 = conn
            .pragma_query_value(None, "foreign_keys", |row| row.get(0))
            .unwrap();
        assert_eq!(fk, 1, "外键约束应开启");

        std::fs::remove_file(&db_path).ok();
    }

    /// 重复初始化(模拟二次启动)不应报错,且不重复建表
    #[test]
    fn initialize_is_idempotent() {
        let db_path =
            std::env::temp_dir().join(format!("vrams_test_idem_{}.db", std::process::id()));
        let db_path_str = db_path.to_str().unwrap();

        initialize_database(db_path_str).expect("首次初始化失败");
        let conn2 = initialize_database(db_path_str).expect("二次初始化应幂等成功");

        // 版本仍为 1,且表可正常查询
        let user_version: i64 = conn2
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        assert_eq!(user_version, 1);
        let count: i64 = conn2
            .query_row("SELECT COUNT(*) FROM assets", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0, "重复迁移不应产生重复数据");

        std::fs::remove_file(&db_path).ok();
    }
}
