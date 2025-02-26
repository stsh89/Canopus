mod remarks;
mod tags;

use crate::session::Session;
use canopus_engine::Engine;
use clap::{Parser, Subcommand, command};
use remarks::{
    DeleteRemarkArguments, GetRemarkArguments, ListRemarksArguments, NewRemarkArguments,
    UpdateRemarkArguments,
};
use tags::{GetTagArguments, ListTagsArguments};

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "Delete-Remark", alias = "delete-remark")]
    DeleteRemark(DeleteRemarkArguments),

    #[command(name = "Get-Remark", alias = "get-remark")]
    GetRemark(GetRemarkArguments),

    #[command(name = "Get-Tag", alias = "get-tag")]
    GetTag(GetTagArguments),

    #[command(name = "List-Remarks", alias = "list-remarks")]
    ListRemarks(ListRemarksArguments),

    #[command(name = "List-Tags", alias = "list-tags")]
    ListTags(ListTagsArguments),

    #[command(name = "New-Remark", alias = "new-remark")]
    NewRemark(NewRemarkArguments),

    #[command(name = "Reset-Session", alias = "reset-session")]
    ResetSession,

    #[command(name = "Update-Remark", alias = "update-remark")]
    UpdateRemark(UpdateRemarkArguments),
}

impl Cli {
    pub async fn execute(self) -> anyhow::Result<()> {
        let engine = Engine::start().await?;
        let mut session = Session::start()?;

        match self.command {
            Commands::DeleteRemark(args) => remarks::delete_remark(&engine, args).await?,
            Commands::GetRemark(args) => remarks::get_remark(&engine, args).await?,
            Commands::GetTag(args) => tags::get_tag(&engine, args).await?,
            Commands::NewRemark(args) => remarks::new_remark(&engine, args).await?,
            Commands::ListRemarks(args) => {
                remarks::list_remarks(&engine, &mut session, args).await?
            }
            Commands::ListTags(args) => tags::list_tags(&engine, &mut session, args).await?,
            Commands::ResetSession => session = session.reset()?,
            Commands::UpdateRemark(args) => remarks::update_remark(&engine, args).await?,
        };

        if session.is_changed() {
            session.save()?;
        }

        Ok(())
    }
}
