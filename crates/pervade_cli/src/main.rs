mod commands;

use clap::Parser;
use commands::Commands;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
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
