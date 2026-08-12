//! 标签相关 Tauri IPC 调用封装
//!
//! 参数名转换：Rust 端参数是 snake_case（group_name），
//! 前端 invoke 传 camelCase（groupName），由 Tauri 自动对齐。
//! 返回值字段名不做转换，保持 snake_case。

import { invoke } from "@tauri-apps/api/core";

import type { CreateTagRequest, Tag, UpdateTagRequest } from "@/types/tag";

/** 查询全部标签 */
export async function getTags(): Promise<Tag[]> {
  return invoke("get_tags");
}

/** 创建标签 */
export async function createTag(req: CreateTagRequest): Promise<Tag> {
  return invoke("create_tag", { name: req.name, groupName: req.group_name });
}

/** 更新标签（部分更新） */
export async function updateTag(id: number, req: UpdateTagRequest): Promise<Tag> {
  return invoke("update_tag", { id, name: req.name, groupName: req.group_name });
}

/** 删除标签 */
export async function deleteTag(id: number): Promise<void> {
  return invoke("delete_tag", { id });
}
