use clap::{Parser, Subcommand};

mod commands;
mod index;
mod utils;

#[derive(Parser)]
#[command(name = "rvcs", version, about = "mini vcs")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize a new repository
    Init,
    /// Add file to staging (index) and store object
    Add { path: String },
    /// Create a new commit
    Commit {
        #[arg(short, long)]
        message: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => commands::init::run(),
        Command::Add { path } => commands::add::run(path),
        Command::Commit { message } => commands::commit::run(message),
    }
}
