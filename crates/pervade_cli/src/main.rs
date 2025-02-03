mod commands;
mod generator;
mod parser;

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
        Commands::Build { input, output } => {
            
        },
        Commands::Deploy => {
            println!("Deploy");
        }
    }
}
