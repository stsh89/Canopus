use clap::Subcommand;
use uuid::Uuid;

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
