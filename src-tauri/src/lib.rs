// 顶级模块声明：按功能域组织，后续各 Phase 在对应模块中填充实现
mod commands;
mod db;
mod download;
mod models;
mod parsers;
mod services;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Phase 4 导入功能依赖：文件选择器（dialog）+ 文件读取（fs）
        // capabilities/default.json 已声明对应权限
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        // 后续 Phase 在此注册业务命令：tauri::generate_handler![commands::asset_commands::...]
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
