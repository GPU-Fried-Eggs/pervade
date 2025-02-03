use std::path::PathBuf;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Build {
        #[arg(value_name = "INPUT", required = true)]
        input: PathBuf,
        #[arg(short, default_value = "build")]
        output: PathBuf,
    },
    Deploy,
}
