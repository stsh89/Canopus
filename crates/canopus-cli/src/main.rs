mod remarks;

use canopus_engine::Engine;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "New-Remark", alias = "new-remark")]
    NewRemark(NewRemarkArguments),
}

#[derive(Parser)]
struct NewRemarkArguments {
    #[arg(id = "Essence", long, alias = "essence")]
    essence: String,

    #[arg(id = "Tag", long, alias = "tag")]
    tags: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let engine = Engine::start().await?;

    match cli.command {
        Commands::NewRemark(arguments) => remarks::new(&engine, arguments).await?,
    };

    Ok(())
}
