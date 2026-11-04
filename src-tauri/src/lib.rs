// 顶级模块声明：按功能域组织，后续各 Phase 在对应模块中填充实现
mod commands;
mod db;
mod download;
mod models;
mod parsers;
mod services;
mod utils;

use std::sync::Mutex;

use rusqlite::Connection;
use tauri::Manager;

/// 全局应用状态,通过 `tauri::manage` 注入,command 用 `tauri::State<'_, AppState>` 访问。
///
/// `rusqlite::Connection` 不是 Send,必须用 `Mutex` 包裹才能放入 Tauri State;
/// 同时也天然保证了同一时刻只有一个 command 访问数据库(SQLite 单写者限制)。
pub struct AppState {
    pub db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Phase 4 导入功能依赖：文件选择器（dialog）+ 文件读取（fs）
        // capabilities/default.json 已声明对应权限
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        // 数据库初始化放在 setup 中:默认库目录依赖 AppHandle 解析应用数据目录
        // (Phase 8 前使用环境变量/默认路径,见 utils::path)
        .setup(|app| {
            let db_path = utils::path::get_database_path(app.handle())?;
            let conn = db::initialize_database(&db_path.to_string_lossy())
                .map_err(|e| format!("数据库初始化失败({}): {e}", db_path.display()))?;
            // 全局状态注入:command 通过 tauri::State<AppState> 访问数据库连接
            app.manage(AppState { db: Mutex::new(conn) });
            Ok(())
        })
        // 注册 IPC 命令:前端 invoke("xxx") 时在这里查找对应函数
        .invoke_handler(tauri::generate_handler![
            commands::tag_commands::get_tags,
            commands::tag_commands::create_tag,
            commands::tag_commands::update_tag,
            commands::tag_commands::delete_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
