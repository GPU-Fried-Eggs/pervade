mod commands;
mod generate;
mod runtime;

use clap::Parser;
use commands::Commands;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "pervade")]
#[command(about = "Show ", long_about = None)]
struct PervadeCli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let PervadeCli { command } = PervadeCli::parse();

    match command {
        Commands::Deploy => {
            println!("Deploy");
        },
    }
}
