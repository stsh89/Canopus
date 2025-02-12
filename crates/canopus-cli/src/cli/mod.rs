mod formatter;
mod remarks;
mod tags;

use canopus_engine::Engine;
use clap::{command, Parser, Subcommand};
use remarks::{DeleteRemarkArguments, GetRemarkArguments, NewRemarkArguments};
use tags::GetTagArguments;

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "New-Remark", alias = "new-remark")]
    NewRemark(NewRemarkArguments),

    #[command(name = "Get-Remark", alias = "get-remark")]
    GetRemark(GetRemarkArguments),

    #[command(name = "Get-Tag", alias = "get-tag")]
    GetTag(GetTagArguments),

    #[command(name = "Delete-Remark", alias = "delete-remark")]
    DeleteRemark(DeleteRemarkArguments),
}

impl Cli {
    pub async fn execute(self) -> anyhow::Result<()> {
        let engine = Engine::start().await?;

        match self.command {
            Commands::DeleteRemark(args) => remarks::delete_remark(&engine, args).await?,
            Commands::GetRemark(args) => remarks::get_remark(&engine, args).await?,
            Commands::GetTag(args) => tags::get_tag(&engine, args).await?,
            Commands::NewRemark(args) => remarks::new_remark(&engine, args).await?,
        };

        Ok(())
    }
}
