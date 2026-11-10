# Phase 2: 数据库与数据模型

**所属**: VRAMS MVP 开发计划
**预估工时**: 1 天
**前置依赖**: Phase 1 完成（Rust 模块结构 + crate 依赖就位）
**产出**: SQLite 数据库初始化完成，Rust 数据模型和 Repository 层可调用

---

## Task 2.1 — 编写 V001__init.sql 完整建表脚本

### 任务目标

创建初始数据库迁移脚本，定义 assets、asset_files、tags、asset_tags 四张表及全部索引。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/migrations/V001__init.sql` | 初始迁移脚本 |

### 修改内容

1. 在 `src-tauri/` 下创建 `migrations/` 目录
2. 创建 `V001__init.sql`，内容基于数据库设计.md 的完整 DDL：

```sql
-- 包含:
--   PRAGMA foreign_keys = ON;
--   CREATE TABLE IF NOT EXISTS assets (...);
--   CREATE TABLE IF NOT EXISTS asset_files (...);
--   CREATE TABLE IF NOT EXISTS tags (...);
--   CREATE TABLE IF NOT EXISTS asset_tags (...);
--   所有索引
--   PRAGMA user_version = 1;
```

注意：
- assets 表无 status 字段（硬删除策略）
- asset_files.file_name 用相对路径 `{id}/{原文件名}`
- 所有外键带 `ON DELETE CASCADE`
- 所有迁移脚本幂等（`IF NOT EXISTS`）

### 前置依赖

- 无

### 验收标准

- [ ] SQL 语法正确，可在 SQLite 命令行工具中执行通过
- [ ] 四张表和所有索引正确创建
- [ ] 重复执行不报错（幂等性验证）
- [ ] `PRAGMA user_version = 1` 正确设置

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| SQLite 不支持某些 MySQL 语法 | 建表失败 | 严格使用 SQLite 兼容语法，避免使用 `AUTO_INCREMENT`（用 `AUTOINCREMENT`） |

---

## Task 2.2 — 实现数据库初始化模块

### 任务目标

实现 `db/mod.rs`，负责打开/创建 SQLite 数据库文件、执行迁移脚本、配置 PRAGMA。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/db/mod.rs` | 数据库模块入口 |

### 修改内容

实现一个 `initialize_database(db_path: &str) -> Result<Connection, String>` 函数：

1. 使用 `rusqlite::Connection::open(db_path)` 打开数据库
2. 执行 `PRAGMA foreign_keys = ON`
3. 读取 `PRAGMA user_version`
4. 若 `user_version < 1`，读取 `V001__init.sql` 并执行
5. 返回 `Connection` 实例

需要同时创建 `V001__init.sql` 的嵌入式资源引用（`include_str!` 或将 SQL 内联）。

### 前置依赖

- Task 2.1（迁移脚本已创建）
- Phase 1 Task 1.1（rusqlite 已安装）

### 验收标准

- [ ] 调用 `initialize_database(path)` 后指定路径生成了 `vrams.db`
- [ ] 打开已有数据库不会重复执行迁移
- [ ] `PRAGMA foreign_keys = ON` 已启用
- [ ] 数据库文件路径不存在时自动创建目录

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| 并发访问 SQLite | 写入冲突 | MVP 单用户无需考虑并发 |
| 嵌入 SQL 文件路径引用 | 路径错误导致 panic | 使用 `include_str!("../migrations/V001__init.sql")` 编译时嵌入 |

---

## Task 2.3 — 定义 Rust 数据模型

### 任务目标

定义与数据库表对应的 Rust struct，带 `Serialize`/`Deserialize` derive，用于 Tauri IPC 序列化。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/models/asset.rs` | 资产模型 |
| `src-tauri/src/models/asset_file.rs` | 资产文件模型 |
| `src-tauri/src/models/tag.rs` | 标签模型 |

### 修改内容

**`models/asset.rs`**:
```rust
// Asset struct — 映射 assets 表
//   id: i64
//   name: String
//   source_url: Option<String>
//   thumbnail: Option<String>
//   notes: Option<String>
//   created_at: String (ISO 8601)
//   updated_at: String
//
// 派生: Debug, Clone, Serialize, Deserialize

// AssetListItem struct — 首页卡片列表的轻量视图
//   含 asset 基础字段 + file_count: i64 + primary_type: Option<String>

// CreateAssetRequest — 导入时的创建请求
// UpdateAssetRequest — 编辑时的更新请求（所有字段 optional）
```

**`models/asset_file.rs`**:
```rust
// AssetFile struct — 映射 asset_files 表
//   id, asset_id, file_name, file_size, is_primary, file_type, internal_info, created_at
```

**`models/tag.rs`**:
```rust
// Tag struct — 映射 tags 表
//   id, name, group_name, created_at

// CreateTagRequest
// UpdateTagRequest
```

### 前置依赖

- Task 2.1（表结构确定后模型字段才能对齐）
- Phase 1 Task 1.1（serde 已安装）

### 验收标准

- [ ] 所有 struct 编译通过
- [ ] 所有 struct 可序列化为 JSON（`#[derive(Serialize, Deserialize)]`）
- [ ] 字段类型与数据库列类型兼容（TEXT → String, INTEGER → i64, 可为空 → Option）
- [ ] 请求/响应 struct 分离（CreateXxx ≠ Xxx，避免将 id 暴露给创建请求）

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| SQLite 的 INTEGER 与 Rust i64 映射 | 溢出风险 | MVP 数据量不会超过 i64 范围 |
| `internal_info` 字段 JSON 序列化 | 双重序列化问题 | 存为 `Option<String>`，前端负责 JSON.parse |

---

## Task 2.4 — 实现资产 Repository

### 任务目标

实现 `asset_repo.rs` 中 assets 表的所有 CRUD 操作。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/db/repository/asset_repo.rs` | 资产仓储 |

### 修改内容

实现以下函数：

```rust
// insert_asset(conn, asset) -> Result<Asset, Error>
// get_asset_by_id(conn, id) -> Result<Option<Asset>, Error>
// get_asset_list(conn, limit, offset) -> Result<(Vec<AssetListItem>, i64), Error>
// search_assets(conn, keyword, limit, offset) -> Result<Vec<AssetListItem>, Error>
// update_asset(conn, id, request) -> Result<Asset, Error>
// delete_asset(conn, id) -> Result<(), Error>
```

### 前置依赖

- Task 2.2（数据库连接可用）
- Task 2.3（模型定义完毕）

### 验收标准

- [ ] 插入资产返回自增 ID
- [ ] 分页查询返回正确条目 + 总数
- [ ] 搜索按 `LIKE %keyword%` 匹配 name 和 notes
- [ ] 更新仅修改非 null 字段（部分更新）
- [ ] 删除后外键级联清理生效

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| 部分更新需动态 SQL | 实现略复杂 | 先实现全字段更新，后续优化为部分更新 |
| `LIKE` 搜索性能 | 大数据量时慢 | MVP 先用 `LIKE`，后续加 FTS5 |

---

## Task 2.5 — 实现资产文件 Repository

### 任务目标

实现 `asset_file_repo.rs` 中 asset_files 表的所有操作。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/db/repository/asset_file_repo.rs` | 资产文件仓储 |

### 修改内容

实现以下函数：

```rust
// insert_asset_files(conn, asset_id, files) -> Result<(), Error>  // 批量插入
// get_files_by_asset_id(conn, asset_id) -> Result<Vec<AssetFile>, Error>
// delete_files_by_asset_id(conn, asset_id) -> Result<(), Error>
// set_primary_file(conn, file_id, asset_id) -> Result<(), Error>  // 设置主文件
```

### 前置依赖

- Task 2.3
- Task 2.4（复用 Connection 类型）

### 验收标准

- [ ] 批量插入多条文件记录在单一事务中
- [ ] 按 asset_id 查询返回正确文件列表
- [ ] 级联删除验证（删除 asset 后关联 files 自动删除）

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| 批量插入事务未提交 | 数据丢失 | 确保 `INSERT` 在事务中包裹，出错时 `ROLLBACK` |

---

## Task 2.6 — 实现标签 Repository

### 任务目标

实现 `tag_repo.rs` 中 tags 表和 asset_tags 关联表的所有操作。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/db/repository/tag_repo.rs` | 标签仓储 |

### 修改内容

实现 tags 表操作：

```rust
// insert_tag(conn, name, group_name) -> Result<Tag, Error>
// get_all_tags(conn) -> Result<Vec<Tag>, Error>
// update_tag(conn, id, name, group_name) -> Result<Tag, Error>
// delete_tag(conn, id) -> Result<(), Error>
```

实现 asset_tags 关联操作：

```rust
// set_asset_tags(conn, asset_id, tag_ids) -> Result<(), Error>
//   → 先删旧关联，后插新关联（全量替换）
// get_tags_by_asset_id(conn, asset_id) -> Result<Vec<Tag>, Error>
// get_assets_by_tag_id(conn, tag_id) -> Result<Vec<i64>, Error>
```

### 前置依赖

- Task 2.3
- Task 2.4

### 验收标准

- [ ] 标签名唯一约束生效
- [ ] 删除标签不删除资产（ON DELETE SET NULL 或手动处理）
- [ ] `set_asset_tags` 原子操作（事务内删除+插入）
- [ ] 标签可分组查询

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| 标签删除后已有资产的标签关联未清理 | 幽灵关联 | 使用 `ON DELETE CASCADE` 或在删除 tag 时手动清理 asset_tags |

---

## Task 2.7 — 初始化 AppState 并注册到 Tauri

### 任务目标

在 `lib.rs` 中定义 `AppState` 持有数据库连接，在 Tauri Builder 中初始化。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/lib.rs` | 应用入口，注册状态和插件 |

### 修改内容

1. 在 `lib.rs` 或新建 `state.rs` 中定义：

```rust
pub struct AppState {
    pub db: Mutex<Connection>,  // rusqlite::Connection 非 Send，需 Mutex 包裹
}
```

2. 修改 `run()` 函数：

```rust
pub fn run() {
    // 获取库目录路径（Phase 8 前先用固定路径或环境变量）
    let db_path = get_library_path() + "/vrams.db";

    // 初始化数据库
    let conn = db::initialize_database(&db_path).expect("数据库初始化失败");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState { db: Mutex::new(conn) })
        .invoke_handler(tauri::generate_handler![])  // 暂时为空
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

3. 引入 `std::sync::Mutex`

### 前置依赖

- Task 2.2（数据库初始化模块）
- Phase 1 Task 1.7（tauri-plugin-dialog、tauri-plugin-fs 已安装并配置权限）

### 验收标准

- [ ] `AppState` 编译通过
- [ ] Tauri 启动时自动创建数据库文件
- [ ] 后续 command 可通过 `tauri::State<AppState>` 访问数据库
- [ ] `Mutex` 使用正确，无死锁风险

### 风险点

| 风险 | 影响 | 缓解 |
|------|------|------|
| `rusqlite::Connection` 不是 `Send` | 无法放入 Tauri State | 使用 `std::sync::Mutex<Connection>` 包裹 |
| 库目录路径未配置 | 数据库生成在错误位置 | Phase 8 实现前先写死为 `./vrams.db` 或环境变量 |
| `Mutex` 在 Tauri command 中使用不当 | 死锁 | 所有 DB 操作使用 `let conn = state.db.lock().unwrap()` 模式，避免嵌套锁 |

---

## Phase 2 完整验收 Checklist

- [ ] 2.1 V001__init.sql 建表脚本编写完毕，含四表 + 索引
- [ ] 2.2 数据库初始化模块实现，启动时自动执行迁移
- [ ] 2.3 Asset、AssetFile、Tag 三个模型定义完毕
- [ ] 2.4 AssetRepository CRUD 全部实现
- [ ] 2.5 AssetFileRepository 批量插入 + 查询实现
- [ ] 2.6 TagRepository 标签 + 关联操作全部实现
- [ ] 2.7 AppState 定义完毕，Tauri 启动时初始化
- [ ] `cargo check` 通过，无编译错误
- [ ] 可向 Phase 3 交付（标签系统全栈验证）

---

## Phase 2 Review（审查环节）

**触发时机**: 本 Phase 全部 Task 完成、验证通过后执行（标准流程第⑤步）。

**审查方式**: 调用 `review` skill（`.claude/skills/review/SKILL.md`），审查范围为 Phase 2 的全部变更。

**审查重点**:

- [ ] `V001__init.sql` 对齐 `docs/数据库设计.md`（四表 + 索引；`assets` 表无 `status` 字段）
- [ ] 迁移脚本幂等（`IF NOT EXISTS`），`PRAGMA user_version` 记录 schema 版本
- [ ] 全部 SQL 使用参数化查询，无字符串拼接
- [ ] 表/列/索引命名符合 development-standard §4.3
- [ ] `AppState` 初始化时机正确，DB 连接失败有明确处理
- [ ] `asset_repo` / `tag_repo` 单元测试覆盖增删改查

**输出**: 审查报告（结论 + 问题清单 + 验收核对表）。S1 阻断问题未清零前，不允许 git commit / 交付下一 Phase。
