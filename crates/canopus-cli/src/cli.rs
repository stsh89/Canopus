use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ShowTag {
        id: Uuid,
    },
    ListTags {
        #[arg(short, long)]
        page_token: Option<String>,
    },
}

impl Cli {
    pub fn new() -> eyre::Result<Self> {
        let cli = Self::try_parse()?;

        Ok(cli)
    }

    pub fn new_with_args(args: &[&str]) -> eyre::Result<Self> {
        let cli = Self::try_parse_from(args)?;

        Ok(cli)
    }
}
