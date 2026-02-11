# Teammate - TODO CLI 工具实现任务文档

本文档详细描述了 Teammate TODO CLI 工具的完整实现计划，基于前期探索任务的设计方案整理而成。

---

## 目录

1. [项目概述](#1-项目概述)
2. [目录结构](#2-目录结构)
3. [核心模块实现任务](#3-核心模块实现任务)
   - 3.1 [CLI 命令系统](#31-cli-命令系统)
   - 3.2 [存储层实现](#32-存储层实现)
   - 3.3 [多语言解析器](#33-多语言解析器)
   - 3.4 [Git 集成](#34-git-集成)
   - 3.5 [TUI 界面](#35-tui-界面)
   - 3.6 [测试策略](#36-测试策略)
4. [数据模型详细定义](#4-数据模型详细定义)
5. [API 接口规范](#5-api-接口规范)
6. [配置文件格式](#6-配置文件格式)
7. [性能优化要求](#7-性能优化要求)
8. [实施路线图](#8-实施路线图)
9. [验收标准](#9-验收标准)

---

## 1. 项目概述

### 1.1 项目目标

Teammate 是一个专为开发者设计的 TODO CLI 工具，用于从代码库中自动发现、跟踪和管理 TODO 注释。

**核心功能**：
- 自动扫描代码库中的 TODO 注释
- 支持 30+ 种编程语言
- 标签系统和优先级管理
- Git 深度集成（blame、log、branch）
- 现代化 TUI 界面
- 跨平台支持（macOS、Linux、Windows）

### 1.2 技术栈

| 层级 | 技术选型 | 理由 |
|------|---------|------|
| 核心语言 | Rust | 高性能、内存安全、跨平台 |
| CLI 框架 | Clap | 成熟的 CLI 生态、类型安全 |
| 存储 | SQLite | 嵌入式、高性能查询 |
| 并行处理 | Rayon | 简洁的并行迭代 |
| TUI | Ratatui | 声明式 UI、丰富组件 |
| Git 操作 | git2-rs | 安全绑定 libgit2 |
| 序列化 | Serde | 成熟的序列化框架 |
| 异步处理 | Tokio | 异步运行时 |

### 1.3 版本规划

| 版本 | 目标 | 预计工时 |
|------|------|---------|
| v0.1 | MVP：基础扫描、存储、CLI | 2 周 |
| v0.2 | 完善解析器、添加 Git 集成 | 2 周 |
| v0.3 | TUI 界面、测试覆盖 | 2 周 |
| v1.0 | 插件系统、性能优化 | 3 周 |

---

## 2. 目录结构

### 2.1 整体结构

```
teammate/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── DESIGN.md
├── TASK.md                    # 本文档
├── src/
│   ├── main.rs               # 程序入口
│   ├── lib.rs                # 库入口
│   ├── bin/
│   │   └── teammate/
│   │       └── main.rs       # CLI 二进制入口
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── scan.rs      # scan 命令
│   │   │   ├── list.rs      # list 命令
│   │   │   ├── add.rs       # add 命令
│   │   │   ├── remove.rs    # remove 命令
│   │   │   ├── update.rs    # update 命令
│   │   │   ├── status.rs    # status 命令
│   │   │   ├── tag.rs       # tag 命令
│   │   │   ├── config.rs    # config 命令
│   │   │   ├── init.rs      # init 命令
│   │   │   ├── stats.rs     # stats 命令
│   │   │   ├── blame.rs     # blame 命令
│   │   │   ├── branch.rs    # branch 命令
│   │   │   ├── sync.rs      # sync 命令
│   │   │   ├── hooks.rs     # hooks 命令
│   │   │   └── help.rs      # help 命令
│   │   ├── args.rs          # 参数解析
│   │   ├── output.rs        # 输出格式化
│   │   └── error.rs         # 错误处理
│   ├── core/
│   │   ├── mod.rs
│   │   ├── config/
│   │   │   ├── mod.rs
│   │   │   ├── app_config.rs
│   │   │   └── hooks_config.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── todo.rs
│   │   │   ├── tag.rs
│   │   │   ├── project.rs
│   │   │   └── filter.rs
│   │   └── state/
│   │       ├── mod.rs
│   │       └── runtime_state.rs
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── database.rs      # SQLite 数据库封装
│   │   ├── migrations/
│   │   │   ├── mod.rs
│   │   │   ├── v1_initial.rs
│   │   │   ├── v2_scan_sessions.rs
│   │   │   └── v3_sync_tables.rs
│   │   ├── queries/
│   │   │   ├── mod.rs
│   │   │   ├── todo_queries.rs
│   │   │   └── tag_queries.rs
│   │   └── cache/
│   │       ├── mod.rs
│   │       ├── memory_cache.rs
│   │       └── disk_cache.rs
│   ├── parsers/
│   │   ├── mod.rs
│   │   ├── traits.rs        # Parser trait
│   │   ├── registry.rs      # Parser registry
│   │   ├── result.rs        # ParsedTodo 结构
│   │   ├── language.rs      # Language detection
│   │   ├── common/
│   │   │   ├── patterns.rs  # 标准模式定义
│   │   │   └── tag_extractor.rs
│   │   ├── javascript.rs    # JS/TS 解析器
│   │   ├── python.rs
│   │   ├── rust.rs
│   │   ├── go.rs
│   │   ├── java.rs
│   │   ├── c_family.rs      # C/C++/Obj-C
│   │   ├── ruby.rs
│   │   ├── php.rs
│   │   ├── shell.rs
│   │   ├── markdown.rs
│   │   ├── html.rs
│   │   ├── yaml.rs
│   │   ├── custom.rs        # 自定义解析器
│   │   └── incremental.rs   # 增量解析
│   ├── scan/
│   │   ├── mod.rs
│   │   ├── walker.rs        # 文件遍历
│   │   ├── ignore.rs        # 忽略规则
│   │   ├── progress.rs      # 进度显示
│   │   └── worker.rs        # 并行工作池
│   ├── git/
│   │   ├── mod.rs
│   │   ├── blame.rs         # Git blame 集成
│   │   ├── log.rs          # Git log 集成
│   │   ├── branch.rs        # 分支 TODO 追踪
│   │   ├── diff.rs          # 变更检测
│   │   ├── hooks/
│   │   │   ├── mod.rs
│   │   │   ├── installer.rs
│   │   │   ├── pre_commit.rs
│   │   │   ├── post_commit.rs
│   │   │   └── post_checkout.rs
│   │   ├── cache.rs         # Git 数据缓存
│   │   └── conflict.rs      # 冲突处理
│   ├── tui/
│   │   ├── mod.rs
│   │   ├── app.rs           # 主应用状态
│   │   ├── components/
│   │   │   ├── mod.rs
│   │   │   ├── list.rs      # TODO 列表组件
│   │   │   ├── detail.rs    # 详情面板
│   │   │   ├── filter.rs    # 过滤器面板
│   │   │   ├── stats.rs     # 统计面板
│   │   │   ├── help.rs      # 帮助面板
│   │   │   └── input.rs     # 输入表单
│   │   ├── events.rs        # 事件处理
│   │   ├── keymap.rs        # 快捷键绑定
│   │   └── theme.rs         # 颜色主题
│   ├── sync/
│   │   ├── mod.rs
│   │   ├── provider/
│   │   │   ├── mod.rs
│   │   │   ├── local.rs
│   │   │   ├── dropbox.rs
│   │   │   └── github.rs
│   │   ├── conflict.rs      # 同步冲突解决
│   │   └── protocol.rs      # 同步协议
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── time.rs          # 时间工具
│   │   ├── path.rs          # 路径工具
│   │   ├── version.rs       # 版本比较
│   │   └── fmt.rs           # 格式化工具
│   └── macros.rs            # 宏定义
├── tests/
│   ├── unit/
│   │   ├── mod.rs
│   │   ├── parsers/
│   │   │   └── mod.rs
│   │   ├── storage/
│   │   │   └── mod.rs
│   │   └── models/
│   │       └── mod.rs
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── scan_workflow.rs
│   │   ├── cli_commands.rs
│   │   └── git_integration.rs
│   ├── fixtures/
│   │   ├── sample_code/
│   │   │   ├── rust/
│   │   │   ├── python/
│   │   │   └── markdown/
│   │   ├── sample_repos/
│   │   │   ├── clean_repo/
│   │   │   └── todo_repo/
│   │   └── expected/
│   │       └── scan_output/
├── benchmarks/
│   ├── lib.rs
│   └── scan_benchmark.rs
├── scripts/
│   ├── fmt.sh
│   ├── clippy.sh
│   └── test.sh
├── .github/
│   ├── workflows/
│   │   ├── ci.yml
│   │   └── release.yml
│   └── scripts/
│       └── setup.sh
├── resources/
│   ├── schema.sql
│   ├── default_config.yaml
│   └── migrations/
├── docs/
│   ├── architecture.md
│   ├── cli_usage.md
│   └── development.md
├── .cargo/
│   ├── config.toml
│   └── config
├── rust-toolchain.toml
└── .gitignore
```

### 2.2 关键文件实现规格

#### `src/main.rs`

```rust
// 程序入口
fn main() {
    // 1. 初始化日志
    // 2. 加载配置
    // 3. 解析命令行参数
    // 4. 执行命令
    // 5. 处理错误
}
```

#### `src/cli/mod.rs`

```rust
// CLI 模块入口
pub struct Cli {
    pub commands: Commands,
    pub global_opts: GlobalOptions,
}

#[derive(Subcommand)]
enum Commands {
    Scan(ScanCommand),
    List(ListCommand),
    Add(AddCommand),
    Remove(RemoveCommand),
    Update(UpdateCommand),
    Status(StatusCommand),
    Tag(TagCommand),
    Config(ConfigCommand),
    Init(InitCommand),
    Stats(StatsCommand),
    Blame(BlameCommand),
    Branch(BranchCommand),
    Sync(SyncCommand),
    Hooks(HooksCommand),
}
```

#### `src/core/models/todo.rs`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,                    // UUID v4
    pub project_id: Option<String>,    // 项目 ID（可选）
    pub content: String,                // TODO 内容
    pub description: Option<String>,    // 详细描述

    // 位置信息
    pub file_path: PathBuf,            // 文件路径
    pub line_number: u32,              // 行号
    pub end_line: Option<u32>,         // 结束行（多行 TODO）

    // 分类信息
    pub language: Option<String>,      // 编程语言
    pub code_context: Option<String>,  // 上下文代码

    // 状态管理
    pub status: TodoStatus,
    pub priority: Priority,
    pub due_date: Option<DateTime<Utc>>,

    // 元数据
    pub author: Option<String>,        // 作者
    pub assignee: Option<String>,       // 负责人
    pub linked_issues: Vec<String>,   // 关联 Issue
    pub linked_commits: Vec<String>,   // 关联 Commit

    // 时间戳
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub last_scanned_at: Option<DateTime<Utc>>,

    // 扩展属性
    pub is_manual: bool,              // 是否手动创建
    pub is_ignored: bool,             // 是否已忽略
    pub ignore_reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "archived")]
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    #[serde(rename = "urgent")]
    Urgent = 5,
    #[serde(rename = "high")]
    High = 4,
    #[serde(rename = "medium")]
    Medium = 3,
    #[serde(rename = "low")]
    Low = 2,
    #[serde(rename = "none")]
    None = 1,
}
```

---

## 3. 核心模块实现任务

### 3.1 CLI 命令系统

#### 任务 3.1.1：实现 CLI 框架

**文件**: `src/cli/mod.rs`, `src/cli/args.rs`

**任务描述**：
- 使用 clap 实现完整的命令行参数解析
- 定义全局选项和命令结构
- 实现命令路由和执行

**验收标准**：
- [ ] `teammate --help` 显示正确的帮助信息
- [ ] `teammate --version` 返回正确的版本号
- [ ] 未知命令返回友好的错误提示
- [ ] 支持 `--verbose`/`-v` 详细输出模式

**实现步骤**：

```rust
// src/cli/args.rs

use clap::{Parser, Subcommand, ValueEnum};

/// Teammate - TODO CLI 工具
#[derive(Parser, Debug)]
#[command(name = "teammate")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 全局日志级别
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// 配置文件路径
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// 项目路径
    #[arg(short, long)]
    pub project: Option<PathBuf>,

    /// 输出格式
    #[arg(long, value_enum)]
    pub format: Option<OutputFormat>,

    /// 禁用颜色输出
    #[arg(long)]
    pub no_color: bool,

    /// 命令
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 扫描代码库中的 TODO
    #[command(alias = "s")]
    Scan(ScanArgs),

    /// 列出 TODO
    #[command(alias = "ls")]
    List(ListArgs),

    /// 添加新 TODO
    #[command(alias = "a")]
    Add(AddArgs),

    /// 删除 TODO
    #[command(alias = "rm", "del")]
    Remove(RemoveArgs),

    /// 更新 TODO
    #[command(alias = "u")]
    Update(UpdateArgs),

    /// 更新 TODO 状态
    Status(StatusArgs),

    /// 管理标签
    Tag(TagArgs),

    /// 管理配置
    Config(ConfigArgs),

    /// 初始化项目
    Init(InitArgs),

    /// 显示统计信息
    Stats(StatsArgs),

    /// 查看 TODO 的 Git blame
    Blame(BlameArgs),

    /// 分支 TODO 管理
    Branch(BranchArgs),

    /// 同步数据
    Sync(SyncArgs),

    /// 管理 Git hooks
    Hooks(HooksArgs),
}

#[derive(Debug, ValueEnum, Clone)]
pub enum OutputFormat {
    /// 表格格式（默认）
    Table,
    /// JSON 格式
    Json,
    /// 简洁格式
    Compact,
    /// CSV 格式
    Csv,
}
```

#### 任务 3.1.2：实现 scan 命令

**文件**: `src/cli/commands/scan.rs`

**任务描述**：
- 实现代码库扫描命令
- 支持文件过滤、模式配置
- 输出扫描结果统计

**验收标准**：
- [ ] `teammate scan` 扫描当前目录
- [ ] `teammate scan ./src` 扫描指定目录
- [ ] `teammate scan --ext rs,py` 过滤文件类型
- [ ] `teammate scan --fix` 将扫描结果添加到数据库
- [ ] 扫描 10K 文件耗时 < 5 秒

**实现步骤**：

```rust
// src/cli/commands/scan.rs

use crate::cli::{Args, Commands};
use crate::core::config::AppConfig;
use crate::scan::TodoScanner;
use crate::storage::Storage;
use anyhow::{Context, Result};
use clap::{Args as ClapArgs, Parser};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, ClapArgs)]
pub struct ScanArgs {
    /// 要扫描的目录路径
    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,

    /// 文件扩展名过滤（逗号分隔）
    #[arg(short, long)]
    pub exts: Option<String>,

    /// 排除的目录模式（逗号分隔）
    #[arg(short, long)]
    pub ignore: Option<String>,

    /// 不递归扫描
    #[arg(long)]
    pub no_recursive: bool,

    /// 自定义 TODO 模式
    #[arg(long)]
    pub pattern: Option<String>,

    /// 扫描并添加到数据库
    #[arg(short = 'f', long)]
    pub fix: bool,

    /// 仅显示统计信息
    #[arg(long)]
    pub stats_only: bool,

    /// 最大文件大小（字节）
    #[arg(long)]
    pub max_size: Option<u64>,
}

pub async fn run(
    args: &ScanArgs,
    config: &AppConfig,
    storage: &Storage,
) -> Result<()> {
    let start = Instant::new();

    // 创建扫描器
    let scanner = TodoScanner::new(config)?;

    // 构建扫描选项
    let options = ScanOptions {
        paths: args.paths.clone(),
        extensions: args.exts.as_ref().map(|e| e.split(',').collect()),
        ignore_patterns: args.ignore.as_ref().map(|e| e.split(',').collect()),
        recursive: !args.no_recursive,
        custom_pattern: args.pattern.clone(),
        max_file_size: args.max_size.unwrap_or(1024 * 1024),
    };

    // 执行扫描
    let result = scanner.scan(&options)?;

    // 如果指定 --fix，保存到数据库
    if args.fix {
        let saved = storage.import_todos(&result.todos)?;
        println!("已保存 {} 个 TODO", saved);
    }

    // 输出结果
    if !args.stats_only {
        print_scan_results(&result)?;
    }

    // 输出统计
    let elapsed = start.elapsed();
    println!("\n扫描完成！耗时: {:.2}秒", elapsed.as_secs_f64());
    println!("扫描文件: {}", result.stats.files_scanned);
    println!("发现 TODO: {}", result.stats.todos_found);
    println!("发现类型: {:?}", result.stats.by_language);

    Ok(())
}

fn print_scan_results(result: &ScanResult) -> Result<()> {
    println!("\n发现 {} 个 TODO:\n", result.todos.len());

    // 按文件分组显示
    for (file, todos) in &result.todos_by_file {
        println!("📄 {}", file.display());
        for todo in todos {
            println!("  {:>4} │ {:<8} │ {}",
                todo.line_number,
                format!("[{:?}]", todo.priority).to_uppercase(),
                todo.content
            );
        }
        println!();
    }

    Ok(())
}
```

#### 任务 3.1.3：实现 list 命令

**文件**: `src/cli/commands/list.rs`

**任务描述**：
- 实现 TODO 列表查询命令
- 支持多维度过滤和排序
- 多种输出格式支持

**验收标准**：
- [ ] `teammate list` 列出所有 TODO
- [ ] `teammate list --status pending` 按状态过滤
- [ ] `teammate list --priority high` 按优先级过滤
- [ ] `teammate list --tag bug` 按标签过滤
- [ ] `teammate list --format json` JSON 输出
- [ ] 支持 `--sort` 排序和 `--order` 排序方向

**实现步骤**：

```rust
// src/cli/commands/list.rs

#[derive(Debug, ClapArgs)]
pub struct ListArgs {
    /// 按状态过滤
    #[arg(long)]
    pub status: Option<TodoStatusFilter>,

    /// 按优先级过滤
    #[arg(short, long)]
    pub priority: Option<Priority>,

    /// 按标签过滤
    #[arg(short, long)]
    pub tag: Option<String>,

    /// 按作者过滤
    #[arg(long)]
    pub author: Option<String>,

    /// 按文件过滤
    #[arg(long)]
    pub file: Option<PathBuf>,

    /// 按项目过滤
    #[arg(long)]
    pub project: Option<String>,

    /// 仅显示已过期
    #[arg(long)]
    pub overdue: bool,

    /// 仅显示即将到期（7天内）
    #[arg(long)]
    pub due_soon: bool,

    /// 搜索关键词
    #[arg(short, long)]
    pub search: Option<String>,

    /// 排序字段
    #[arg(long)]
    pub sort: Option<SortField>,

    /// 排序方向
    #[arg(long)]
    pub order: Option<SortOrder>,

    /// 限制数量
    #[arg(short, long)]
    pub limit: Option<u32>,

    /// 偏移量（分页）
    #[arg(long)]
    pub offset: Option<u32>,

    /// 输出格式
    #[arg(short, long, value_enum)]
    pub format: Option<OutputFormat>,

    /// 仅显示 ID
    #[arg(long)]
    pub ids: bool,

    /// 显示已归档
    #[arg(long)]
    pub archived: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TodoStatusFilter {
    /// 全部
    All,
    /// 进行中
    Open,
    /// 进行中
    InProgress,
    /// 已解决
    Done,
    /// 已关闭
    Closed,
    /// 已归档
    Archived,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SortField {
    /// 按创建时间
    Created,
    /// 按更新时间
    Updated,
    /// 按优先级
    Priority,
    /// 按截止日期
    Due,
    /// 按文件路径
    File,
    /// 按行号
    Line,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SortOrder {
    /// 升序
    Asc,
    /// 降序
    Desc,
}
```

#### 任务 3.1.4：实现 add 命令

**文件**: `src/cli/commands/add.rs`

**任务描述**：
- 实现手动添加 TODO 功能
- 支持交互式输入
- 自动完成标签、优先级等字段

**验收标准**：
- [ ] `teammate add "修复登录 bug"` 简单添加
- [ ] `teammate add "任务" --priority high` 指定优先级
- [ ] `teammate add "任务" --tag bug,urgent` 指定标签
- [ ] `teammate add --interactive` 交互式添加
- [ ] 支持 `--parent` 指定父 TODO

**实现步骤**：

```rust
// src/cli/commands/add.rs

#[derive(Debug, ClapArgs)]
pub struct AddArgs {
    /// TODO 内容（必填）
    #[arg(required = true)]
    pub title: String,

    /// 详细描述
    #[arg(short, long)]
    pub description: Option<String>,

    /// 优先级
    #[arg(short, long, value_enum)]
    pub priority: Option<Priority>,

    /// 截止日期
    #[arg(short, long)]
    pub due: Option<String>,

    /// 标签
    #[arg(short, long)]
    pub tag: Vec<String>,

    /// 指定项目
    #[arg(long)]
    pub project: Option<String>,

    /// 父 TODO ID
    #[arg(long)]
    pub parent: Option<String>,

    /// 文件路径
    #[arg(long)]
    pub file: Option<PathBuf>,

    /// 行号
    #[arg(long)]
    pub line: Option<u32>,

    /// 作者
    #[arg(long)]
    pub author: Option<String>,

    /// 关联 Issue
    #[arg(long)]
    pub issue: Vec<String>,

    /// 交互式添加
    #[arg(short, long)]
    pub interactive: bool,

    /// 从文件批量导入
    #[arg(long)]
    pub from_file: Option<PathBuf>,
}
```

#### 任务 3.1.5：实现 status 命令

**文件**: `src/cli/commands/status.rs`

**任务描述**：
- 实现 TODO 状态更新
- 支持状态流转历史记录
- 快捷完成命令

**验收标准**：
- [ ] `teammate status 1 done` 更新状态
- [ ] `teammate done 1` 标记完成
- [ ] `teammate undo 1` 撤销完成
- [ ] `teammate done --all` 完成所有
- [ ] 状态变更记录到历史表

**实现步骤**：

```rust
// src/cli/commands/status.rs

#[derive(Debug, ClapArgs)]
pub struct StatusArgs {
    /// TODO ID
    #[arg(required = true)]
    pub id: String,

    /// 新状态
    #[arg(value_enum)]
    pub status: Option<TodoStatus>,

    /// 添加完成注释
    #[arg(long)]
    pub comment: Option<String>,

    /// 完成日期（默认现在）
    #[arg(long)]
    pub at: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum StatusCommands {
    /// 设置状态
    Set(StatusArgs),

    /// 标记为完成
    Done(DoneArgs),

    /// 撤销完成
    Undo(UndoArgs),
}

#[derive(Debug, ClapArgs)]
pub struct DoneArgs {
    /// TODO ID（支持多个，逗号分隔）
    pub ids: String,

    /// 添加完成注释
    #[arg(long)]
    pub comment: Option<String>,

    /// 完成所有匹配的 TODO
    #[arg(long)]
    pub all: bool,

    /// 完成早于指定日期的所有
    #[arg(long)]
    pub older: Option<String>,
}

#[derive(Debug, ClapArgs)]
pub struct UndoArgs {
    /// TODO ID
    pub ids: String,

    /// 撤销所有已完成的
    #[arg(long)]
    pub all: bool,

    /// 保留完成注释
    #[arg(long)]
    pub keep_comment: bool,
}
```

#### 任务 3.1.6：实现 tag 命令

**文件**: `src/cli/commands/tag.rs`

**任务描述**：
- 实现 TODO 标签管理
- 支持标签别名、合并
- 标签使用统计

**验收标准**：
- [ ] `teammate tag add bug --color red` 添加标签
- [ ] `teammate tag list` 列出所有标签
- [ ] `teammate tag rename old new` 重命名标签
- [ ] `teammate tag merge source target` 合并标签
- [ ] `teammate tag remove tagname` 删除标签
- [ ] `teammate tag colors` 显示标签颜色

**实现步骤**：

```rust
// src/cli/commands/tag.rs

#[derive(Debug, Subcommand)]
pub enum TagCommands {
    /// 列出所有标签
    List,

    /// 添加标签
    Add(TagAddArgs),

    /// 重命名标签
    Rename(TagRenameArgs),

    /// 合并标签
    Merge(TagMergeArgs),

    /// 删除标签
    Remove(TagRemoveArgs),

    /// 显示标签颜色配置
    Colors,
}

#[derive(Debug, ClapArgs)]
pub struct TagAddArgs {
    /// 标签名称
    pub name: String,

    /// 标签颜色
    #[arg(short, long)]
    pub color: Option<String>,

    /// 标签描述
    #[arg(long)]
    pub description: Option<String>,

    /// 标签分类
    #[arg(long)]
    pub category: Option<String>,
}
```

#### 任务 3.1.7：实现 config 命令

**文件**: `src/cli/commands/config.rs`

**任务描述**：
- 实现配置文件管理
- 支持查看、修改、导出配置

**验收标准**：
- [ ] `teammate config get editor` 获取配置
- [ ] `teammate config set default_priority high` 设置配置
- [ ] `teammate config unset confirm_delete` 移除配置
- [ ] `teammate config list` 列出所有配置
- [ ] `teammate config edit` 打开编辑器编辑
- [ ] `teammate config export` 导出配置
- [ ] `teammate config reset` 重置为默认

**实现步骤**：

```rust
// src/cli/commands/config.rs

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// 获取配置值
    Get(ConfigGetArgs),

    /// 设置配置值
    Set(ConfigSetArgs),

    /// 移除配置
    Unset(ConfigUnsetArgs),

    /// 列出所有配置
    List,

    /// 编辑配置文件
    Edit,

    /// 导出配置
    Export(ConfigExportArgs),

    /// 导入配置
    Import(ConfigImportArgs),

    /// 重置为默认值
    Reset,
}

#[derive(Debug, ClapArgs)]
pub struct ConfigSetArgs {
    /// 配置键
    pub key: String,

    /// 配置值
    pub value: String,
}
```

#### 任务 3.1.8：实现 blame 命令

**文件**: `src/cli/commands/blame.rs`

**任务描述**：
- 实现 TODO Git blame 查询
- 显示作者信息和修改历史

**验收标准**：
- [ ] `teammate blame <todo-id>` 显示 blame 信息
- [ ] 显示作者、提交时间、提交信息
- [ ] 显示 TODO 修改历史
- [ ] `--format json` JSON 输出

**实现步骤**：

```rust
// src/cli/commands/blame.rs

#[derive(Debug, ClapArgs)]
pub struct BlameArgs {
    /// TODO ID 或序号
    pub id: String,

    /// 显示完整历史
    #[arg(long)]
    pub history: bool,

    /// 输出格式
    #[arg(short, long, value_enum)]
    pub format: Option<OutputFormat>,
}
```

#### 任务 3.1.9：实现 branch 命令

**文件**: `src/cli/commands/branch.rs`

**任务描述**：
- 实现分支 TODO 管理
- 分支间 TODO 比较

**验收标准**：
- [ ] `teammate branch` 显示当前分支 TODO
- [ ] `teammate branch feature/login` 指定分支
- [ ] `teammate branch compare main feature` 比较分支
- [ ] `teammate branch stats` 分支 TODO 统计

**实现步骤**：

```rust
// src/cli/commands/branch.rs

#[derive(Debug, Subcommand)]
pub enum BranchCommands {
    /// 显示当前分支 TODO
    List(BranchListArgs),

    /// 比较分支
    Compare(BranchCompareArgs),

    /// 显示统计
    Stats(BranchStatsArgs),
}

#[derive(Debug, ClapArgs)]
pub struct BranchListArgs {
    /// 分支名称
    pub branch: Option<String>,

    /// 过滤条件
    #[arg(long)]
    pub status: Option<TodoStatusFilter>,

    #[arg(long)]
    pub priority: Option<Priority>,
}

#[derive(Debug, ClapArgs)]
pub struct BranchCompareArgs {
    /// 源分支
    pub source: String,

    /// 目标分支
    pub target: String,
}
```

#### 任务 3.1.10：实现 hooks 命令

**文件**: `src/cli/commands/hooks.rs`

**任务描述**：
- 实现 Git hooks 管理
- 安装、卸载 hooks

**验收标准**：
- [ ] `teammate hooks install` 安装所有 hooks
- [ ] `teammate hooks uninstall` 卸载所有 hooks
- [ ] `teammate hooks status` 查看状态
- [ ] `teammate hooks enable pre-commit` 启用单个 hook
- [ ] `teammate hooks disable pre-commit` 禁用单个 hook

**实现步骤**：

```rust
// src/cli/commands/hooks.rs

#[derive(Debug, Subcommand)]
pub enum HookCommands {
    /// 安装 hooks
    Install(HookInstallArgs),

    /// 卸载 hooks
    Uninstall(HookUninstallArgs),

    /// 查看 hooks 状态
    Status,

    /// 启用 hook
    Enable(HookEnableArgs),

    /// 禁用 hook
    Disable(HookDisableArgs),

    /// 运行 hook
    Run(HookRunArgs),
}

#[derive(Debug, ClapArgs)]
pub struct HookInstallArgs {
    /// 只安装指定 hook
    #[arg(value_enum)]
    pub hook: Option<HookType>,

    /// 强制覆盖
    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum HookType {
    PreCommit,
    PostCommit,
    PostCheckout,
    PrePush,
    PostMerge,
    PostRebase,
}
```

#### 任务 3.1.11：实现 output 模块

**文件**: `src/cli/output.rs`

**任务描述**：
- 实现多种输出格式
- 表格格式化、JSON 序列化

**验收标准**：
- [ ] Table 格式：美观对齐、带颜色
- [ ] JSON 格式：结构化数据
- [ ] Compact 格式：简洁单行
- [ ] CSV 格式：可导入 Excel
- [ ] 支持 `--no-color` 禁用颜色

**实现步骤**：

```rust
// src/cli/output.rs

use crate::core::models::Todo;
use crate::cli::OutputFormat;
use std::path::PathBuf;

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn format_todos(todos: &[Todo], format: OutputFormat) -> String {
        match format {
            OutputFormat::Table => Self::format_table(todos),
            OutputFormat::Json => Self::format_json(todos),
            OutputFormat::Compact => Self::format_compact(todos),
            OutputFormat::Csv => Self::format_csv(todos),
        }
    }

    fn format_table(todos: &[Todo]) -> String {
        // 使用 unicode-table 或手动实现
    }

    fn format_json(todos: &[Todo]) -> String {
        serde_json::to_string_pretty(todos).unwrap()
    }

    fn format_compact(todos: &[Todo]) -> String {
        todos.iter()
            .map(|t| format!("[{}] {} {}",
                Self::status_icon(&t.status),
                t.id,
                t.content
            ))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_csv(todos: &[Todo]) -> String {
        // CSV 格式实现
    }

    fn status_icon(status: &TodoStatus) -> char {
        match status {
            TodoStatus::Open => ' ',
            TodoStatus::InProgress => '▶',
            TodoStatus::Resolved => '✓',
            TodoStatus::Closed => '×',
            TodoStatus::Archived => '⊢',
        }
    }
}
```

---

### 3.2 存储层实现

#### 任务 3.2.1：设计并实现数据库 schema

**文件**: `src/storage/database.rs`, `src/storage/migrations/*.rs`

**任务描述**：
- 设计 SQLite 数据库 schema
- 实现数据访问层
- 实现数据库迁移机制

**验收标准**：
- [ ] 数据库 schema 符合第三范式
- [ ] 支持事务操作
- [ ] 实现版本迁移
- [ ] 性能：单次查询 < 100ms
- [ ] 支持 WAL 模式优化并发

**实现步骤**：

```rust
// src/storage/database.rs

use rusqlite::{Connection, Result, OpenFlags};
use std::path::PathBuf;
use once_cell::sync::Lazy;
use crate::core::config::DatabaseConfig;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &PathBuf, config: &DatabaseConfig) -> Result<Self> {
        // 配置连接参数
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX;  // 单线程模式

        let conn = Connection::open_with_flags(path, flags)?;

        // 设置 WAL 模式
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "cache_size", "-64000")?;  // 64MB

        // 启用外键约束
        conn.pragma_update(None, "foreign_keys", "ON")?;

        Ok(Self { conn })
    }

    pub fn initialize(&self) -> Result<()> {
        // 创建表
        self.create_projects_table()?;
        self.create_todos_table()?;
        self.create_tags_table()?;
        self.create_todo_tags_table()?;
        self.create_filters_table()?;
        self.create_scan_sessions_table()?;
        self.create_sync_history_table()?;
        self.create_schema_migrations_table()?;

        // 创建索引
        self.create_indexes()?;

        Ok(())
    }

    fn create_todos_table(&self) -> Result<()> {
        self.conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS todos (
                id TEXT PRIMARY KEY,
                project_id TEXT,
                content TEXT NOT NULL,
                description TEXT,
                file_path TEXT NOT NULL,
                line_number INTEGER NOT NULL,
                end_line INTEGER,
                language TEXT,
                code_context TEXT,
                status TEXT NOT NULL DEFAULT 'open'
                    CHECK(status IN ('open', 'in_progress', 'resolved', 'closed', 'archived')),
                priority TEXT NOT NULL DEFAULT 'medium'
                    CHECK(priority IN ('low', 'medium', 'high', 'urgent', 'none')),
                due_date TEXT,
                author TEXT,
                assignee TEXT,
                linked_issues TEXT,
                linked_commits TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now')),
                started_at TEXT,
                completed_at TEXT,
                last_scanned_at TEXT,
                is_manual INTEGER NOT NULL DEFAULT 0,
                is_ignored INTEGER NOT NULL DEFAULT 0,
                ignore_reason TEXT,
                metadata TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )
        "#)?;
        Ok(())
    }

    fn create_todo_tags_table(&self) -> Result<()> {
        self.conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS todo_tags (
                todo_id TEXT NOT NULL,
                tag_id TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                PRIMARY KEY (todo_id, tag_id),
                FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            )
        "#)?;
        Ok(())
    }

    fn create_indexes(&self) -> Result<()> {
        self.conn.execute_batch(r#"
            CREATE INDEX IF NOT EXISTS idx_todos_project ON todos(project_id);
            CREATE INDEX IF NOT EXISTS idx_todos_status ON todos(status);
            CREATE INDEX IF NOT EXISTS idx_todos_priority ON todos(priority);
            CREATE INDEX IF NOT EXISTS idx_todos_file ON todos(file_path);
            CREATE INDEX IF NOT EXISTS idx_todos_author ON todos(author);
            CREATE INDEX IF NOT EXISTS idx_todos_due_date ON todos(due_date);
            CREATE INDEX IF NOT EXISTS idx_todos_created_at ON todos(created_at);

            CREATE INDEX IF NOT EXISTS idx_todo_tags_todo ON todo_tags(todo_id);
            CREATE INDEX IF NOT EXISTS idx_todo_tags_tag ON todo_tags(tag_id);

            CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
            CREATE INDEX IF NOT EXISTS idx_tags_project ON tags(project_id);
        "#)?;
        Ok(())
    }
}
```

#### 任务 3.2.2：实现 Todo CRUD 操作

**文件**: `src/storage/queries/todo_queries.rs`

**任务描述**：
- 实现 TODO 的增删改查操作
- 支持批量操作
- 实现过滤查询

**验收标准**：
- [ ] 支持单条 TODO 的增删改查
- [ ] 支持批量导入/导出
- [ ] 支持多条件过滤
- [ ] 支持分页查询
- [ ] 批量插入 1000 条 < 1 秒

**实现步骤**：

```rust
// src/storage/queries/todo_queries.rs

use rusqlite::{params, Connection, Result, Row};
use crate::core::models::{Todo, TodoStatus, Priority};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct TodoQueries<'a> {
    conn: &'a Connection,
}

impl<'a> TodoQueries<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, todo: &Todo) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO todos (
                id, project_id, content, description, file_path, line_number,
                end_line, language, code_context, status, priority, due_date,
                author, assignee, linked_issues, linked_commits, created_at,
                updated_at, is_manual, metadata
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
            params![
                todo.id,
                todo.project_id,
                todo.content,
                todo.description,
                todo.file_path.to_string_lossy(),
                todo.line_number,
                todo.end_line,
                todo.language,
                todo.code_context,
                serde_json::to_string(&todo.status)?,
                serde_json::to_string(&todo.priority)?,
                todo.due_date.map(|d| d.to_rfc3339()),
                todo.author,
                todo.assignee,
                serde_json::to_string(&todo.linked_issues)?,
                serde_json::to_string(&todo.linked_commits)?,
                todo.created_at.to_rfc3339(),
                todo.updated_at.to_rfc3339(),
                todo.is_manual,
                todo.metadata.map(|m| m.to_string()),
            ],
        )?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<Todo>> {
        self.conn.query_row_and_then(
            "SELECT * FROM todos WHERE id = ?",
            params![id],
            |row| Self::row_to_todo(row),
        )
    }

    pub fn list(&self, filter: &TodoFilter) -> Result<Vec<Todo>> {
        let mut query = "SELECT * FROM todos WHERE 1=1".to_string();
        let mut params: Vec<rusqlite::types::Value> = Vec::new();

        if let Some(status) = &filter.status {
            query.push_str(" AND status = ?");
            params.push(serde_json::to_string(status)?.into());
        }

        if let Some(priority) = &filter.priority {
            query.push_str(" AND priority = ?");
            params.push(serde_json::to_string(priority)?.into());
        }

        if let Some(tag) = &filter.tag {
            query.push_str(
                " AND id IN (SELECT todo_id FROM todo_tags tt
                 JOIN tags t ON tt.tag_id = t.id WHERE t.name = ?)"
            );
            params.push(tag.clone().into());
        }

        query.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = filter.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        let mut stmt = self.conn.prepare(&query)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(&params), |row| {
            Self::row_to_todo(row)
        })?;

        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn update_status(&self, id: &str, status: TodoStatus) -> Result<()> {
        let now = Utc::now();
        self.conn.execute(
            "UPDATE todos SET status = ?, updated_at = ?, completed_at = ? WHERE id = ?",
            params![
                serde_json::to_string(&status)?,
                now.to_rfc3339(),
                match status {
                    TodoStatus::Resolved | TodoStatus::Closed => Some(now.to_rfc3339()),
                    _ => None,
                },
                id,
            ],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM todos WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn bulk_insert(&self, todos: &[Todo]) -> Result<usize> {
        let tx = self.conn.transaction()?;
        let mut inserted = 0;

        for todo in todos {
            // 使用 INSERT OR IGNORE 避免重复
            tx.execute(
                "INSERT OR IGNORE INTO todos (...) VALUES (...)",
                params![...],
            )?;
            inserted += 1;
        }

        tx.commit()?;
        Ok(inserted)
    }

    fn row_to_todo(row: &Row) -> Result<Todo> {
        // 行转换逻辑
    }
}

pub struct TodoFilter {
    pub status: Option<TodoStatus>,
    pub priority: Option<Priority>,
    pub tag: Option<String>,
    pub author: Option<String>,
    pub file: Option<PathBuf>,
    pub project: Option<String>,
    pub search: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
```

#### 任务 3.2.3：实现数据库迁移

**文件**: `src/storage/migrations/mod.rs`

**任务描述**：
- 实现数据库版本迁移机制
- 支持迁移回滚

**验收标准**：
- [ ] 自动检测当前版本
- [ ] 按顺序执行迁移
- [ ] 记录迁移历史
- [ ] 迁移失败回滚

**实现步骤**：

```rust
// src/storage/migrations/mod.rs

use rusqlite::{Connection, Result};
use std::path::PathBuf;

const CURRENT_VERSION: i32 = 3;

pub struct Migration;

impl Migration {
    pub fn run(conn: &Connection) -> Result<()> {
        let current_version = Self::get_current_version(conn)?;

        if current_version < 1 {
            Self::v1_initial_schema(conn)?;
        }
        if current_version < 2 {
            Self::v2_scan_sessions(conn)?;
        }
        if current_version < 3 {
            Self::v3_sync_tables(conn)?;
        }

        Ok(())
    }

    fn v1_initial_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(include_str!("./v1_initial.sql"))?;
        Self::record_migration(conn, 1, "Initial schema")?;
        Ok(())
    }

    fn v2_scan_sessions(conn: &Connection) -> Result<()> {
        conn.execute_batch(include_str!("./v2_scan_sessions.sql"))?;
        Self::record_migration(conn, 2, "Add scan_sessions table")?;
        Ok(())
    }

    fn v3_sync_tables(conn: &Connection) -> Result<()> {
        conn.execute_batch(include_str!("./v3_sync_tables.sql"))?;
        Self::record_migration(conn, 3, "Add sync tables")?;
        Ok(())
    }

    fn get_current_version(conn: &Connection) -> Result<i32> {
        conn.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
    }

    fn record_migration(conn: &Connection, version: i32, description: &str) -> Result<()> {
        conn.execute(
            "INSERT INTO schema_migrations (version, description) VALUES (?, ?)",
            params![version, description],
        )?;
        Ok(())
    }
}
```

#### 任务 3.2.4：实现缓存系统

**文件**: `src/storage/cache/memory_cache.rs`, `src/storage/cache/disk_cache.rs`

**任务描述**：
- 实现多级缓存系统
- 内存缓存 + 磁盘缓存

**验收标准**：
- [ ] L1 内存缓存：容量 10,000 项
- [ ] L2 磁盘缓存：最大 100MB
- [ ] 缓存过期时间可配置
- [ ] 缓存命中率 > 70%

**实现步骤**：

```rust
// src/storage/cache/memory_cache.rs

use moka::future::Cache;
use std::time::Duration;
use crate::core::models::Todo;

pub struct MemoryCache {
    todo_cache: Cache<String, Todo>,
    tag_cache: Cache<String, Vec<String>>,
    stats_cache: Cache<String, serde_json::Value>,
}

impl MemoryCache {
    pub fn new() -> Self {
        Self {
            todo_cache: Cache::builder()
                .max_capacity(10_000)
                .time_to_live(Duration::from_secs(300))  // 5 分钟
                .build(),
            tag_cache: Cache::builder()
                .max_capacity(1_000)
                .time_to_live(Duration::from_secs(600))  // 10 分钟
                .build(),
            stats_cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(60))  // 1 分钟
                .build(),
        }
    }

    pub fn get_todo(&self, id: &str) -> Option<Todo> {
        self.todo_cache.get(id).await
    }

    pub fn set_todo(&self, id: &str, todo: &Todo) {
        self.todo_cache.insert(id.to_string(), todo.clone()).await;
    }

    pub fn invalidate_todo(&self, id: &str) {
        self.todo_cache.invalidate(id).await;
    }

    pub fn invalidate_all(&self) {
        self.todo_cache.invalidate_all().await;
    }
}
```

---

### 3.3 多语言解析器

#### 任务 3.3.1：实现解析器 trait

**文件**: `src/parsers/traits.rs`

**任务描述**：
- 定义解析器接口 trait
- 实现解析结果数据结构

**验收标准**：
- [ ] `Parser` trait 包含所有必要方法
- [ ] `ParsedTodo` 包含完整信息
- [ ] 支持增量解析
- [ ] 支持自定义模式

**实现步骤**：

```rust
// src/parsers/traits.rs

use crate::core::models::{Todo, Priority, TodoStatus};
use std::path::PathBuf;
use regex::Regex;

pub trait Parser: Send + Sync {
    fn name(&self) -> &'static str;
    fn extensions(&self) -> &'static [&'static str];
    fn shebangs(&self) -> Option<&'static [&'static str]>;
    fn parse(&self, content: &str, file_path: &PathBuf) -> Vec<ParsedTodo>;
    fn validate_config(&self, config: &ParserConfig) -> Result<(), ParserError>;
}

pub trait AdvancedParser: Parser {
    fn custom_patterns(&self) -> Option<&'static [CustomPattern]>;
    fn extract_metadata(&self, content: &str, line: usize) -> Option<TodoMetadata>;
    fn parse_incremental(&self, old_content: &str, new_content: &str, file_path: &PathBuf) -> IncrementalResult;
}

#[derive(Debug, Clone)]
pub struct ParsedTodo {
    pub raw_content: String,
    pub clean_content: String,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub language: String,
    pub todo_type: TodoType,
    pub tags: Vec<String>,
    pub priority: Priority,
    pub author: Option<String>,
    pub linked_issues: Vec<String>,
    pub created_at: Option<String>,
    pub parser_name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TodoType {
    Todo,
    Fixme,
    Bug,
    Feature,
    Optimize,
    Doc,
    Security,
    Deprecated,
    Question,
    Custom(&'static str),
}

#[derive(Debug)]
pub struct CustomPattern {
    pub name: &'static str,
    pub regex: Regex,
    pub todo_type: TodoType,
    pub default_priority: Priority,
    pub tags: Vec<&'static str>,
}
```

#### 任务 3.3.2：实现 JavaScript/TypeScript 解析器

**文件**: `src/parsers/javascript.rs`

**任务描述**：
- 实现 JS/TS TODO 解析
- 支持多种注释风格
- 提取标签和优先级

**验收标准**：
- [ ] 解析 `// TODO` 单行注释
- [ ] 解析 `/* TODO */` 块注释
- [ ] 解析 JSDoc `@todo`
- [ ] 提取 `(author)` `(priority)` 格式

**实现步骤**：

```rust
// src/parsers/javascript.rs

use super::{Parser, ParsedTodo, TodoType, CustomPattern};
use crate::core::models::Priority;
use regex::Regex;
use std::path::PathBuf;

pub struct JavaScriptParser;

impl Parser for JavaScriptParser {
    fn name(&self) -> &'static str {
        "JavaScript/TypeScript"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["js", "jsx", "mjs", "cjs", "ts", "tsx", "mts", "cts"]
    }

    fn shebangs(&self) -> Option<&'static [&'static str]> {
        Some(&["#!/usr/bin/node", "#!/usr/bin/env node"])
    }

    fn parse(&self, content: &str, file_path: &PathBuf) -> Vec<ParsedTodo> {
        let mut todos = Vec::new();

        // 单行注释模式
        let single_line = Regex::new(
            r"(?m)//\s*(TODO|FIXME|XXX|BUG|HACK|NOTE|TODO\(([^)]+)\))[:\s]*(.*)$"
        ).unwrap();

        // 多行注释模式
        let multi_line = Regex::new(
            r"/\*s*(TODO|FIXME|XXX|BUG|HACK|NOTE)\s*[:*]*\s*(.*?)\s*\*/"
        ).unwrap();

        // JSDoc 模式
        let jsdoc = Regex::new(
            r"/\*\*\s*@todo\s*(.*?)(?:\*/|$)"
        ).unwrap();

        // TSDoc 模式
        let tsdoc = Regex::new(
            r"///\s*@todo\s*(.*)$"
        ).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = single_line.captures(line) {
                todos.push(self.build_parsed_todo(
                    line,
                    line_num + 1,
                    file_path,
                    caps.get(1).unwrap().as_str(),
                    caps.get(3).unwrap().as_str(),
                ));
            }

            if let Some(caps) = tsdoc.captures(line) {
                todos.push(self.build_parsed_todo(
                    line,
                    line_num + 1,
                    file_path,
                    "TODO",
                    caps.get(1).unwrap().as_str().trim(),
                ));
            }
        }

        todos
    }

    fn validate_config(&self, config: &ParserConfig) -> Result<(), ParserError> {
        Ok(())
    }
}

impl JavaScriptParser {
    fn build_parsed_todo(&self, line: &str, line_num: usize, file_path: &PathBuf, todo_type: &str, content: &str) -> ParsedTodo {
        // 提取标签
        let tags = self.extract_tags(content);

        // 提取优先级
        let priority = self.extract_priority(content);

        // 提取作者
        let author = self.extract_author(content);

        // 提取关联 Issue
        let linked_issues = self.extract_issues(content);

        ParsedTodo {
            raw_content: line.to_string(),
            clean_content: content.trim().to_string(),
            file_path: file_path.clone(),
            line_number,
            column_start: 0,
            column_end: line.len(),
            language: "javascript".to_string(),
            todo_type: Self::parse_todo_type(todo_type),
            tags,
            priority,
            author,
            linked_issues,
            created_at: None,
            parser_name: "JavaScript/TypeScript",
        }
    }

    fn extract_tags(&self, content: &str) -> Vec<String> {
        let tag_pattern = Regex::new(r"@(\w+)").unwrap();
        tag_pattern.captures_iter(content)
            .filter_map(|c| Some(c.get(1)?.as_str().to_string()))
            .collect()
    }

    fn extract_priority(&self, content: &str) -> Priority {
        if content.to_uppercase().contains("(URGENT)") {
            return Priority::Urgent;
        }
        if content.to_uppercase().contains("(HIGH)") {
            return Priority::High;
        }
        if content.to_uppercase().contains("(LOW)") {
            return Priority::Low;
        }
        Priority::None
    }

    fn parse_todo_type(todo_type: &str) -> TodoType {
        match todo_type.to_uppercase().as_str() {
            "FIXME" => TodoType::Fixme,
            "BUG" => TodoType::Bug,
            "HACK" => TodoType::Optimize,
            "NOTE" => TodoType::Doc,
            _ => TodoType::Todo,
        }
    }
}
```

#### 任务 3.3.3：实现 Python 解析器

**文件**: `src/parsers/python.rs`

**任务描述**：
- 实现 Python TODO 解析
- 支持 `# TODO` 风格
- 支持 `# TODO(author):` 格式
- 支持文档字符串中的 TODO

**实现步骤**：

```rust
// src/parsers/python.rs

use super::{Parser, ParsedTodo, TodoType};
use regex::Regex;
use std::path::PathBuf;

pub struct PythonParser;

impl Parser for PythonParser {
    fn name(&self) -> &'static str {
        "Python"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["py", "pyi", "pyw"]
    }

    fn shebangs(&self) -> Option<&'static [&'static str]> {
        Some(&[
            "#!/usr/bin/python",
            "#!/usr/bin/env python",
            "#!/usr/bin/python3",
            "#!/usr/bin/env python3",
        ])
    }

    fn parse(&self, content: &str, file_path: &PathBuf) -> Vec<ParsedTodo> {
        let mut todos = Vec::new();

        // 单行注释
        let single_line = Regex::new(
            r"(?m)#\s*(TODO|FIXME|XXX|BUG|HACK|NOTE|TODO\(([^)]+)\))[:\s]*(.*)$"
        ).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            if line_num == 0 && line.starts_with("#!") {
                continue;  // 跳过 shebang
            }

            if let Some(caps) = single_line.captures(line) {
                todos.push(self.build_parsed_todo(
                    line,
                    line_num + 1,
                    file_path,
                    caps.get(1).unwrap().as_str(),
                    caps.get(3).unwrap().as_str(),
                ));
            }
        }

        // 文档字符串
        self.parse_docstrings(content, file_path, &mut todos);

        todos
    }

    fn parse_docstrings(&self, content: &str, file_path: &PathBuf, todos: &mut Vec<ParsedTodo>) {
        // 解析 """ 或 ''' 包围的文档字符串
        let triple_doc = Regex::new(r#"(?s)(?:"""|''')(.*?)(?:"""|''')"#).unwrap();

        for mat in triple_doc.find_iter(content) {
            let docstring = mat.as_str();
            let todo_pattern = Regex::new(r"(TODO|FIXME|XXX|BUG)[:\s-]*\s*(.+)").unwrap();

            if let Some(caps) = todo_pattern.captures(docstring) {
                let line_num = content[..mat.start()].lines().count();
                todos.push(ParsedTodo {
                    raw_content: docstring.to_string(),
                    clean_content: caps.get(2).unwrap().as_str().trim().to_string(),
                    file_path: file_path.clone(),
                    line_number: line_num,
                    column_start: 0,
                    column_end: docstring.len(),
                    language: "python".to_string(),
                    todo_type: TodoType::Todo,
                    tags: vec![],
                    priority: crate::core::models::Priority::None,
                    author: None,
                    linked_issues: vec![],
                    created_at: None,
                    parser_name: "Python",
                });
            }
        }
    }
}
```

#### 任务 3.3.4：实现 Rust 解析器

**文件**: `src/parsers/rust.rs`

**任务描述**：
- 实现 Rust TODO 解析
- 支持 `// TODO` 和 `//!`、`///` 文档注释
- 解析 `// TODO(high)` 优先级格式

**实现步骤**：

```rust
// src/parsers/rust.rs

use super::{Parser, ParsedTodo, TodoType};
use regex::Regex;
use std::path::PathBuf;

pub struct RustParser;

impl Parser for RustParser {
    fn name(&self) -> &'static str {
        "Rust"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["rs"]
    }

    fn shebangs(&self) -> Option<&'static [&'static str]> {
        None
    }

    fn parse(&self, content: &str, file_path: &PathBuf) -> Vec<ParsedTodo> {
        let mut todos = Vec::new();

        // 标准单行注释
        let single_line = Regex::new(
            r"//\s*(TODO|FIXME|XXX|BUG|HACK|NOTE|TODO\(([^)]+)\))[:\s]*(.*)$"
        ).unwrap();

        // 内部文档 //! 和外部文档 ///
        let inner_doc = Regex::new(r"//!\s*(TODO|FIXME|XXX|BUG)[:\s]*(.*)$").unwrap();
        let outer_doc = Regex::new(r"///\s*(TODO|FIXME|XXX|BUG)[:\s]*(.*)$").unwrap();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = outer_doc.captures(line) {
                todos.push(self.build_parsed_todo(
                    line,
                    line_num + 1,
                    file_path,
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str(),
                ));
            } else if let Some(caps) = inner_doc.captures(line) {
                todos.push(self.build_parsed_todo(
                    line,
                    line_num + 1,
                    file_path,
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str(),
                ));
            } else if let Some(caps) = single_line.captures(line) {
                todos.push(self.build_parsed_todo(
                    line,
                    line_num + 1,
                    file_path,
                    caps.get(1).unwrap().as_str(),
                    caps.get(3).unwrap().as_str(),
                ));
            }
        }

        todos
    }
}
```

#### 任务 3.3.5：实现语言检测器

**文件**: `src/parsers/language.rs`

**任务描述**：
- 实现基于扩展名的语言检测
- 实现基于 Shebang 的检测
- 实现基于内容特征的检测

**验收标准**：
- [ ] 准确识别 30+ 种语言
- [ ] Shebang 检测 Shell 脚本
- [ ] 检测 JSON、YAML 等配置文件

**实现步骤**：

```rust
// src/parsers/language.rs

use std::path::PathBuf;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    JavaScript,
    TypeScript,
    Python,
    Rust,
    Go,
    Java,
    C,
    Cpp,
    Ruby,
    PHP,
    Shell,
    Markdown,
    HTML,
    YAML,
    JSON,
    CSS,
    Vue,
    Svelte,
    Unknown,
}

pub struct LanguageDetector {
    extension_map: HashMap<&'static str, Language>,
    shebang_map: HashMap<&'static str, Language>,
}

impl LanguageDetector {
    pub fn new() -> Self {
        let mut extension_map = HashMap::new();

        // 填充扩展名映射
        extension_map.insert("js", Language::JavaScript);
        extension_map.insert("jsx", Language::JavaScript);
        extension_map.insert("ts", Language::TypeScript);
        extension_map.insert("tsx", Language::TypeScript);
        extension_map.insert("py", Language::Python);
        extension_map.insert("rs", Language::Rust);
        extension_map.insert("go", Language::Go);
        extension_map.insert("java", Language::Java);
        extension_map.insert("rb", Language::Ruby);
        extension_map.insert("php", Language::PHP);
        extension_map.insert("sh", Language::Shell);
        extension_map.insert("md", Language::Markdown);
        extension_map.insert("html", Language::HTML);
        extension_map.insert("yml", Language::YAML);
        extension_map.insert("yaml", Language::YAML);
        extension_map.insert("json", Language::JSON);
        extension_map.insert("css", Language::CSS);
        extension_map.insert("vue", Language::Vue);
        extension_map.insert("svelte", Language::Svelte);
        extension_map.insert("c", Language::C);
        extension_map.insert("cpp", Language::Cpp);
        extension_map.insert("h", Language::C);
        extension_map.insert("hpp", Language::Cpp);

        let mut shebang_map = HashMap::new();
        shebang_map.insert("#!/usr/bin/python", Language::Python);
        shebang_map.insert("#!/usr/bin/env python", Language::Python);
        shebang_map.insert("#!/usr/bin/node", Language::JavaScript);
        shebang_map.insert("#!/usr/bin/env node", Language::JavaScript);
        shebang_map.insert("#!/bin/bash", Language::Shell);
        shebang_map.insert("#!/usr/bin/bash", Language::Shell);
        shebang_map.insert("#!/usr/bin/env bash", Language::Shell);
        shebang_map.insert("#!/usr/bin/php", Language::PHP);
        shebang_map.insert("#!/usr/bin/env php", Language::PHP);

        Self {
            extension_map,
            shebang_map,
        }
    }

    pub fn detect(&self, file_path: &PathBuf, content: Option<&str>) -> Language {
        // 1. 扩展名检测
        if let Some(ext) = self.get_extension(file_path) {
            if let Some(lang) = self.extension_map.get(ext.to_lowercase().as_str()) {
                return *lang;
            }
        }

        // 2. Shebang 检测
        if let Some(first_line) = content.and_then(|c| c.lines().next()) {
            let trimmed = first_line.trim();
            if let Some(shebang) = trimmed.strip_prefix("#!") {
                let shebang = shebang.trim();
                for (prefix, lang) in &self.shebang_map {
                    if shebang.starts_with(prefix) {
                        return *lang;
                    }
                }
            }
        }

        // 3. 内容检测
        if let Some(content) = content {
            if content.starts_with("{") && content.ends_with("}") {
                return Language::JSON;
            }
            if content.trim_start().starts_with("---") {
                return Language::YAML;
            }
        }

        Language::Unknown
    }

    fn get_extension(&self, file_path: &PathBuf) -> Option<&str> {
        file_path.extension()?.to_str()
    }
}
```

#### 任务 3.3.6：实现解析器注册表

**文件**: `src/parsers/registry.rs`

**任务描述**：
- 实现解析器注册表
- 管理所有语言解析器
- 支持动态注册

**验收标准**：
- [ ] 自动注册所有内置解析器
- [ ] 支持按语言获取解析器
- [ ] 支持按扩展名获取解析器
- [ ] 支持自定义解析器注册

**实现步骤**：

```rust
// src/parsers/registry.rs

use super::{Parser, Language};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ParserRegistry {
    by_name: HashMap<&'static str, Arc<dyn Parser>>,
    by_extension: HashMap<&'static str, Arc<dyn Parser>>,
    by_language: HashMap<Language, Arc<dyn Parser>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            by_name: HashMap::new(),
            by_extension: HashMap::new(),
            by_language: HashMap::new(),
        };

        // 注册内置解析器
        registry.register(Arc::new(super::JavaScriptParser));
        registry.register(Arc::new(super::TypeScriptParser));
        registry.register(Arc::new(super::PythonParser));
        registry.register(Arc::new(super::RustParser));
        registry.register(Arc::new(super::GoParser));
        registry.register(Arc::new(super::JavaParser));
        registry.register(Arc::new(super::C_FamilyParser));
        registry.register(Arc::new(super::RubyParser));
        registry.register(Arc::new(super::PHParser));
        registry.register(Arc::new(super::ShellParser));
        registry.register(Arc::new(super::MarkdownParser));
        registry.register(Arc::new(super::HTMLParser));
        registry.register(Arc::new(super::YAMLParser));
        registry.register(Arc::new(super::JSONParser));
        registry.register(Arc::new(super::VueParser));

        registry
    }

    pub fn register(&mut self, parser: Arc<dyn Parser>) {
        self.by_name.insert(parser.name(), Arc::clone(&parser));

        for ext in parser.extensions() {
            self.by_extension.insert(ext, Arc::clone(&parser));
        }
    }

    pub fn get_by_name(&self, name: &str) -> Option<&dyn Parser> {
        self.by_name.get(name).map(|p| p.as_ref())
    }

    pub fn get_by_extension(&self, ext: &str) -> Option<&dyn Parser> {
        self.by_extension.get(ext).map(|p| p.as_ref())
    }

    pub fn get_by_language(&self, language: Language) -> Option<&dyn Parser> {
        self.by_language.get(&language).map(|p| p.as_ref())
    }

    pub fn list_all(&self) -> Vec<&'static str> {
        self.by_name.keys().cloned().collect()
    }
}
```

#### 任务 3.3.7：实现增量解析

**文件**: `src/parsers/incremental.rs`

**任务描述**：
- 实现增量解析机制
- 检测文件变更（新增、修改、删除 TODO）
- 减少重复解析开销

**验收标准**：
- [ ] 检测 TODO 新增
- [ ] 检测 TODO 内容修改
- [ ] 检测 TODO 删除
- [ ] 增量更新耗时 < 100ms

**实现步骤**：

```rust
// src/parsers/incremental.rs

use super::ParsedTodo;
use std::path::PathBuf;
use std::collections::HashMap;

pub struct IncrementalParser {
    previous_hashes: HashMap<(PathBuf, usize), String>,
}

impl IncrementalParser {
    pub fn new() -> Self {
        Self {
            previous_hashes: HashMap::new(),
        }
    }

    pub fn parse_incremental(
        &mut self,
        old_content: &str,
        new_content: &str,
        file_path: &PathBuf,
    ) -> IncrementalResult {
        let old_lines: Vec<&str> = old_content.lines().collect();
        let new_lines: Vec<&str> = new_content.lines().collect();

        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut modified = Vec::new();

        // 计算每行的哈希
        let old_hashes: HashMap<usize, String> = old_lines
            .iter()
            .enumerate()
            .map(|(i, line)| (i, self.hash_line(line)))
            .collect();

        let new_hashes: HashMap<usize, String> = new_lines
            .iter()
            .enumerate()
            .map(|(i, line)| (i, self.hash_line(line)))
            .collect();

        // 检测新增行
        for (i, hash) in &new_hashes {
            if !old_hashes.contains_key(i) {
                added.push(*i);
            } else if old_hashes[i] != *hash {
                modified.push(*i);
            }
        }

        // 检测删除行
        for (i, _) in &old_hashes {
            if !new_hashes.contains_key(i) {
                removed.push(*i);
            }
        }

        IncrementalResult { added, removed, modified }
    }

    fn hash_line(&self, line: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(line.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub struct IncrementalResult {
    pub added: Vec<usize>,
    pub removed: Vec<usize>,
    pub modified: Vec<usize>,
}
```

#### 任务 3.3.8：实现标准模式库

**文件**: `src/parsers/common/patterns.rs`

**任务描述**：
- 定义标准 TODO 模式
- 支持 15+ 种类型（TODO、FIXME、BUG 等）
- 支持优先级解析

**实现步骤**：

```rust
// src/parsers/common/patterns.rs

use super::{TodoType, Priority};

pub struct StandardPatterns;

impl StandardPatterns {
    pub fn todo_keywords() -> &'static [(&'static str, TodoType)] {
        &[
            ("TODO", TodoType::Todo),
            ("FIXME", TodoType::Fixme),
            ("XXX", TodoType::Bug),
            ("BUG", TodoType::Bug),
            ("HACK", TodoType::Optimize),
            ("NOTE", TodoType::Doc),
            ("REVIEW", TodoType::Todo),
            ("CLEANUP", TodoType::Optimize),
            ("DEBUG", TodoType::Bug),
            ("TEMP", TodoType::Todo),
            ("PERF", TodoType::Optimize),
            ("SECURITY", TodoType::Security),
            ("FEATURE", TodoType::Feature),
            ("DOC", TodoType::Doc),
            ("QUESTION", TodoType::Question),
        ]
    }

    pub fn priority_keywords() -> &'static [(&'static str, Priority)] {
        &[
            ("URGENT", Priority::Urgent),
            ("CRITICAL", Priority::Urgent),
            ("HIGH", Priority::High),
            ("IMPORTANT", Priority::High),
            ("MEDIUM", Priority::Medium),
            ("NORMAL", Priority::Medium),
            ("LOW", Priority::Low),
            ("MINOR", Priority::Low),
            ("TRIVIAL", Priority::Low),
        ]
    }
}
```

#### 任务 3.3.9：实现标签提取器

**文件**: `src/parsers/common/tag_extractor.rs`

**任务描述**：
- 从 TODO 内容提取标签
- 支持多种格式 `@tag` `[tag]` `:tag:`

**实现步骤**：

```rust
// src/parsers/common/tag_extractor.rs

use regex::Regex;

pub struct TagExtractor;

impl TagExtractor {
    pub fn extract(content: &str) -> Vec<String> {
        let mut tags = Vec::new();

        // @tagname 格式
        let at_pattern = Regex::new(r"@(\w+)").unwrap();
        for cap in at_pattern.captures_iter(content) {
            tags.push(cap.get(1).unwrap().as_str().to_string());
        }

        // [tagname] 格式
        let bracket_pattern = Regex::new(r"\[(\w+)\]").unwrap();
        for cap in bracket_pattern.captures_iter(content) {
            tags.push(cap.get(1).unwrap().as_str().to_string());
        }

        // :tagname: 格式（Rust 风格）
        let colon_pattern = Regex::new(r":(\w+):").unwrap();
        for cap in colon_pattern.captures_iter(content) {
            tags.push(cap.get(1).unwrap().as_str().to_string());
        }

        tags
    }
}
```

#### 任务 3.3.10：实现文件扫描器

**文件**: `src/scan/walker.rs`

**任务描述**：
- 实现目录遍历
- 支持并发扫描
- 支持忽略规则

**验收标准**：
- [ ] 深度遍历目录
- [ ] 支持 `node_modules/` `.git/` 等忽略
- [ ] 支持 `.teammateignore` 自定义规则
- [ ] 10K 文件 < 3 秒

**实现步骤**：

```rust
// src/scan/walker.rs

use ignore::{WalkBuilder, DirEntry};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use crate::scan::IgnoreManager;

pub struct FileScanner {
    ignore_manager: Arc<IgnoreManager>,
    max_concurrent: usize,
}

impl FileScanner {
    pub fn new(ignore_manager: Arc<IgnoreManager>) -> Self {
        Self {
            ignore_manager,
            max_concurrent: num_cpus::get(),
        }
    }

    pub fn scan(&self, root: &PathBuf) -> Vec<PathBuf> {
        let mut files = Vec::new();

        let walker = WalkBuilder::new(root)
            .standard_filters(true)
            .git_global(true)
            .git_ignore(true)
            .build();

        for entry in walker {
            match entry {
                Ok(entry) => {
                    if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                        let path = entry.path().to_path_buf();
                        if !self.ignore_manager.should_ignore(&path) {
                            files.push(path);
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Error walking path: {}", e);
                }
            }
        }

        files
    }

    pub fn scan_parallel(&self, roots: &[PathBuf]) -> Vec<PathBuf> {
        roots.par_iter()
            .flat_map(|root| self.scan(root))
            .collect()
    }
}
```

---

### 3.4 Git 集成

#### 任务 3.4.1：实现 Git 服务封装

**文件**: `src/git/mod.rs`, `src/git/blame.rs`, `src/git/log.rs`

**任务描述**：
- 使用 git2-rs 封装 Git 操作
- 实现 blame 信息获取
- 实现日志分析

**验收标准**：
- [ ] 自动检测 Git 仓库
- [ ] 获取指定文件的 blame 信息
- [ ] 解析 commit 元数据
- [ ] 缓存 Git 数据

**实现步骤**：

```rust
// src/git/blame.rs

use git2::{Repository, Blame, BlameHunk};
use std::path::Path;
use anyhow::{Result, Context};

pub struct GitBlame {
    repo: Repository,
}

impl GitBlame {
    pub fn new(path: &Path) -> Result<Self> {
        let repo = Repository::discover(path)
            .context("Failed to discover git repository")?;
        Ok(Self { repo })
    }

    pub fn get_blame(&self, file_path: &Path) -> Result<Vec<BlameHunk>> {
        let blame = self.repo
            .blame_file(file_path)?
            .into_iter()
            .collect();

        Ok(blame)
    }

    pub fn get_line_blame(&self, file_path: &Path, line: u32) -> Result<Option<BlameHunk>> {
        let blame = self.repo.blame_file(file_path)?;
        let hunk = blame.get_line(line);
        Ok(hunk)
    }

    pub fn get_blame_for_todo(&self, file_path: &Path, line: u32) -> Result<BlameInfo> {
        if let Some(hunk) = self.get_line_blame(file_path, line)? {
            let commit = self.repo.find_commit(hunk.commit_id())?;

            Ok(BlameInfo {
                commit_hash: hunk.commit_id().to_string(),
                author_name: commit.author().name().unwrap_or("Unknown").to_string(),
                author_email: commit.author().email().unwrap_or("Unknown").to_string(),
                timestamp: commit.time(),
                summary: commit.summary().unwrap_or("").to_string(),
            })
        } else {
            Ok(BlameInfo::default())
        }
    }
}

#[derive(Debug, Default)]
pub struct BlameInfo {
    pub commit_hash: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: git2::Time,
    pub summary: String,
}
```

#### 任务 3.4.2：实现分支 TODO 追踪

**文件**: `src/git/branch.rs`

**任务描述**：
- 追踪分支引入的 TODO
- 分支间 TODO 比较
- 合并时 TODO 处理

**验收标准**：
- [ ] 识别 TODO 首次出现的分支
- [ ] `teammate branch` 显示分支 TODO
- [ ] `teammate branch compare` 比较分支

**实现步骤**：

```rust
// src/git/branch.rs

use git2::{Repository, Branch, Commit};
use std::path::Path;
use anyhow::Result;
use std::collections::HashMap;

pub struct BranchTodoTracker {
    repo: Repository,
}

impl BranchTodoTracker {
    pub fn new(path: &Path) -> Result<Self> {
        let repo = Repository::discover(path)?;
        Ok(Self { repo })
    }

    pub fn get_current_branch_name(&self) -> Result<String> {
        let head = self.repo.head()?;
        let branch_name = head.shorthand();
        Ok(branch_name.to_string())
    }

    pub fn get_branch_todos(&self, branch_name: &str) -> Result<Vec<BranchTodo>> {
        // 获取分支的 TODO
        // 追溯 TODO 的来源分支
    }

    pub fn compare_branches(&self, source: &str, target: &str) -> Result<BranchComparison> {
        // 比较两个分支的 TODO 差异
    }
}

pub struct BranchTodo {
    pub id: String,
    pub content: String,
    pub file: String,
    pub line: u32,
    pub introduced_branch: String,
    pub introduced_commit: String,
    pub is_merged: bool,
}

pub struct BranchComparison {
    pub source: String,
    pub target: String,
    pub only_in_source: Vec<BranchTodo>,
    pub only_in_target: Vec<BranchTodo>,
    pub common: Vec<BranchTodo>,
}
```

#### 任务 3.4.3：实现 Git Hooks 管理

**文件**: `src/git/hooks/installer.rs`

**任务描述**：
- 实现 hooks 安装/卸载
- 实现 pre-commit、post-commit 等钩子
- 阻塞关键 TODO 提交

**验收标准**：
- [ ] `teammate hooks install` 安装 hooks
- [ ] pre-commit 检查紧急 TODO
- [ ] post-commit 更新 TODO 关联
- [ ] 保留原有 hooks

**实现步骤**：

```rust
// src/git/hooks/installer.rs

use std::path::Path;
use std::fs;
use anyhow::{Result, Context};
use crate::git::hooks::HookType;

pub struct HooksInstaller {
    repo_path: PathBuf,
}

impl HooksInstaller {
    pub fn new(repo_path: &Path) -> Self {
        Self {
            repo_path: repo_path.to_path_buf(),
        }
    }

    pub fn install_all(&self) -> Result<()> {
        for hook_type in HookType::all() {
            self.install_hook(hook_type)?;
        }
        Ok(())
    }

    pub fn install_hook(&self, hook_type: HookType) -> Result<()> {
        let hook_path = self.repo_path
            .join(".git/hooks")
            .join(hook_type.name());

        let script = self.generate_hook_script(hook_type)?;

        fs::write(&hook_path, script)
            .with_context(|| format!("Failed to write hook: {:?}", hook_path))?;

        // 设置可执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)?;
        }

        Ok(())
    }

    fn generate_hook_script(&self, hook_type: HookType) -> Result<String> {
        match hook_type {
            HookType::PreCommit => Ok(r#"#!/bin/bash
# Teammate pre-commit hook

# 检查是否有未解决的阻塞 TODO
teammate hook check-blockers --hook-script
exit $?
"#.to_string()),
            HookType::PostCommit => Ok(r#"#!/bin/bash
# Teammate post-commit hook

# 更新 TODO 关联的 commit
teammate hook update-commit --hook-script
"#.to_string()),
            _ => Ok(String::new()),
        }
    }
}
```

---

### 3.5 TUI 界面

#### 任务 3.5.1：实现 TUI 应用框架

**文件**: `src/tui/app.rs`, `src/tui/events.rs`

**任务描述**：
- 使用 ratatui 实现 TUI 框架
- 实现事件循环
- 实现主状态机

**验收标准**：
- [ ] `teammate --tui` 启动 TUI
- [ ] 响应键盘事件
- [ ] 支持鼠标事件（可选）
- [ ] 优雅退出

**实现步骤**：

```rust
// src/tui/app.rs

use ratatui::{Terminal, backend::CrosstermBackend};
use crossterm::{
    event::{self, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io;
use crate::core::state::AppState;

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    state: AppState,
    should_quit: bool,
}

impl TuiApp {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All))?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            state: AppState::new(),
            should_quit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.should_quit {
            self.draw()?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => self.handle_key(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        match key.code {
            event::KeyCode::Char('q') => self.should_quit = true,
            event::KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            // 渲染主界面
        })?;
        Ok(())
    }

    pub fn shutdown(&mut self) {
        disable_raw_mode().ok();
    }
}

impl Drop for TuiApp {
    fn drop(&mut self) {
        self.shutdown();
    }
}
```

#### 任务 3.5.2：实现 TODO 列表组件

**文件**: `src/tui/components/list.rs`

**任务描述**：
- 实现 TODO 列表展示
- 支持键盘导航
- 支持选择和批量操作

**验收标准**：
- [ ] 显示 TODO 列表
- [ ] 上下键导航
- [ ] Enter 查看详情
- [ ] 空格选择

**实现步骤**：

```rust
// src/tui/components/list.rs

use ratatui::{
    widgets::{Table, TableState, Row, Cell},
    layout::{Constraint, Rect},
    Frame,
};
use crate::core::models::Todo;

pub struct TodoList {
    state: TableState,
    todos: Vec<Todo>,
    selected: Option<usize>,
}

impl TodoList {
    pub fn new(todos: Vec<Todo>) -> Self {
        Self {
            state: TableState::default(),
            todos,
            selected: None,
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let header = ["ID", "Priority", "Title", "File", "Line"];

        let rows: Vec<Row> = self.todos
            .iter()
            .map(|todo| Row::new(vec![
                Cell::from(todo.id.clone()),
                Cell::from(format!("{:?}", todo.priority)),
                Cell::from(todo.content.clone()),
                Cell::from(todo.file_path.to_string_lossy()),
                Cell::from(todo.line_number.to_string()),
            ]))
            .collect();

        let table = Table::new(rows)
            .header(header)
            .widths(&[
                Constraint::Length(36),
                Constraint::Length(10),
                Constraint::Percentage(50),
                Constraint::Percentage(30),
                Constraint::Length(8),
            ])
            .column_spacing(1)
            .block(ratatui::widgets::Block::default().title("TODOs"))
            .highlight_style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD))
            .row_highlight_style(ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray));

        f.render_stateful_widget(table, area, &mut self.state);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.todos.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.todos.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_todo(&self) -> Option<&Todo> {
        self.state.selected().and_then(|i| self.todos.get(i))
    }
}
```

#### 任务 3.5.3：实现过滤器面板

**文件**: `src/tui/components/filter.rs`

**任务描述**：
- 实现过滤条件设置
- 支持状态、优先级、标签过滤
- 实时更新列表

**实现步骤**：

```rust
// src/tui/components/filter.rs

use ratatui::{widgets::{Block, Paragraph, Borders}, style::{Style, Color}, layout::Rect};
use crate::core::models::{TodoStatus, Priority};

pub struct FilterPanel {
    pub status_filter: Option<TodoStatus>,
    pub priority_filter: Option<Priority>,
    pub tag_filter: Option<String>,
    pub search_query: String,
}

impl FilterPanel {
    pub fn new() -> Self {
        Self {
            status_filter: None,
            priority_filter: None,
            tag_filter: None,
            search_query: String::new(),
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let content = format!(
            r#"Filter
Status: {:?}
Priority: {:?}
Tag: {:?}
Search: {}"#,
            self.status_filter.as_ref().unwrap_or(&TodoStatus::Open),
            self.priority_filter.as_ref().unwrap_or(&Priority::None),
            self.tag_filter.as_ref().map(|s| s.as_str()).unwrap_or("All"),
            if self.search_query.is_empty() { "..." } else { &self.search_query }
        );

        let block = Block::default()
            .title("Filters")
            .borders(ratatui::widgets::Borders::ALL);

        f.render_widget(
            Paragraph::new(content)
                .block(block)
                .style(Style::default().fg(Color::White)),
            area,
        );
    }
}
```

#### 任务 3.5.4：实现详情面板

**文件**: `src/tui/components/detail.rs`

**任务描述**：
- 显示 TODO 详细信息
- 支持编辑操作

**实现步骤**：

```rust
// src/tui/components/detail.rs

use ratatui::{widgets::{Block, Paragraph, Borders, List, ListItem}, style::{Style, Color}, layout::Rect, Frame, text::Text};
use crate::core::models::Todo;

pub struct DetailPanel {
    todo: Option<Todo>,
}

impl DetailPanel {
    pub fn new() -> Self {
        Self { todo: None }
    }

    pub fn set_todo(&mut self, todo: Option<Todo>) {
        self.todo = todo;
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let content = match &self.todo {
            Some(todo) => {
                format!(
                    r#"ID: {}
Title: {}
Status: {:?}
Priority: {:?}
File: {}:{}
Created: {}
Updated: {}
Tags: {}
Linked Issues: {}"#,
                    todo.id,
                    todo.content,
                    todo.status,
                    todo.priority,
                    todo.file_path.display(),
                    todo.line_number,
                    todo.created_at,
                    todo.updated_at,
                    todo.tags.join(", "),
                    todo.linked_issues.join(", ")
                )
            }
            None => "No TODO selected".to_string(),
        };

        let block = Block::default()
            .title("Details")
            .borders(ratatui::widgets::Borders::ALL);

        f.render_widget(
            Paragraph::new(content)
                .block(block)
                .scrollable(true),
            area,
        );
    }
}
```

#### 任务 3.5.5：实现快捷键绑定

**文件**: `src/tui/keymap.rs`

**任务描述**：
- 定义 TUI 快捷键
- 实现快捷键分发

**验收标准**：
- [ ] Vim 风格导航（hjkl）
- [ ] 常见操作快捷键
- [ ] 可配置快捷键

**实现步骤**：

```rust
// src/tui/keymap.rs

use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use crate::tui::actions::Action;

#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub key: KeyEvent,
    pub action: Action,
    pub description: &'static str,
}

pub struct KeyMap {
    bindings: Vec<KeyBinding>,
}

impl KeyMap {
    pub fn new() -> Self {
        Self {
            bindings: Self::default_bindings(),
        }
    }

    fn default_bindings() -> Vec<KeyBinding> {
        vec![
            // 导航
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
                action: Action::NextItem,
                description: "Move down",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
                action: Action::PreviousItem,
                description: "Move up",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
                action: Action::PreviousColumn,
                description: "Move left",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
                action: Action::NextColumn,
                description: "Move right",
            },
            // 操作
            KeyBinding {
                key: KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
                action: Action::ViewDetail,
                description: "View details",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
                action: Action::AddTodo,
                description: "Add TODO",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
                action: Action::EditTodo,
                description: "Edit TODO",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
                action: Action::DeleteTodo,
                description: "Delete TODO",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
                action: Action::ToggleSelect,
                description: "Toggle selection",
            },
            // 过滤
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE),
                action: Action::OpenFilter,
                description: "Open filter",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE),
                action: Action::Search,
                description: "Search",
            },
            // 系统
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
                action: Action::Quit,
                description: "Quit",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
                action: Action::Quit,
                description: "Quit",
            },
            KeyBinding {
                key: KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE),
                action: Action::ShowHelp,
                description: "Show help",
            },
        ]
    }

    pub fn find_action(&self, key: &KeyEvent) -> Option<&Action> {
        self.bindings
            .iter()
            .find(|b| b.key == *key)
            .map(|b| &b.action)
    }
}
```

#### 任务 3.5.6：实现颜色主题

**文件**: `src/tui/theme.rs`

**任务描述**：
- 定义 TUI 颜色主题
- 支持亮色/暗色模式
- 可配置主题

**验收标准**：
- [ ] 默认暗色主题
- [ ] 支持亮色主题
- [ ] 主题可配置

**实现步骤**：

```rust
// src/tui/theme.rs

use ratatui::style::{Color, Style, Modifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub colors: ThemeColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub highlight: Color,

    // 优先级颜色
    pub priority_urgent: Color,
    pub priority_high: Color,
    pub priority_medium: Color,
    pub priority_low: Color,

    // 状态颜色
    pub status_open: Color,
    pub status_progress: Color,
    pub status_resolved: Color,

    // UI 元素
    pub border: Color,
    pub header: Color,
    pub selection: Color,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            name: "catppuccin-mocha".to_string(),
            colors: ThemeColors {
                background: Color::Rgb(30, 30, 46),
                foreground: Color::Rgb(205, 214, 244),
                primary: Color::Rgb(137, 180, 250),
                secondary: Color::Rgb(180, 190, 254),
                accent: Color::Rgb(148, 226, 213),
                highlight: Color::Rgb(49, 50, 68),

                priority_urgent: Color::Rgb(243, 139, 168),
                priority_high: Color::Rgb(243, 139, 168),
                priority_medium: Color::Rgb(249, 226, 175),
                priority_low: Color:: Rgb(166, 227, 161),

                status_open: Color::Rgb(243, 139, 168),
                status_progress: Color::Rgb(249, 226, 175),
                status_resolved: Color::Rgb(166, 227, 161),

                border: Color::Rgb(108, 112, 134),
                header: Color::Rgb(137, 180, 250),
                selection: Color::Rgb(49, 50, 68),
            },
        }
    }
}
```

---

### 3.6 测试策略

#### 任务 3.6.1：编写解析器单元测试

**文件**: `tests/unit/parsers/*.rs`

**任务描述**：
- 测试各语言解析器
- 测试边界情况
- 测试标签提取

**验收标准**：
- [ ] 核心解析器 90%+ 覆盖率
- [ ] 覆盖常见边界情况
- [ ] 使用 rstest 参数化测试

**实现步骤**：

```rust
// tests/unit/parsers/javascript.rs

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::parsers::JavaScriptParser;
    use std::path::PathBuf;

    #[rstest]
    #[case("// TODO: simple todo", "simple todo")]
    #[case("// TODO(username): with author", "with author")]
    #[case("// TODO(high): high priority", "high priority")]
    #[case("/* TODO: block comment */", "block comment")]
    #[case("/// @todo typescript doc", "typescript doc")]
    fn test_javascript_parser(
        #[case] input: &str,
        #[case] expected_content: &str,
    ) {
        let parser = JavaScriptParser;
        let path = PathBuf::from("test.js");
        let todos = parser.parse(input, &path);

        assert_eq!(todos.len(), 1);
        assert!(todos[0].content.contains(expected_content));
    }

    #[test]
    fn test_multiline_todo() {
        let content = r#"/*
         * TODO:
         *   - Task one
         *   - Task two
         */"#;
        let parser = JavaScriptParser;
        let path = PathBuf::from("test.js");
        let todos = parser.parse(content, &path);

        assert!(!todos.is_empty());
    }

    #[test]
    fn test_no_todo() {
        let content = r#"fn main() {
    println!("Hello");
}"#;
        let parser = JavaScriptParser;
        let path = PathBuf::from("test.js");
        let todos = parser.parse(content, &path);

        assert!(todos.is_empty());
    }
}
```

#### 任务 3.6.2：编写 CLI 集成测试

**文件**: `tests/integration/cli_commands.rs`

**任务描述**：
- 测试 CLI 命令
- 使用 assert_cmd
- 测试临时目录

**验收标准**：
- [ ] 每个命令至少一个测试
- [ ] 测试参数验证
- [ ] 测试错误处理

**实现步骤**：

```rust
// tests/integration/cli_commands.rs

use assert_cmd::Command;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_scan_command() {
    let temp_dir = TempDir::new().unwrap();

    // 创建测试文件
    fs::write(
        temp_dir.path().join("test.rs"),
        r#"// TODO: test todo"#,
    ).unwrap();

    let mut cmd = Command::cargo_bin("teammate").unwrap();
    cmd.arg("scan")
       .arg(temp_dir.path())
       .assert()
       .success()
       .stdout(predicate::str::contains("test todo"));
}

#[test]
fn test_list_command() {
    let mut cmd = Command::cargo_bin("teammate").unwrap();
    cmd.arg("list")
       .assert()
       .success();
}

#[test]
fn test_add_command() {
    let mut cmd = Command::cargo_bin("teammate").unwrap();
    cmd.arg("add")
       .arg("test todo")
       .arg("--priority")
       .arg("high")
       .assert()
       .success();
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("teammate").unwrap();
    cmd.arg("invalid-command")
       .assert()
       .failure()
       .stderr(predicate::str::contains("unknown command"));
}
```

#### 任务 3.6.3：配置 GitHub Actions CI

**文件**: `.github/workflows/ci.yml`

**任务描述**：
- 配置 CI 流水线
- 多平台测试
- 代码覆盖率

**验收标准**：
- [ ] CI 通过所有测试
- [ ] 多平台测试通过
- [ ] 覆盖率报告

**实现步骤**：

```yaml
# .github/workflows/ci.yml

name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          components: rustfmt, clippy

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Run Clippy
        run: cargo clippy --all-features -- -D warnings

      - name: Check docs
        run: cargo doc --no-deps

  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
        exclude:
          - os: windows-latest
            rust: beta

    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust ${{ matrix.rust }}
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          profile: minimal

      - name: Run tests
        run: cargo test --all-features --all

      - name: Upload coverage
        if: matrix.rust == 'stable' && matrix.os == 'ubuntu-latest'
        uses: codecov/codecov-action@v3
        with:
          files: ./target/debug/coverage.lcov
```

---

## 4. 数据模型详细定义

### 4.1 核心实体

```sql
-- projects 项目表
CREATE TABLE projects (
    id TEXT PRIMARY KEY,           -- UUID
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    repository_url TEXT,
    root_path TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    is_active INTEGER NOT NULL DEFAULT 1,
    settings TEXT                   -- JSON 配置
);

-- todos 待办事项表
CREATE TABLE todos (
    id TEXT PRIMARY KEY,           -- UUID
    project_id TEXT,
    content TEXT NOT NULL,
    description TEXT,
    file_path TEXT NOT NULL,
    line_number INTEGER NOT NULL,
    end_line INTEGER,
    language TEXT,
    code_context TEXT,
    status TEXT NOT NULL DEFAULT 'open'
        CHECK(status IN ('open', 'in_progress', 'resolved', 'closed', 'archived')),
    priority TEXT NOT NULL DEFAULT 'medium'
        CHECK(priority IN ('low', 'medium', 'high', 'urgent', 'none')),
    due_date TEXT,
    author TEXT,
    assignee TEXT,
    linked_issues TEXT,             -- JSON 数组
    linked_commits TEXT,            -- JSON 数组
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    started_at TEXT,
    completed_at TEXT,
    last_scanned_at TEXT,
    is_manual INTEGER NOT NULL DEFAULT 0,
    is_ignored INTEGER NOT NULL DEFAULT 0,
    ignore_reason TEXT,
    metadata TEXT,                  -- JSON
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- tags 标签表
CREATE TABLE tags (
    id TEXT PRIMARY KEY,           -- UUID
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT '#6B7280',
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    usage_count INTEGER NOT NULL DEFAULT 0,
    category TEXT,
    is_system INTEGER NOT NULL DEFAULT 0,
    project_id TEXT,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- todo_tags 多对多关联表
CREATE TABLE todo_tags (
    todo_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (todo_id, tag_id),
    FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
```

### 4.2 索引设计

```sql
-- 常用查询索引
CREATE INDEX idx_todos_project ON todos(project_id);
CREATE INDEX idx_todos_status ON todos(status);
CREATE INDEX idx_todos_priority ON todos(priority);
CREATE INDEX idx_todos_file ON todos(file_path);
CREATE INDEX idx_todos_author ON todos(author);
CREATE INDEX idx_todos_due_date ON todos(due_date);
CREATE INDEX idx_todos_created_at ON todos(created_at);

-- 复合索引
CREATE INDEX idx_todos_project_status ON todos(project_id, status);
CREATE INDEX idx_todos_project_priority ON todos(project_id, priority DESC);
CREATE INDEX idx_todos_open_priority ON todos(status, priority DESC) WHERE status = 'open';

-- 标签索引
CREATE INDEX idx_todo_tags_todo ON todo_tags(todo_id);
CREATE INDEX idx_todo_tags_tag ON todo_tags(tag_id);
CREATE INDEX idx_tags_name ON tags(name);
```

---

## 5. API 接口规范

### 5.1 核心 API

```rust
// Scanner API
pub trait TodoScanner {
    fn scan(&self, options: &ScanOptions) -> Result<ScanResult>;
    fn scan_file(&self, path: &PathBuf) -> Result<Vec<ParsedTodo>>;
    fn scan_content(&self, content: &str, path: &PathBuf) -> Vec<ParsedTodo>;
}

// Storage API
pub trait Storage {
    // Todo operations
    fn create_todo(&self, todo: &Todo) -> Result<()>;
    fn get_todo(&self, id: &str) -> Result<Option<Todo>>;
    fn update_todo(&self, todo: &Todo) -> Result<()>;
    fn delete_todo(&self, id: &str) -> Result<()>;
    fn list_todos(&self, filter: &TodoFilter) -> Result<Vec<Todo>>;

    // Tag operations
    fn create_tag(&self, tag: &Tag) -> Result<()>;
    fn list_tags(&self) -> Result<Vec<Tag>>;

    // Batch operations
    fn bulk_insert_todos(&self, todos: &[Todo]) -> Result<usize>;
}

// Git API
pub trait GitService {
    fn is_repo(&self) -> bool;
    fn get_blame(&self, file: &Path, line: u32) -> Result<Option<BlameInfo>>;
    fn get_current_branch(&self) -> Result<String>;
    fn get_branches(&self) -> Result<Vec<String>>;
}
```

---

## 6. 配置文件格式

### 6.1 主配置文件

```yaml
# ~/.teammate/config.yaml

app:
  name: "teammate"
  version: "0.1.0"
  data_dir: "~/.teammate"
  log_level: "info"

database:
  path: "~/.teammate/data/teammate.db"
  pool:
    min_connections: 1
    max_connections: 10

scan:
  languages:
    - rust
    - typescript
    - python
    - go
    - java

  patterns:
    - "// TODO"
    - "# TODO"
    - "/* TODO */"
    - "<!-- TODO -->"
    - "[ ] TODO"
    - "TODO:"
    - "FIXME:"

  exclude:
    - "**/node_modules/**"
    - "**/.git/**"
    - "**/target/**"
    - "**/build/**"

  max_file_size: 1048576

display:
  theme: "dark"
  date_format: "%Y-%m-%d"
  columns:
    - id
    - content
    - status
    - priority
    - tags
    - file
    - line

git:
  enabled: true
  blame:
    enabled: true
    cache_ttl: 3600
  hooks:
    pre_commit: true
    post_commit: true

sync:
  enabled: false
  provider: "local"

tags:
  defaults:
    - name: "bug"
      color: "#EF4444"
    - name: "feature"
      color: "#10B981"
    - name: "enhancement"
      color: "#3B82F6"
```

---

## 7. 性能优化要求

### 7.1 性能指标

| 操作 | 目标时间 | 测试场景 |
|------|---------|---------|
| 全量扫描 10K 文件 | < 5 秒 | SSD, 8 核 CPU |
| 单次查询 | < 100ms | 1K TODO |
| 增量扫描 | < 500ms | 10 个修改文件 |
| CLI 启动 | < 200ms | 冷启动 |
| TUI 渲染 | < 50ms | 60 FPS |

### 7.2 优化策略

1. **并行扫描**：使用 Rayon 并行处理文件
2. **增量更新**：只扫描变更的文件
3. **智能缓存**：L1 内存缓存 + L2 磁盘缓存
4. **数据库优化**：WAL 模式、适当索引
5. **懒加载**：按需加载数据

---

## 8. 实施路线图

### Phase 1: MVP (第 1-2 周)

| 任务 | 状态 | 负责人 |
|------|------|--------|
| 项目初始化 | 待开始 | - |
| CLI 框架实现 | 待开始 | - |
| 存储层实现 | 待开始 | - |
| 基础解析器 | 待开始 | - |
| scan/list/add 命令 | 待开始 | - |

### Phase 2: 完善 (第 3-4 周)

| 任务 | 状态 | 负责人 |
|------|------|--------|
| 所有解析器 | 待开始 | - |
| Git blame 集成 | 待开始 | - |
| 标签管理 | 待开始 | - |
| 统计命令 | 待开始 | - |

### Phase 3: TUI (第 5-6 周)

| 任务 | 状态 | 负责人 |
|------|------|--------|
| TUI 框架 | 待开始 | - |
| 列表组件 | 待开始 | - |
| 过滤器组件 | 待开始 | - |
| 详情面板 | 待开始 | - |

### Phase 4: 测试与优化 (第 7 周)

| 任务 | 状态 | 负责人 |
|------|------|--------|
| 单元测试 | 待开始 | - |
| 集成测试 | 待开始 | - |
| CI 配置 | 待开始 | - |
| 性能优化 | 待开始 | - |

---

## 9. 验收标准

### 功能验收

- [ ] 所有 CLI 命令正常工作
- [ ] 支持 30+ 种语言解析
- [ ] Git blame 集成正常
- [ ] TUI 界面流畅

### 性能验收

- [ ] 扫描 10K 文件 < 5 秒
- [ ] 单次查询 < 100ms
- [ ] CLI 启动 < 200ms

### 代码质量

- [ ] 测试覆盖率 > 80%
- [ ] Clippy 无警告
- [ ] 文档完整

---

**文档版本**: 1.0
**最后更新**: 2026-02-11
**状态**: 待实施
