//! 压缩包解析入口模块
//!
//! 只读解析包体（不落盘、不缓存），提取文件清单与元数据。

pub mod sevenz_archive;
pub mod unitypackage;
pub mod zip_archive;
