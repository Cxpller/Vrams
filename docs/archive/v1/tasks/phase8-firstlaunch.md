# Phase 8: 首次启动体验

**所属**: VRAMS MVP 开发计划
**预估工时**: 0.5 天
**前置依赖**: Phase 5（首页可用）+ Phase 7（可上传封面）
**产出**: 首次启动引导用户选择库目录，后续直接进入首页

---

## Task 8.1 — 实现配置持久化命令

### 任务目标

读取/写入应用配置（库目录路径、自动下载开关等）。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src-tauri/src/commands/system_commands.rs` | 系统配置命令 |
| `src-tauri/src/utils/config.rs` | 配置管理工具 |

### 修改内容

**`utils/config.rs`**:
```rust
// AppConfig struct:
//   library_path: String
//   auto_fetch_thumbnail: bool (默认 true)
//   delete_originals: bool (默认 true)
//
// save_config(config: &AppConfig) -> Result<(), String>
//   → 以 JSON 格式保存到 {library_path}/vrams_config.json
//
// load_config() -> Result<Option<AppConfig>, String>
//   → 从默认路径加载配置
```

**`commands/system_commands.rs`**:
```rust
#[tauri::command]
fn get_config() -> Result<AppConfig, String>

#[tauri::command]
fn update_config(state: State<AppState>, config: AppConfig) -> Result<(), String>
```

### 前置依赖

- 无

### 验收标准

- [ ] 配置可保存到本地文件
- [ ] 配置修改后重新读取得到新值
- [ ] 配置文件损坏时返回默认配置
- [ ] 初次使用时可检测到"未配置"

---

## Task 8.2 — 实现首次启动检测

### 任务目标

应用启动时检测是否为首次启动，跳转到引导流程或首页。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/App.vue` | 根组件，启动逻辑入口 |

### 修改内容

在 `App.vue` 的 `onMounted` 中：

```typescript
// 1. 调用 getConfig()
// 2. 如果 library_path 为空或不存在 → 进入首次启动引导
// 3. 如果 library_path 已配置 → 正常进入首页
```

### 前置依赖

- Task 8.1

### 验收标准

- [ ] 无配置时进入引导流程
- [ ] 有配置时直接进入首页

---

## Task 8.3 — 实现库目录选择引导

### 任务目标

首次启动时弹出引导对话框，让用户选择库目录。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/components/business/FirstLaunchWizard.vue` | 首次启动向导 |

### 修改内容

向导步骤：
1. 欢迎页：介绍 VRAMS
2. 选择库目录：使用 `tauri-plugin-dialog` 的文件夹选择器
   - 默认为 `{Documents}/VRAMS-Library/`
   - 用户可自定义
   - 校验：目标目录磁盘空间 ≥ 1GB、路径无特殊字符
3. 确认页：显示选择的路径，点"开始使用"完成

完成后：
1. 创建库目录下的 `assets/` `thumbnails/` 子目录
2. 调用 `initialize_database` 创建数据库
3. 保存配置
4. 跳转到首页

### 前置依赖

- Phase 2（数据库初始化）
- Phase 1（dialog 插件）

### 验收标准

- [ ] 首次启动弹出向导
- [ ] 用户可选择自定义目录
- [ ] 校验目标目录可写
- [ ] 选择后自动创建子目录并初始化数据库
- [ ] 向导完成后进入首页
- [ ] 用户取消选择时应用退出

---

## Task 8.4 — 集成引导流程到 App.vue

### 任务目标

将首次启动引导整合到应用启动流程中。

### 涉及文件

| 文件 | 角色 |
|------|------|
| `src/App.vue` | 根组件 |

### 修改内容

```typescript
// App.vue 启动逻辑:
const config = ref<AppConfig | null>(null);
const isFirstLaunch = ref(true);

onMounted(async () => {
  const cfg = await invoke("get_config");
  if (cfg.library_path && exists(cfg.library_path)) {
    isFirstLaunch.value = false;
    config.value = cfg;
  } else {
    isFirstLaunch.value = true;
  }
});
```

根据 `isFirstLaunch` 条件渲染 FirstLaunchWizard 或 RouterView。

### 前置依赖

- Task 8.2, 8.3

### 验收标准

- [ ] 首次启动 → 向导 → 选目录 → 进首页
- [ ] 二次启动 → 直接进首页
- [ ] 首页能正常读取已配置的库目录

---

## Phase 8 完整验收 Checklist

- [ ] 8.1 get_config / update_config 命令可用
- [ ] 8.2 首次启动检测逻辑实现
- [ ] 8.3 FirstLaunchWizard 向导（选择目录 → 初始化 → 完成）
- [ ] 8.4 引导流程集成到 App.vue
- [ ] 全流程：首次启动 → 引导 → 首页；二次启动 → 首页

---

## Phase 8 Review（审查环节）

**触发时机**: 本 Phase 全部 Task 完成、验证通过后执行（标准流程第⑤步）。

**审查方式**: 调用 `review` skill（`.claude/skills/review/SKILL.md`），审查范围为 Phase 8 的全部变更。

**审查重点**:

- [ ] 配置读写（JSON 文件）写入原子性、损坏容错
- [ ] 库目录权限检测，`assets/` `thumbnails/` 子目录正确创建
- [ ] 用户取消目录选择 → 应用退出逻辑
- [ ] 二次启动跳过引导（配置持久化生效）
- [ ] 数据库初始化幂等（重复启动不报错）

**输出**: 审查报告（结论 + 问题清单 + 验收核对表）。S1 阻断问题未清零前，不允许 git commit / 交付下一 Phase。
