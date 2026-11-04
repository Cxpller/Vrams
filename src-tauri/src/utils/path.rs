//! 路径工具函数

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use tauri::{AppHandle, Manager, Runtime};

/// 默认库目录在应用数据目录下的子目录名
const DEFAULT_LIBRARY_DIR: &str = "library";

/// 获取库目录路径(Phase 8 首次启动引导前使用)。
///
/// 优先级:
/// 1. 环境变量 `VRAMS_LIBRARY_PATH`(开发/测试可配置)
/// 2. 回退:应用数据目录下的 `library/`
///    (Windows 为 `%APPDATA%\com.cxpller.vrams\library`)
///
/// 不放在可执行文件旁边:MSI 安装到 `Program Files` 时该目录不可写,
/// 且卸载/升级会清理安装目录,导致用户资产库被一并删除。
///
/// 返回的目录可能不存在,调用方负责创建。
pub fn get_library_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {e}"))?;
    Ok(resolve_library_path(
        std::env::var_os("VRAMS_LIBRARY_PATH"),
        &app_data_dir,
    ))
}

/// 数据库文件完整路径:`{library}/vrams.db`
pub fn get_database_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    Ok(get_library_path(app)?.join("vrams.db"))
}

/// 纯函数:根据环境变量覆盖值与应用数据目录决定库目录(便于单元测试)
fn resolve_library_path(env_override: Option<OsString>, app_data_dir: &Path) -> PathBuf {
    match env_override {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => app_data_dir.join(DEFAULT_LIBRARY_DIR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_app_data_dir() {
        let app_data = PathBuf::from("C:/Users/u/AppData/Roaming/com.cxpller.vrams");
        assert_eq!(
            resolve_library_path(None, &app_data),
            app_data.join("library"),
            "未设置环境变量时应位于应用数据目录下"
        );
    }

    #[test]
    fn env_override_takes_precedence() {
        let app_data = PathBuf::from("C:/appdata");
        assert_eq!(
            resolve_library_path(Some("D:/my-library".into()), &app_data),
            PathBuf::from("D:/my-library")
        );
    }

    #[test]
    fn empty_env_override_is_ignored() {
        let app_data = PathBuf::from("C:/appdata");
        assert_eq!(
            resolve_library_path(Some(OsString::new()), &app_data),
            app_data.join("library"),
            "空环境变量视为未设置"
        );
    }
}
