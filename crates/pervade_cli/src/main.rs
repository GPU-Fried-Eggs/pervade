mod bundler;
mod command;
mod common;
mod compiler;
mod error;
mod generator;
mod resolver;
mod runtime;

use clap::Parser;
use command::Command;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "pervade")]
#[command(about = "Show ", long_about = None)]
struct PervadeCli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let PervadeCli { command } = PervadeCli::parse();

    match command {
        Command::Deploy => {
            println!("Deploy");
        }
    }
}
