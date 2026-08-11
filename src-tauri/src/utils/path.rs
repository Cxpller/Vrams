//! 路径工具函数

use std::path::PathBuf;

/// 获取库目录路径(Phase 8 首次启动引导前使用)。
///
/// 优先级:
/// 1. 环境变量 `VRAMS_LIBRARY_PATH`(开发/测试可配置)
/// 2. 回退:当前可执行文件所在目录下的 `vrams-library/`
///
/// 返回的目录可能不存在,调用方负责创建。
pub fn get_library_path() -> PathBuf {
    if let Some(p) = std::env::var_os("VRAMS_LIBRARY_PATH") {
        return PathBuf::from(p);
    }
    // 可执行文件目录,避免受"当前工作目录"影响
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|d| d.join("vrams-library")))
        .unwrap_or_else(|| PathBuf::from("vrams-library"))
}

/// 数据库文件完整路径:`{library}/vrams.db`
pub fn get_database_path() -> PathBuf {
    get_library_path().join("vrams.db")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_path_is_under_library() {
        let lib = get_library_path();
        let db = get_database_path();
        assert_eq!(db.parent().unwrap(), lib, "数据库应位于库目录下");
        assert_eq!(db.file_name().unwrap(), "vrams.db");
    }
}
