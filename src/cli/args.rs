use std::path::PathBuf;
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

    /// 启用 TUI 界面
    #[arg(long)]
    pub tui: bool,

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
    #[command(alias = "rm")]
    #[command(alias = "del")]
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

#[derive(Debug, Parser)]
pub struct ScanArgs {
    /// 扫描路径
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// 排除的路径
    #[arg(short, long)]
    pub exclude: Vec<PathBuf>,

    /// 只扫描特定语言
    #[arg(short, long)]
    pub language: Option<String>,

    /// 包含 TODO, FIXME, HACK 等
    #[arg(long)]
    pub include_all: bool,

    /// 递归深度
    #[arg(short, long)]
    pub max_depth: Option<usize>,

    /// 输出到文件
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    
    /// 不递归扫描
    #[arg(long)]
    pub no_recursive: bool,
    
    /// 仅显示统计信息
    #[arg(long)]
    pub stats_only: bool,
}

#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 过滤：仅显示未解决的 TODO
    #[arg(long)]
    pub open: bool,

    /// 过滤：按标签过滤
    #[arg(short, long)]
    pub tag: Vec<String>,

    /// 过滤：按优先级过滤
    #[arg(long, value_enum)]
    pub priority: Option<Priority>,

    /// 过滤：按作者过滤
    #[arg(long)]
    pub author: Option<String>,

    /// 过滤：按文件路径过滤
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// 排序字段
    #[arg(long, value_enum)]
    pub sort: Option<SortField>,

    /// 升序排列
    #[arg(long)]
    pub ascending: bool,

    /// 限制显示数量
    #[arg(short, long)]
    pub limit: Option<usize>,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum SortField {
    Priority,
    Created,
    Updated,
    File,
}

#[derive(Debug, Parser)]
pub struct AddArgs {
    /// TODO 内容
    pub content: String,

    /// 关联文件
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// 关联行号
    #[arg(short, long)]
    pub line: Option<usize>,

    /// 标签
    #[arg(short, long)]
    pub tag: Vec<String>,

    /// 优先级
    #[arg(long, value_enum)]
    pub priority: Option<Priority>,

    /// 作者
    #[arg(long)]
    pub author: Option<String>,

    /// 关联 Issue
    #[arg(short, long)]
    pub issue: Option<String>,
}

#[derive(Debug, Parser)]
pub struct RemoveArgs {
    /// TODO ID
    pub id: String,

    /// 强制删除（不询问确认）
    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Parser)]
pub struct UpdateArgs {
    /// TODO ID
    pub id: String,

    /// 新内容
    #[arg(short, long)]
    pub content: Option<String>,

    /// 新优先级
    #[arg(long, value_enum)]
    pub priority: Option<Priority>,

    /// 新作者
    #[arg(long)]
    pub author: Option<String>,
}

#[derive(Debug, Parser)]
pub struct StatusArgs {
    /// TODO ID
    pub id: String,

    /// 新状态
    #[arg(value_enum)]
    pub status: TodoStatus,

    /// 添加评论
    #[arg(short, long)]
    pub comment: Option<String>,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum TodoStatus {
    Open,
    InProgress,
    Resolved,
}

#[derive(Debug, Parser)]
pub struct TagArgs {
    /// TODO ID
    pub id: Option<String>,

    /// 添加标签
    #[arg(short, long)]
    pub add: Vec<String>,

    /// 移除标签
    #[arg(short, long)]
    pub remove: Vec<String>,

    /// 列出所有标签
    #[arg(long)]
    pub list: bool,
}

#[derive(Debug, Parser)]
pub struct ConfigArgs {
    /// 查看当前配置
    #[arg(long)]
    pub show: bool,

    /// 重置为默认配置
    #[arg(long)]
    pub reset: bool,

    /// 设置配置项
    #[arg(long)]
    pub set: Option<String>,
}

#[derive(Debug, Parser)]
pub struct InitArgs {
    /// 项目路径
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// 强制重新初始化
    #[arg(long)]
    pub force: bool,

    /// 不初始化 Git 仓库
    #[arg(long)]
    pub no_git: bool,
}

#[derive(Debug, Parser)]
pub struct StatsArgs {
    /// 按标签分组统计
    #[arg(long)]
    pub by_tag: bool,

    /// 按优先级分组统计
    #[arg(long)]
    pub by_priority: bool,

    /// 按文件分组统计
    #[arg(long)]
    pub by_file: bool,

    /// 生成图表
    #[arg(long)]
    pub chart: bool,
}

#[derive(Debug, Parser)]
pub struct BlameArgs {
    /// TODO ID
    pub id: String,

    /// 显示详细 blame 信息
    #[arg(long)]
    pub verbose: bool,
}

#[derive(Debug, Parser)]
pub struct BranchArgs {
    /// 切换到指定分支
    #[arg(long)]
    pub switch: Option<String>,

    /// 列出分支 TODO
    #[arg(long)]
    pub list: bool,

    /// 比较两个分支的 TODO
    #[arg(long)]
    pub compare: Option<String>,
}

#[derive(Debug, Parser)]
pub struct SyncArgs {
    /// 同步来源
    #[arg(long)]
    pub from: Option<String>,

    /// 同步目标
    #[arg(long)]
    pub to: Option<String>,

    /// 解决冲突
    #[arg(long)]
    pub resolve: Option<String>,

    /// 强制覆盖
    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Parser)]
pub struct HooksArgs {
    /// 安装 Git hooks
    #[arg(long)]
    pub install: bool,

    /// 卸载 Git hooks
    #[arg(long)]
    pub uninstall: bool,

    /// 列出已安装的 hooks
    #[arg(long)]
    pub list: bool,

    /// 配置 hook
    #[arg(long)]
    pub config: Option<String>,
}
