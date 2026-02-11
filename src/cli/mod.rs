use crate::cli::args::Args;
use crate::cli::args::Commands;

pub mod args;
pub mod commands;
pub mod error;
pub mod output;

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Command routing and execution
    match &args.command {
        Some(Commands::Scan(scan_args)) => commands::scan::execute(scan_args),
        Some(Commands::List(list_args)) => commands::list::execute(list_args),
        Some(Commands::Add(add_args)) => commands::add::execute(add_args),
        Some(Commands::Remove(remove_args)) => commands::remove::execute(remove_args),
        Some(Commands::Update(update_args)) => commands::update::execute(update_args),
        Some(Commands::Status(status_args)) => commands::status::execute(status_args),
        Some(Commands::Tag(tag_args)) => commands::tag::execute(tag_args),
        Some(Commands::Config(config_args)) => commands::config::execute(config_args),
        Some(Commands::Init(init_args)) => commands::init::execute(init_args),
        Some(Commands::Stats(stats_args)) => commands::stats::execute(stats_args),
        Some(Commands::Blame(blame_args)) => commands::blame::execute(blame_args),
        Some(Commands::Branch(branch_args)) => commands::branch::execute(branch_args),
        Some(Commands::Sync(sync_args)) => commands::sync::execute(sync_args),
        Some(Commands::Hooks(hooks_args)) => commands::hooks::execute(hooks_args),
        None => {
            // No command provided, show help
            println!("Teammate - TODO CLI 工具\n");
            println!("使用: teammate <命令> [选项]");
            println!("\n可用命令:");
            println!("  scan      扫描代码库中的 TODO");
            println!("  list      列出 TODO");
            println!("  add       添加新 TODO");
            println!("  remove    删除 TODO");
            println!("  update    更新 TODO");
            println!("  status    更新 TODO 状态");
            println!("  tag       管理标签");
            println!("  config    管理配置");
            println!("  init      初始化项目");
            println!("  stats     显示统计信息");
            println!("  blame     查看 TODO 的 Git blame");
            println!("  branch    分支 TODO 管理");
            println!("  sync      同步数据");
            println!("  hooks     管理 Git hooks");
            println!("\n使用 teammate <命令> --help 查看命令详情");
            Ok(())
        }
    }
}
