use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a upstream/downstream link
    Add,
    /// List current links
    List,
    /// Remove a link
    Remove(CommandArg),
    /// Update downstream
    Update,
}

#[derive(Args)]
pub struct CommandArg {
    argument: String,
}

impl CommandArg {
    pub fn get(&self) -> &String {
        &self.argument
    }
}
