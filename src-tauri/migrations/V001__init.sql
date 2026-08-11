-- V001__init.sql
-- 初始数据库 schema
-- 版本: user_version = 1
-- 幂等: 全部使用 IF NOT EXISTS,可重复执行

PRAGMA foreign_keys = ON;

-- assets: 一个用户可见的"资产"(商品)。无 status 字段(硬删除策略)
CREATE TABLE IF NOT EXISTS assets (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    source_url  TEXT,
    thumbnail   TEXT,
    notes       TEXT,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- asset_files: 资产关联的导入文件(可多个,合并场景)
-- file_name 存相对路径 {asset_id}/{原始文件名},磁盘位置 {library}/assets/{file_name}
CREATE TABLE IF NOT EXISTS asset_files (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id      INTEGER NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    file_name     TEXT    NOT NULL,
    file_size     INTEGER,
    is_primary    INTEGER NOT NULL DEFAULT 0
                          CHECK (is_primary IN (0, 1)),
    file_type     TEXT    NOT NULL DEFAULT 'zip'
                          CHECK (file_type IN ('zip', '7z', 'unitypackage')),
    internal_info TEXT,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- tags: 标签,name 唯一
CREATE TABLE IF NOT EXISTS tags (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL UNIQUE,
    group_name TEXT    NOT NULL DEFAULT '',
    created_at TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- asset_tags: 资产-标签多对多关联表,联合主键防重复
CREATE TABLE IF NOT EXISTS asset_tags (
    asset_id INTEGER NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    tag_id   INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (asset_id, tag_id)
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_assets_name       ON assets(name);
CREATE INDEX IF NOT EXISTS idx_assets_created_at ON assets(created_at);
CREATE INDEX IF NOT EXISTS idx_asset_files_asset ON asset_files(asset_id);
CREATE INDEX IF NOT EXISTS idx_tags_group        ON tags(group_name);
CREATE INDEX IF NOT EXISTS idx_tags_name         ON tags(name);
CREATE INDEX IF NOT EXISTS idx_asset_tags_tag    ON asset_tags(tag_id);

PRAGMA user_version = 1;
