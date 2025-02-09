use canopus_engine::{operations::create_remark, Engine};
use canopus_protocol::NewRemark;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "New-Remark")]
    NewRemark {
        #[arg(id = "Essence", long)]
        essence: String,

        #[arg(id = "Tag", long)]
        tags: Vec<String>,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let cli = Cli::parse();
    let engine = Engine::start().await?;

    match cli.command {
        Commands::NewRemark { essence, tags } => {
            let id = create_remark(&engine, NewRemark { essence, tags }).await?;

            println!("Created remark with id: {}", id);
        }
    };

    Ok(())
}
