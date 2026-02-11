use crate::cli::args::Args;
use crate::cli::commands::Commands;

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
            // No command provided, show help by default
            args.command.print_help()?;
            Ok(())
        }
    }
}
