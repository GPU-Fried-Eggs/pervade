use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    Deploy,
}
