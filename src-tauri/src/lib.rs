//! VRAMS v2 后端入口
//!
//! 模块按"需要时再加"的原则组织：现在只有数据库和工具函数，
//! 后续扫描（scanner）、命令（commands）等模块随第一版开发逐步加入。

mod db;
mod utils;

use std::sync::Mutex;

use rusqlite::Connection;
use tauri::Manager;

/// 全局应用状态，通过 `app.manage()` 注入，command 用 `tauri::State<'_, AppState>` 访问。
///
/// `rusqlite::Connection` 不能被多个线程同时使用，所以用 `Mutex` 包起来：
/// 同一时刻只有一个 command 能拿到连接（也正好符合 SQLite 单写者的限制）。
pub struct AppState {
    pub db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // opener：用系统默认程序打开文件、在资源管理器中显示
        .plugin(tauri_plugin_opener::init())
        // dialog：选择素材库目录时的文件夹选择器
        .plugin(tauri_plugin_dialog::init())
        // 数据库初始化放在 setup 中：默认库目录依赖 AppHandle 解析应用数据目录
        .setup(|app| {
            let db_path = utils::path::get_database_path(app.handle())?;
            let conn = db::initialize_database(&db_path)
                .map_err(|e| format!("数据库初始化失败（{}）: {e}", db_path.display()))?;
            app.manage(AppState { db: Mutex::new(conn) });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
