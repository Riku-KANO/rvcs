use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Add { path: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Init => println!("init"),
        Command::Add { path } => println!("add {path}"),
    }
}
