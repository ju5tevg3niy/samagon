use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run,
    GameLoopTest,
    ReadProject { path: String },
    ReadAsset { path: String },
}
