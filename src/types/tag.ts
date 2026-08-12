//! 标签类型定义
//! 与 src-tauri/src/models/tag.rs 对齐：字段名必须与 Rust 端 serde 序列化的 JSON 严格一致。
//! 注意：返回结构体的字段名不会被 Tauri 转换，保持 snake_case 原样。

/** 标签实体（对应数据库 tags 表） */
export interface Tag {
  id: number;
  name: string;
  group_name: string;
  created_at: string;
}

/** 创建标签请求 */
export interface CreateTagRequest {
  name: string;
  group_name?: string;
}

/** 更新标签请求 — 所有字段可选，未传的字段保持不变 */
export interface UpdateTagRequest {
  name?: string;
  group_name?: string;
}
