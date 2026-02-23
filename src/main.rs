use crate::cli::args::Args;
use crate::cli::run;
use clap::Parser;

mod cli;
mod storage;
mod core;
mod parsers;
mod git;

fn main() {
    let args = Args::parse();
    
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
